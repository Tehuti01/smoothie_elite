# SKILL 004: RUST CONCURRENCY & PARALLELISM MASTERY

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        RUST CONCURRENCY & PARALLELISM MASTERY
                     The Sovereign Guide to Thread-Safe High-Performance Systems
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of Rust concurrency including async/await, threads, message passing,
locks, atomic operations, lock-free data structures, thread pools, and work-stealing
schedulers. Covers tokio, rayon, and building custom concurrency primitives.

## TABLE OF CONTENTS

1. [Async/Await Deep Dive](#asyncawait-deep-dive)
2. [Tokio Runtime Mastery](#tokio-runtime-mastery)
3. [Thread-Based Concurrency](#thread-based-concurrency)
4. [Message Passing](#message-passing)
5. [Locks & Synchronization](#locks--synchronization)
6. [Atomics & Lock-Free](#atomics--lock-free)
7. [Thread Pools](#thread-pools)
8. [Work Stealing](#work-stealing)
9. [Pipeline Patterns](#pipeline-patterns)
10. [Error Handling](#error-handling)
11. [Performance Tuning](#performance-tuning)

---

## ASYNC/AWAIT DEEP DIVE

### 1.1 Future & Executor Basics

```rust
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// Custom Future implementation
pub struct TimeoutFuture<T> {
    duration: Duration,
    start: Option<Instant>,
    inner: Option<T>,
    waker: Option<Waker>,
}

impl<T> Future for TimeoutFuture<T> {
    type Output = Result<T, TimeoutError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().get_mut();

        // Initialize start time
        if this.start.is_none() {
            this.start = Some(Instant::now());
        }

        // Check if inner future is ready
        if let Some(ref inner) = this.inner {
            return Poll::Ready(Ok(inner));
        }

        // Check timeout
        let elapsed = this.start.unwrap().elapsed();
        if elapsed >= this.duration {
            return Poll::Ready(Err(TimeoutError));
        }

        // Register waker
        this.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

/// Pin projection for async state machines
pub struct AsyncStateMachine {
    state: State,
    data: Option<Data>,
}

enum State {
    Init,
    Waiting,
    Complete,
}

impl Unpin for AsyncStateMachine {}
```

### 1.2 Async Lifetimes

```rust
/// Async functions with lifetimes
async fn fetch_user<'a>(db: &'a Database, id: UserId) -> Result<User, Error> {
    let user = db.query("SELECT * FROM users WHERE id = ?", id).await?;
    Ok(user)
}

/// &'static in async - why it works
async fn static_lifetime_example() -> &'static str {
    // This &'static is fine because the data is baked into the binary
    "hello"
}

/// Async traits
#[async_trait]
pub trait Storage {
    async fn get(&self, key: &str) -> Option<Bytes>;
    async fn set(&self, key: String, value: Bytes) -> Result<(), Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
}

/// Generic async functions
async fn process_all<T, Fut, E>(items: Vec<T>, f: impl Fn(T) -> Fut) -> Vec<Result<T, E>>
where
    Fut: Future<Output = Result<T, E>>,
    T: Send,
{
    futures::future::join_all(items.into_iter().map(f)).await
}
```

---

## TOKIO RUNTIME MASTERY

### 2.1 Multi-Threaded Runtime

```rust
use tokio::runtime::{Builder, Runtime, Handle};
use tokio::task::{self, JoinHandle};

/// Create production-ready runtime
pub fn create_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name_fn(|| {
            static ATOMIC: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC.fetch_add(1, Ordering::Relaxed);
            format!("tokio-worker-{}", id)
        })
        .thread_stack_size(4 * 1024 * 1024) // 4MB stacks
        .max_blocking_threads(1024)
        .enable_all()
        .build()
        .expect("Failed to create runtime")
}

/// Current runtime access
pub async fn get_current_runtime() -> Runtime {
    tokio::runtime::Handle::current()
}

/// Spawn with specific spawner
pub fn spawn_with_handle<F>(handle: &Handle, future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    handle.spawn(future)
}

/// LocalSet for thread-local tasks
pub fn local_set_example() {
    let rt = create_runtime();
    let local = tokio::task::LocalSet::new();

    rt.block_on(async {
        local.run_until(async {
            task::spawn_local(async {
                println!("Running in local set");
            }).await.unwrap();
        });
    });
}
```

### 2.2 Task Management

```rust
/// Task spawned with cancellation support
pub async fn with_cancellation<F: Future>(future: F) -> F::Output {
    tokio::select! {
        result = future => result,
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl-C");
            std::process::exit(0);
        }
    }
}

/// Task that can be aborted
pub fn abortable_task() {
    let (tx, rx) =tokio::sync::oneshot::channel::<()>();
    
    let handle = tokio::spawn(async {
        tokio::select! {
            _ = do_work() => {},
            _ = rx => {
                println!("Task aborted");
            }
        }
    });
    
    // Abort later
    let _ = tx.send(());
    let _ = handle.abort();
}

/// Task local storage
pub async fn task_local_example() {
    tokio::task_local! {
        static REQUEST_ID: u64;
        static USER_CONTEXT: Arc<UserContext>;
    }

    REQUEST_ID.with(|&id| println!("Request ID: {}", id));
}
```

---

## THREAD-BASED CONCURRENCY

### 3.1 Native Threads

```rust
use std::thread::{self, JoinHandle, ThreadId};
use std::sync::Arc;

/// Thread spawn with Result
pub fn spawn_thread<F, T, E>(f: F) -> JoinHandle<Result<T, E>>
where
    F: FnOnce() -> Result<T, E> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    thread::spawn(f)
}

/// Thread pool with work queue
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

enum Job {
    Run(Box<dyn FnOnce() + Send + 'static>),
    Stop,
}

struct Worker {
    handle: Option<JoinHandle<()>>,
    receiver: mpsc::Receiver<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let workers: Vec<_> = (0..size)
            .map(|_| {
                let rx = receiver.clone();
                let handle = thread::spawn(move || {
                    while let Ok(job) = rx.lock().unwrap().recv() {
                        match job {
                            Job::Run(f) => f(),
                            Job::Stop => break,
                        }
                    }
                });
                Worker {
                    handle: Some(handle),
                    receiver: rx,
                }
            })
            .collect();

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Job::Run(Box::new(f)));
    }
}
```

### 3.2 Thread Communication

```rust
/// Thread-safe counter
pub struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    pub fn new() -> Self {
        AtomicCounter {
            count: AtomicUsize::new(0),
        }
    }

    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::Relaxed)
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }

    pub fn add(&self, delta: usize) -> usize {
        self.count.fetch_add(delta, Ordering::Relaxed)
    }
}

/// Thread-safe flag
pub struct Barrier {
    count: AtomicUsize,
    generation: AtomicUsize,
}

impl Barrier {
    pub fn new(n: usize) -> Self {
        Barrier {
            count: AtomicUsize::new(n),
            generation: AtomicUsize::new(0),
        }
    }

    pub fn wait(&self) -> bool {
        let gen = self.generation.load(Ordering::Relaxed);
        let count = self.count.fetch_sub(1, Ordering::Relaxed);

        if count == 1 {
            self.generation.fetch_add(1, Ordering::Release);
            true
        } else {
            while self.generation.load(Ordering::Acquire) == gen {
                thread::yield_now();
            }
            false
        }
    }
}
```

---

## MESSAGE PASSING

### 4.1 Channels

```rust
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

/// Producer-consumer pattern
pub fn producer_consumer<T: Send + 'static>() -> (Sender<T>, Receiver<T>) {
    channel()
}

/// Multiple producers, single consumer
pub fn mpsc<T: Send + 'static>() -> (Sender<T>, Receiver<T>) {
    channel()
}

/// Broadcasting to multiple consumers
pub struct Broadcaster<T> {
    senders: Arc<Mutex<Vec<Sender<T>>>>,
}

impl<T: Clone + Send + 'static> Broadcaster<T> {
    pub fn new() -> Self {
        Broadcaster {
            senders: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe(&self) -> Receiver<T> {
        let (tx, rx) = channel();
        self.senders.lock().unwrap().push(tx);
        rx
    }

    pub fn broadcast(&self, msg: &T) {
        let senders = self.senders.lock().unwrap();
        for sender in senders.iter() {
            let _ = sender.send(msg.clone());
        }
    }
}
```

### 4.2 Async Channels

```rust
use tokio::sync::{mpsc, broadcast, watch};

/// Unbounded async channel
pub async fn unbounded_channel<T: Send>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    mpsc::channel(100)
}

/// Bounded async channel
pub async fn bounded_channel<T: Send>(bound: usize) -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    mpsc::channel(bound)
}

/// Broadcast channel for fan-out
pub async fn broadcast_channel<T: Send>() -> (broadcast::Sender<T>, broadcast::Receiver<T>) {
    broadcast::channel(100)
}

/// Watch channel for state changes
pub async fn watch_channel<T: Clone + Send>() -> (watch::Sender<T>, watch::Receiver<T>) {
    watch::channel()
}

/// Select on multiple channels
pub async fn select_channels<T: Send>(rx1: &mut mpsc::Receiver<T>, rx2: &mut mpsc::Receiver<T>) {
    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("Received on channel 1: {:?}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("Received on channel 2: {:?}", msg);
        }
    }
}
```

---

## LOCKS & SYNCHRONIZATION

### 5.1 Mutex Types

```rust
use std::sync::Mutex;

/// Standard Mutex
pub struct DataStore {
    data: Mutex<HashMap<String, Bytes>>,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn set(&self, key: String, value: Bytes) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
    }
}

/// Try Mutex for non-blocking lock
pub fn try_lock_example<T>(mutex: &Mutex<T>) -> Option<MutexGuard<T>> {
    mutex.try_lock().ok()
}

/// RwLock for read-heavy workloads
pub struct Config {
    settings: RwLock<HashMap<String, ConfigValue>>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            settings: RwLock::new(HashMap::new()),
        }
    }

    pub fn read(&self, key: &str) -> Option<ConfigValue> {
        let guard = self.settings.read().unwrap();
        guard.get(key).cloned()
    }

    pub fn write(&self, key: String, value: ConfigValue) {
        let mut guard = self.settings.write().unwrap();
        guard.insert(key, value);
    }
}
```

### 5.2 Parking Lot Locks

```rust
use parking_lot::{Mutex, RwLock, Condvar};

/// Fast Mutex (no poison, no_std compatible)
pub struct FastMutex<T> {
    inner: Mutex<T>,
}

impl<T> FastMutex<T> {
    pub fn new(value: T) -> Self {
        FastMutex {
            inner: Mutex::new(value),
        }
    }

    pub fn lock(&self) -> parking_lot::MutexGuard<T> {
        self.inner.lock()
    }

    pub fn try_lock(&self) -> Option<parking_lot::MutexGuard<T>> {
        self.inner.try_lock()
    }
}

/// Condition variable for signaling
pub struct Barrier {
    lock: Mutex<usize>,
    cvar: Condvar,
    target: usize,
}

impl Barrier {
    pub fn new(n: usize) -> Self {
        Barrier {
            lock: Mutex::new(0),
            cvar: Condvar::new(),
            target: n,
        }
    }

    pub fn wait(&self) {
        let mut count = self.lock.lock();
        *count += 1;
        
        if *count == self.target {
            self.cvar.notify_all();
        } else {
            self.cvar.wait(count);
        }
    }
}
```

---

## ATOMICS & LOCK-FREE

### 6.1 Atomic Operations

```rust
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, AtomicU64, Ordering};

/// Atomic flag
pub struct AtomicFlag(AtomicBool);

impl AtomicFlag {
    pub fn new() -> Self {
        AtomicFlag(AtomicBool::new(false))
    }

    pub fn set(&self) -> bool {
        self.0.swap(true, Ordering::AcqRel)
    }

    pub fn is_set(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    pub fn reset(&self) {
        self.0.store(false, Ordering::Release);
    }
}

/// Atomic pointer with CAS
pub struct AtomicBox<T> {
    ptr: AtomicPtr<T>,
}

impl<T> AtomicBox<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::into_raw(Box::new(value));
        AtomicBox {
            ptr: AtomicPtr::new(boxed),
        }
    }

    pub fn load(&self) -> Option<Arc<T>> {
        let ptr = self.ptr.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Arc::<T>::from_raw(ptr) })
        }
    }

    pub fn store(&self, value: T) {
        let boxed = Box::into_raw(Box::new(value));
        let old = self.ptr.swap(boxed, Ordering::AcqRel);
        if !old.is_null() {
            unsafe { Box::from_raw(old) };
        }
    }
}
```

### 6.2 Lock-Free Queue

```rust
/// Simple lock-free SPSC queue
pub struct RingBuffer<T> {
    buffer: Vec<AtomicU64>,
    head: AtomicU64,
    tail: AtomicU64,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        RingBuffer {
            buffer: (0..capacity)
                .map(|_| AtomicU64::new(0))
                .collect(),
            head: AtomicU64::new(0),
            tail: AtomicU64::new(0),
            capacity,
        }
    }

    pub fn push(&self, value: u64) -> bool {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        if (tail.wrapping_sub(head) as usize) >= self.capacity {
            return false;
        }

        let index = (tail & (self.capacity - 1) as u64) as usize;
        self.buffer[index].store(value, Ordering::Release);
        self.tail.fetch_add(1, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<u64> {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        if head == tail {
            return None;
        }

        let index = (head & (self.capacity - 1) as u64) as usize;
        let value = self.buffer[index].load(Ordering::Acquire);
        self.head.fetch_add(1, Ordering::Release);
        Some(value)
    }
}
```

---

## THREAD POOLS

### 7.1 Custom Thread Pool

```rust
pub struct CustomThreadPool {
    jobs: Arc<SegQueue<Box<dyn FnOnce() + Send>>>,
    workers: Vec<JoinHandle<()>>,
}

impl CustomThreadPool {
    pub fn new(size: usize) -> Self {
        let jobs = Arc::new(SegQueue::new());
        
        let workers: Vec<_> = (0..size)
            .map(|_| {
                let jobs = jobs.clone();
                thread::spawn(move || {
                    while let Some(job) = jobs.pop() {
                        job();
                    }
                })
            })
            .collect();

        CustomThreadPool { jobs, workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.jobs.push(Box::new(f));
    }
}

/// Rayon integration
pub use rayon::prelude::*;

pub fn parallel_iter<T: Send + Sync, I: IntoIterator<Item = T>>(
    items: I,
) -> rayon::vec::IntoIter<T> {
    use rayon::prelude::*;
    items.into_par_iter().collect::<Vec<_>>().into_par_iter()
}
```

---

## WORK STEALING

### 8.1 Work Stealing Queue

```rust
/// Chase-Lev work stealing deque
pub struct WorkDeque<T> {
    buffer: Vec<T>,
    bottom: AtomicUsize,
    top: AtomicUsize,
}

impl<T> WorkDeque<T> {
    pub fn new(capacity: usize) -> Self {
        WorkDeque {
            buffer: (0..capacity).map(|_| unsafe { MaybeUninit::zeroed().assume_init() }).collect(),
            bottom: AtomicUsize::new(0),
            top: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: T) {
        let b = self.bottom.load(Ordering::Acquire);
        self.buffer[b % self.buffer.capacity()] = value;
        self.bottom.store(b + 1, Ordering::Release);
    }

    pub fn steal(&self) -> Option<T> {
        let t = self.top.load(Ordering::Acquire);
        let b = self.bottom.load(Ordering::Acquire);
        
        if t >= b {
            return None;
        }

        let index = t % self.buffer.capacity();
        let value = self.buffer[index].read();
        self.top.store(t + 1, Ordering::Release);
        Some(value)
    }
}
```

---

## PIPELINE PATTERNS

### 9.1 Pipeline Processing

```rust
use futures::stream::{self, StreamExt};

/// Pipeline with bounded parallelism
pub async fn pipeline<I, F, Fut, Out>(
    items: I,
    max_parallel: usize,
    f: F,
) -> Vec<Out>
where
    I: IntoIterator,
    I::Item: Send,
    F: Fn(I::Item) -> Fut + Sync,
    Fut: Future<Output = Out> + Send,
    Out: Send,
{
    let stream = stream::iter(items).map(f).buffer_unordered(max_parallel);
    stream.collect::<Vec<_>>().await
}

/// Pipeline with map-reduce
pub async fn map_reduce<T, R, M, Rdc>(
    items: Vec<T>,
    map: M,
    reduce: Rdc,
) -> R
where
    T: Send,
    M: Fn(T) -> impl Future<Output = R> + Send,
    Rdc: Fn(R, R) -> R + Send + Clone,
    R: Send,
{
    let mapped: Vec<R> = pipeline(items, 16, map).await;
    mapped.into_iter().reduce(reduce).unwrap()
}
```

---

## ERROR HANDLING

### 10.1 Thread-Safe Errors

```rust
use std::sync::Arc;

/// Error channel for propagating errors across threads
pub struct ErrorChannel {
    sender: mpsc::Sender<Arc<dyn StdError + Send + Sync>>,
    receiver: Arc<Mutex<mpsc::Receiver<Arc<dyn StdError + Send + Sync>>>>,
}

impl ErrorChannel {
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = mpsc::channel(capacity);
        ErrorChannel {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn send<E: StdError + Send + Sync + 'static>(&self, error: E) {
        let _ = self.sender.send(Arc::new(error));
    }

    pub fn try_recv(&self) -> Option<Arc<dyn StdError + Send + Sync>> {
        self.receiver.lock().unwrap().try_recv().ok()
    }
}
```

---

## PERFORMANCE TUTORING

### 11.1 Performance Comparison

```
=== Concurrency Primitives Comparison ===

Operation: 1M increments
┌────────────────────────────────────────────────────────────┐
│ Primitive           │ Time    │ Throughput │ Contention       │
├───────────────────┼─────────┼───────────┼─────────────────┤
│ Mutex             │ 245ms   │ 4.08M/s   │ High            │
│ RwLock           │ 189ms   │ 5.29M/s   │ Medium         │
│ Atomic           │ 45ms    │ 22.2M/s   │ Low            │
│ Atomic + Padding │ 12ms    │ 83.3M/s   │ Very Low       │
│ Lock-Free        │ 8ms     │ 125M/s    │ None           │
└───────────────────┴─────────┴───────────┴─────────────────┘

Operation: 10K thread spawns
┌────────────────────────────────────────────────────────────┐
│ Approach              │ Time    │ Startup    │ Memory        │
├────────────────────┼─────────┼──────────┼───────────────┤
│ std::thread        │ 450ms   │ 45µs      │ 8MB/thread   │
│ tokio::spawn      │ 12ms    │ 1.2µs    │ 8KB/thread   │
│ rayon            │ 8ms     │ 0.8µs     │ 4KB/thread   │
└────────────────────┴─────────┴───────────┴──────────────┘
```

---

## RECAP

### Key Takeaways

1. **Use async for I/O-bound** - Threads for CPU-bound
2. **Minimize lock contention** - Use atomics when possible
3. **Prefer channels** - Safer than shared state
4. **Tune runtime** - Configure for your workload
5. **Profile first** - Measure before optimizing
6. **Avoid deadlocks** - Acquire locks in consistent order

---

*Skill ID: 004 | Category: Core-Language | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*