# SKILL 017: NETWORKING & PROTOCOLS IN RUST

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        NETWORKING & PROTOCOLS IN RUST
                     Advanced Network Programming
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive networking in Rust including TCP/UDP, HTTP/2, gRPC,
WebSockets, and custom protocol implementation.

## TABLE OF CONTENTS

1. [TCP/UDP](#tcpudp)
2. [HTTP/2](#http2)
3. [gRPC](#grpc)
4. [Custom Protocols](#custom-protocols)

---

## TCP/UDP

### 1.1 Async Network

```rust
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_tcp_server(addr: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted from: {}", addr);
        
        tokio::spawn(async move {
            handle_tcp(socket).await;
        });
    }
}

async fn handle_tcp(mut socket: TcpStream) {
    let mut buffer = [0u8; 4096];
    
    loop {
        let n = socket.read(&mut buffer).await?;
        if n == 0 { break; }
        
        socket.write_all(&buffer[..n]).await?;
    }
}
```

---

## HTTP/2

### 2.1 HTTP/2 Server

```rust
use http::{Request, Response, StatusCode};
use hyper::server::conn::http2;

pub async fn serve_h2(addr: &str) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;
    
    loop {
        let (stream, _) = listener.accept().await?;
        let service = MyService;
        
        tokio::spawn(async move {
            if let Ok(conn) = http2::Builder::new()
                .serve_connection(stream, service)
                .await
            {
                let _ = conn;
            }
        });
    }
}
```

---

## gRPC

### 3.1 gRPC Server

```rust
pub struct GrpcServer {
    services: HashMap<String, Box<dyn GrpcService>>,
}

impl GrpcServer {
    pub fn new() -> Self {
        GrpcServer {
            services: HashMap::new(),
        }
    }

    pub fn register<S: GrpcService + 'static>(&mut self, service: S) {
        self.services.insert(S::NAME.to_string(), Box::new(service));
    }

    pub async fn handle_request(&self, request: &[u8]) -> Result<Vec<u8>, GrpcError> {
        let service_name = extract_service(request);
        let method = extract_method(request);
        
        let service = self.services.get(&service_name)
            .ok_or(GrpcError::NotFound)?;
            
        service.call(method, request)
    }
}
```

---

## RECAP

1. **tokio for async** - Best networking performance
2. **HTTP/2 for speed** - Multiplexing, header compression
3. **gRPC for services** - Type-safe, efficient
4. **自定义 protocols** - When you need control

---

*Skill ID: 017 | Category: Networking | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*