# SKILL 005: RUST WEB FRAMEWORKS - AXUM, ACTIX, ROCKET MASTERY

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        RUST WEB FRAMEWORKS MASTERY
                     Building Elite Production Web Services
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of building high-performance web services in Rust using axum, actix-web, and rocket.
Covers REST APIs, GraphQL, WebSockets, middleware, authentication, rate limiting, and deployment.

## TABLE OF CONTENTS

1. [Axum Fundamentals](#axum-fundamentals)
2. [Axum Advanced](#axum-advanced)
3. [Actix-Web Deep Dive](#actix-web-deep-dive)
4. [Rocket Framework](#rocket-framework)
5. [Authentication & Authorization](#authentication--authorization)
6. [WebSockets](#websockets)
7. [GraphQL](#graphql)
8. [Rate Limiting](#rate-limiting)
9. [Security](#security)
10. [Deployment](#deployment)

---

## AXUM FUNDAMENTALS

### 1.1 Basic Axum Server

```rust
use axum::{
    routing::{get, post},
    Router,
    response::Json,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn get_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
        User { id: 2, name: "Bob".into(), email: "bob@example.com".into() },
    ])
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (axum::http::StatusCode, Json<User>) {
    let user = User {
        id: 3,
        name: payload.name,
        email: payload.email,
    };
    (axum::http::StatusCode::CREATED, Json(user))
}

pub fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .layer(ServiceBuilder::new().into_inner())
}

#[tokio::main]
pub async fn main() {
    let app = create_app();
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
```

### 1.2 State Management

```rust
use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// App state shared across requests
pub struct AppState {
    pub users: RwLock<HashMap<u64, User>>,
    pub db: DbPool,
}

pub type SharedState = Arc<AppState>;

pub fn create_state() -> SharedState {
    let mut users = HashMap::new();
    users.insert(1, User { id: 1, name: "Alice".into(), email: "alice@example.com".into() });
    users.insert(2, User { id: 2, name: "Bob".into(), email: "bob@example.com".into() });
    
    Arc::new(AppState {
        users: RwLock::new(users),
        db: DbPool::new(),
    })
}

pub async fn get_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().await;
    users
        .get(&user_id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn update_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().await;
    
    if let Some(user) = users.get_mut(&user_id) {
        if let Some(name) = payload.name {
            user.name = name;
        }
        if let Some(email) = payload.email {
            user.email = email;
        }
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_user(
    State(state): State<SharedState>,
    Path(user_id): Path<u64>,
) -> StatusCode {
    let mut users = state.users.write().await;
    if users.remove(&user_id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
```

---

## AXUM ADVANCED

### 2.1 Middleware

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    middleware::Next,
};

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    println!(
        "{} {} {} {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    response
}

pub async fn require_header(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if request.headers().contains_key("x-api-key") {
        next.run(request).await
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
```

### 2.2 Dependency Injection

```rust
use axum::{
    extract::FromRef,
    http::StatusCode,
};

pub struct DbPool { /* ... */ }

pub struct AuthService { /* ... */ }

pub struct EmailService { /* ... */ }

/// Extract multiple services
pub async fn handler(
    State(db): State<DbPool>,
    State(auth): State<AuthService>,
    State(email): State<EmailService>,
) -> Result<Json<Response>, StatusCode> {
    // Use all services
    todo!()
}

/// Custom extractor
pub struct AuthUser(pub UserId);

#[async_trait]
impl<S> FromRef<S> for AuthUser
where
    S: Send + Sync,
{
    async fn from_ref(state: &S) -> Self {
        // Extract from request context
        todo!()
    }
}
```

---

## ACTIX-WEB DEEP DIVE

### 3.1 Actix-Web Server

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

pub async fn health() -> impl Responder {
    web::Json(json!({
        "status": "healthy",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```

### 3.2 Actix Patterns

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web::error::Error;

/// Resource handlers
pub async fn get_items(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| Error::from(e))?;
    
    Ok(HttpResponse::Ok().json(items))
}

/// Path and query parameters
pub async fn get_item(
    web::Path(id): web::Path<i64>,
    web::Query(params): web::Query<Params>,
) -> Result<HttpResponse> {
    println!("Item {} with params: {:?}", id, params);
    Ok(HttpResponse::Ok().json(json!({ "id": id })))
}
```

---

## ROCKET FRAMEWORK

### 4.1 Rocket Basics

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

rocket::build()
    .mount("/", routes![index, hello])
    .launch();
```

### 4.2 Rocket Advanced

```rust
use rocket::{post, State, Guard};
use rocket::fairing::{Fairing, Info, Kind};

pub struct AppState {
    db: DbPool,
}

#[derive(FromForm)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login", data = "<form>")]
fn login(
    form: Form<LoginForm>,
    state: State<AppState>,
) -> Result<Json<Token>, ApiError> {
    // Validate credentials
    todo!()
}
```

---

## AUTHENTICATION & AUTHORIZATION

### 5.1 JWT Authentication

```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
}

pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new(secret: impl Into<String>) -> Self {
        JwtService { secret: secret.into() }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn decode(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|t| t.claims)
    }
}
```

### 5.2 OAuth2

```rust
pub struct OAuth2 {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl OAuth2 {
    pub fn authorize_url(&self, state: &str) -> String {
        format!(
            "https://oauthprovider.com/authorize?\
             client_id={}&\
             redirect_uri={}&\
             response_type=code&\
             scope=email&\
             state={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            state
        )
    }

    pub async fn exchange_code(
        &self,
        code: &str,
    ) -> Result<TokenResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        
        client
            .post("https://oauthprovider.com/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", code),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await?
            .json::<TokenResponse>()
            .await
    }
}
```

---

## WEBSOCKETS

### 6.1 Axum WebSocket

```rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(handle_socket)
}

pub async fn handle_socket(
    socket: WebSocket,
) {
    let (sender, mut receiver) = socket.split();
    
    tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        let _ = sender.send(Message::text(format!("Echo: {}", text))).await;
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
        }
    });
}
```

### 6.2 WebSocket Broadcasting

```rust
use tokio::sync::broadcast;

pub struct WebSocketManager {
    sender: broadcast::Sender<WsMessage>,
}

struct WsMessage {
    room: String,
    content: String,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        WebSocketManager { sender }
    }

    pub fn subscribe(&self, room: String) -> broadcast::Receiver<WsMessage> {
        self.sender.subscribe()
    }

    pub fn broadcast(&self, room: &str, content: String) {
        let _ = self.sender.send(WsMessage {
            room: room.to_string(),
            content,
        });
    }
}
```

---

## GRAPHQL

### 7.1 Async-GraphQL with Axum

```rust
use async_graphql::{Object, Schema, SimpleObject, InputObject};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Option<User> {
        ctx.data::<DbPool>()?.get_user(&id).await
    }

    async fn users(&self, ctx: &Context<'_>) -> Vec<User> {
        ctx.data::<DbPool>()?.get_users().await
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> User {
        ctx.data::<DbPool>()?.create_user(input).await
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    Json(req): Json<Request>,
) -> Json<Response> {
    Json(schema.execute(req).await)
}
```

---

## RATE LIMITING

### 8.1 Token Bucket

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    bucket_size: usize,
    refill_rate: Duration,
    tokens: AtomicUsize,
    last_refill: parking_lot::Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(bucket_size: usize, refill_rate: Duration) -> Self {
        RateLimiter {
            bucket_size,
            refill_rate,
            tokens: AtomicUsize::new(bucket_size),
            last_refill: parking_lot::Mutex::new(Instant::now()),
        }
    }

    pub fn try_acquire(&self) -> bool {
        let mut last = self.last_refill.lock();
        let now = Instant::now();
        
        if now.duration_since(*last) >= self.refill_rate {
            self.tokens.store(self.bucket_size, Ordering::Release);
            *last = now;
        }
        
        self.tokens.fetch_sub(1, Ordering::AcqRel) > 0
    }
}
```

---

## SECURITY

### 9.1 Security Headers

```rust
use axum::middleware::FutureOr;

pub async fn security_headers<
    B: Send + std::ops::Try<Output = ()>,
>(
    mut request: Request,
    next: Next<B>,
) -> impl IntoResponse {
    let response = next.run(request).await;
    
    tokio::spawn(async move {
        let response = response.into_response();
        
        let mut response = response;
        
        response.headers_mut().insert(
            "Strict-Transport-Security",
            "max-age=31536000; includeSubDomains"
                .parse()
                .unwrap(),
        );
        
        response.headers_mut().insert(
            "X-Content-Type-Options",
            "nosniff".parse().unwrap(),
        );
        
        response.headers_mut().insert(
            "X-Frame-Options",
            "DENY".parse().unwrap(),
        );
        
        response
    })
}
```

---

## DEPLOYMENT

### 10.1 Docker Configuration

```dockerfile
# Build stage
FROM rust:1.75 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp

EXPOSE 3000

CMD ["myapp"]
```

---

## RECAP

1. **Choose axum for async-first** - Best integration with tokio
2. **Actix for raw performance** - When every microsecond counts
3. **Rocket for ergonomics** - Clean API for simple services
4. **Always add security headers** - HTTPS, HSTS, etc.
5. **Implement rate limiting** - Protect your services
6. **Use WebSockets for real-time** - Bidirectional communication

---

*Skill ID: 005 | Category: Web-Development | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*