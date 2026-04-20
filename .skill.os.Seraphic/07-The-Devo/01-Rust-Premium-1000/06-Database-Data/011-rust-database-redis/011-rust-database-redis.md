# SKILL 011-C: REDIS & CACHE

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        REDIS & CACHE SYSTEMS
                     In-Memory Data Stores
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Redis implementation in Rust for caching, session storage, pub/sub, and rate limiting.

---

## REDIS CLIENT

```rust
use redis::{Client, Commands, RedisResult};

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new(url: &str) -> RedisResult<Self> {
        Ok(RedisCache {
            client: Client::open(url)?,
        })
    }

    pub fn get(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.client.get_connection()?;
        conn.get(key)
    }

    pub fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.set(key, value)
    }

    pub fn set_ex(&self, key: &str, value: &str, ttl: u64) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.set_ex(key, value, ttl)
    }

    pub fn del(&self, key: &str) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.del(key)
    }
}
```

---

## RATE LIMITING

```rust
pub struct RateLimiter {
    cache: RedisCache,
    key_prefix: String,
}

impl RateLimiter {
    pub fn new(cache: RedisCache) -> Self {
        RateLimiter {
            cache,
            key_prefix: "rate_limit".to_string(),
        }
    }

    pub fn check(&self, identifier: &str, limit: u64, window: u64) -> RedisResult<bool> {
        let key = format!("{}:{}", self.key_prefix, identifier);
        let count: u64 = self.cache.incr(&key)?;
        
        if count == 1 {
            self.cache.expire(&key, window)?;
        }
        
        Ok(count <= limit)
    }
}
```

---

## PUB/SUB

```rust
pub struct PubSub {
    client: Client,
}

impl PubSub {
    pub fn listen(&self, channel: &str) -> RedisResult<Receiver> {
        let mut conn = self.client.get_connection()?;
        conn.subscribe(channel)
    }

    pub fn publish(&self, channel: &str, message: &str) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        conn.publish(channel, message)
    }
}
```

---

*Skill ID: 011-C | Category: Database | Complexity: Expert*