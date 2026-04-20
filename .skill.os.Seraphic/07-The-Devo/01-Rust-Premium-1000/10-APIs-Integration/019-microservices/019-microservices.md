# SKILL 019: MICROSERVICES & DISTRIBUTED SYSTEMS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        MICROSERVICES ARCHITECTURE
                     Service Communication & Orchestration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Building microservices in Rust with service discovery, load balancing,
circuit breakers, and distributed tracing.

---

## SERVICE DISCOVERY

### 1.1 Consul Integration

```rust
use reqwest::Client;

pub struct ServiceDiscovery {
    client: Client,
    consul_url: String,
}

impl ServiceDiscovery {
    pub fn new(consul_url: &str) -> Self {
        ServiceDiscovery {
            client: Client::new(),
            consul_url: consul_url.to_string(),
        }
    }

    pub async fn get_service(&self, name: &str) -> Result<Vec<ServiceEndpoint>, Error> {
        let url = format!("{}/v1/health/service/{}", self.consul_url, name);
        let response = self.client.get(&url).send().await?;
        
        let services: Vec<ConsulService> = response.json().await?;
        
        Ok(services
            .into_iter()
            .filter(|s| s.passes())
            .map(|s| ServiceEndpoint {
                address: s.Service.Address,
                port: s.Service.Port,
            })
            .collect())
    }
}
```

---

## CIRCUIT BREAKER

### 2.1 Circuit Breaker

```rust
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    state: CircuitState,
    failures: u32,
    successes: u32,
}

#[derive(Clone, Copy)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub async fn call<F, R>(&mut self, service_name: &str, f: F) -> Result<R, CircuitError>
    where
        F: Future<Output = Result<R, Error>>,
    {
        match self.state {
            CircuitState::Open => {
                if self.should_attempt() {
                    self.state = CircuitState::HalfOpen;
                    return self.call_service(f).await;
                }
                Err(CircuitError::Open)
            }
            _ => self.call_service(f).await,
        }
    }

    async fn call_service<F, R>(&mut self, f: F) -> Result<R, Error>
    where
        F: Future<Output = Result<R, Error>>,
    {
        match f.await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(e)
            }
        }
    }

    fn on_success(&mut self) {
        self.failures = 0;
        if self.successes >= self.success_threshold {
            self.state = CircuitState::Closed;
        }
    }

    fn on_failure(&mut self) {
        self.failures += 1;
        if self.failures >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}
```

---

## LOAD BALANCING

### 3.1 Round Robin

```rust
pub struct RoundRobin<T> {
    items: Vec<T>,
    current: Index<usize>,
}

impl<T> RoundRobin<T> {
    pub fn new(items: Vec<T>) -> Self {
        RoundRobin {
            items,
            current: Index(0),
        }
    }

    pub fn next(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }

        let item = &self.items[self.current.0];
        self.current.0 = (self.current.0 + 1) % self.items.len();
        Some(item)
    }
}
```

---

## DISTRIBUTED TRACING

### 4.1 OpenTelemetry

```rust
use opentelemetry::{trace::{Tracer, SpanKind}, Context, KeyValue};
use opentelemetry_sdk::trace::Config;

pub fn init_tracer(service_name: &str) -> Tracer {
    opentelemetry::tracer()
        .with_config(
            Config::default()
                .with_service_name(service_name)
                .with_span_limits(100, 10),
        )
}

pub fn inject_span_context<T>(injector: &mut T)
where
    T: Injector,
{
    let context = Context::current();
    if let Some(span) = context.span() {
        let span_context = span.get_span_context();
        injector.set("trace_id", span_context.trace_id().to_string());
        injector.set("span_id", span_context.span_id().to_string());
    }
}
```

---

## RECAP

1. **Service discovery** - Find healthy instances
2. **Circuit breaker** - Fail fast
3. **Load balancing** - Distribute requests
4. **Tracing** - Debug distributed systems

---

*Skill ID: 019 | Category: Microservices | Complexity: Expert*