# SKILL 004-B: ADVANCED ASYNC & TOKIO

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        ADVANCED ASYNC & TOKIO
                     Production Async Patterns
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Advanced async Rust patterns including streams, cancellation, backpressure, and debugging.

---

## STREAMS

### 1.1 Async Streams

```rust
use tokio_stream::{Stream, StreamExt};
use futures::stream::Stream;

pub struct EventStream {
    events: tokio::sync::mpsc::Receiver<Event>,
}

impl Stream for EventStream {
    type Item = Event;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.events.poll_recv(cx)
    }
}

pub async fn process_events(stream: impl Stream<Item = Event>) {
    tokio::pin!(stream);
    
    while let Some(event) = stream.next().await {
        handle_event(event).await;
    }
}
```

---

## CANCELLATION

### 2.1 Graceful Shutdown

```rust
use tokio::signal;
use tokio::sync::broadcast;

pub async fn run_with_shutdown() {
    let (shutdown_tx, _) = broadcast::channel(1);
    
    let server = async {
        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    println!("Shutting down...");
                    break;
                }
                conn = accept_connection() => {
                    handle_connection(conn).await;
                }
            }
        }
    };
    
    let metrics = async {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            print_metrics();
        }
    };
    
    tokio::join!(server, metrics);
}
```

---

## BACKPRESSURE

### 3.1 Rate Limiter

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(max_concurrent: usize) -> Self {
        RateLimiter {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn acquire(&self) -> Permit<'_> {
        let permit = self.semaphore.acquire().await.unwrap();
        Permit { permit }
    }
}

pub struct Permit<'a> {
    permit: tokio::sync::SemaphorePermit<'a>,
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        // Semaphore permit automatically released
    }
}
```

---

## DEBUGGING

### 4.1 Async Debug

```rust
#[macro_export]
macro_rules! trace_async {
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*)
    };
}

pub async fn debug_await<T>(future: impl Future<Output = T>, label: &str) -> T {
    tracing::debug!("Starting: {}", label);
    let result = future.await;
    tracing::debug!("Completed: {}", label);
    result
}
```

---

## RECAP

1. **Streams for data** - Async iteration
2. **Cancellation** - Graceful shutdown
3. **Backpressure** - Rate limiting
4. **Tracing** - Debug async code

---

*Skill ID: 004-B | Category: Async | Complexity: Expert*