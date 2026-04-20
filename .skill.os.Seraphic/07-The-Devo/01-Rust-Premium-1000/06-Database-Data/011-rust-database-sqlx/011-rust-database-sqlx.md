# SKILL 011-B: SQLX DATABASE

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        DATABASE WITH SQLX
                     Type-Safe Database Access
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive SQLx database programming in Rust with compile-time SQL verification.

---

## SQLX BASICS

### 1.1 Connection Pool

```rust
use sqlx::{postgres::PgPool, mysql::MySqlPool, sqlite::SqlitePool};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::new(database_url).await
}

pub async fn with_pool<F, R>(pool: &PgPool, f: F) -> Result<R, sqlx::Error>
where
    F: FnOnce(&PgPool) -> std::pin::Pin<Box<dyn Future<Output = Result<R, sqlx::Error>>>>,
{
    f(pool).await
}
```

### 1.2 Query Execution

```rust
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(pool: &PgPool, email: &str, name: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, name) VALUES (?, ?) RETURNING *"
    )
    .bind(email)
    .bind(name)
    .fetch_one(pool)
    .await
}
```

### 1.3 Transactions

```rust
pub async fn transfer_funds(
    pool: &PgPool,
    from_id: i64,
    to_id: i64,
    amount: f64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        .bind(amount)
        .bind(from_id)
        .execute(&mut tx)
        .await?;

    sqlx::query("UPDATE accounts SET balance = balance + ? WHERE id = ?")
        .bind(amount)
        .bind(to_id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;
    Ok(())
}
```

---

## MIGRATIONS

### 2.1 Migration Runner

```rust
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
}
```

---

## RECAP

1. **SQLx for type safety** - Compile-time query checking
2. **Transactions** - ACID compliance
3. **Migrations** -Schema version control

---

*Skill ID: 011-B | Category: Database | Complexity: Expert*