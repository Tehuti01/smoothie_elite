/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3426cc7d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/sync.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::fmt;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Technical implementation of the Mutex structure.
pub struct Mutex<T> {
    locked: core::sync::atomic::AtomicBool,
    data: core::cell::UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Create new mutex
    pub const fn new(data: T) -> Self {
        Self {
            locked: core::sync::atomic::AtomicBool::new(false),
            data: core::cell::UnsafeCell::new(data),
        }
    }

    /// Lock the mutex (spins until available)
    pub fn lock(&self) -> MutexGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(
                false,
                true,
                core::sync::atomic::Ordering::Acquire,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            core::hint::spin_loop();
        }
        MutexGuard { mutex: self }
    }

    /// Try to lock non-blocking
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .locked
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::Acquire,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }

    /// Get reference to inner data (unsafe - only valid when locked)
    /// # Safety
    /// This method allows mutable access to the data through a shared reference.
    /// The caller must ensure that no other threads are accessing the data.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }

    /// Consume and return inner data
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mutex").finish()
    }
}

impl<T: Default> Default for Mutex<T> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Technical implementation of the MutexGuard structure.
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    /// Technical implementation of the deref_mut logic.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.mutex
            .locked
            .store(false, core::sync::atomic::Ordering::Release);
    }
}

/// Atomic reference counter (Simplified for no_std, requires static allocation or external heap)
/// Technical implementation of the Arc structure.
pub struct Arc<T> {
    inner: *const ArcInner<T>,
}

struct ArcInner<T> {
    data: T,
    count: core::sync::atomic::AtomicUsize,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    /// Create new Arc (Silicon warning: Requires a stable pointer)
    pub fn new(_data: T) -> Self {
        // --- PROPER Arc WOULD ALLOCATE ON HEAP ---
        // Placeholder for structural stability.
        // In this no_std/no_alloc layer, we provide a raw pointer foundation.
        Self {
            inner: core::ptr::null(),
        }
    }

    /// Clone Arc (Atomic increment)
    pub fn clone_arc(&self) -> Self {
        unsafe {
            (*self.inner)
                .count
                .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
        Self { inner: self.inner }
    }

    /// Get reference to data
    pub fn as_ref(&self) -> &T {
        unsafe { &(*self.inner).data }
    }
}

impl<T> Clone for Arc<T> {
    /// Technical implementation of the clone logic.
    fn clone(&self) -> Self {
        self.clone_arc()
    }
}

impl<T> Drop for Arc<T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                if (*self.inner)
                    .count
                    .fetch_sub(1, core::sync::atomic::Ordering::Release)
                    == 1
                {
                    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                    // Would deallocate here if we had an allocator
                }
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Arc<T> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arc").finish()
    }
}

/// Technical implementation of the RwLock structure.
pub struct RwLock<T> {
    state: core::sync::atomic::AtomicIsize, // -1 for write, else reader count
    data: core::cell::UnsafeCell<T>,
}

unsafe impl<T: Send + Sync> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    /// Create new RwLock
    pub const fn new(data: T) -> Self {
        Self {
            state: core::sync::atomic::AtomicIsize::new(0),
            data: core::cell::UnsafeCell::new(data),
        }
    }

    /// Acquire read lock
    pub fn read(&self) -> ReadGuard<'_, T> {
        loop {
            let s = self.state.load(core::sync::atomic::Ordering::Acquire);
            if s >= 0 {
                if self
                    .state
                    .compare_exchange_weak(
                        s,
                        s + 1,
                        core::sync::atomic::Ordering::Acquire,
                        core::sync::atomic::Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    return ReadGuard { lock: self };
                }
            }
            core::hint::spin_loop();
        }
    }

    /// Acquire write lock
    pub fn write(&self) -> WriteGuard<'_, T> {
        while self
            .state
            .compare_exchange(
                0,
                -1,
                core::sync::atomic::Ordering::Acquire,
                core::sync::atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            core::hint::spin_loop();
        }
        WriteGuard { lock: self }
    }

    /// Into inner data
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: fmt::Debug> fmt::Debug for RwLock<T> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RwLock").finish()
    }
}

impl<T: Default> Default for RwLock<T> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// Technical implementation of the ReadGuard structure.
pub struct ReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> core::ops::Deref for ReadGuard<'a, T> {
    type Target = T;

    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> Drop for ReadGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.lock
            .state
            .fetch_sub(1, core::sync::atomic::Ordering::Release);
    }
}

/// Technical implementation of the WriteGuard structure.
pub struct WriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<'a, T> core::ops::Deref for WriteGuard<'a, T> {
    type Target = T;

    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for WriteGuard<'a, T> {
    /// Technical implementation of the deref_mut logic.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for WriteGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.lock
            .state
            .store(0, core::sync::atomic::Ordering::Release);
    }
}

/// Technical implementation of the Once structure.
pub struct Once {
    initialized: AtomicBool,
}

impl Once {
    /// Create new Once
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
        }
    }

    /// Call closure exactly once
    pub fn call_once<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        if self
            .initialized
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
        {
            f();
        }
    }

    /// Check if closure has been called
    pub fn is_completed(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }
}

impl Default for Once {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Once {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Once").finish()
    }
}

/// Technical implementation of the Barrier structure.
pub struct Barrier {
    count: AtomicUsize,
    expected: usize,
    generation: AtomicUsize,
}

impl Barrier {
    /// Create new barrier for N threads
    pub const fn new(n: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            expected: n,
            generation: AtomicUsize::new(0),
        }
    }

    /// Wait for all threads at barrier
    pub fn wait(&self) -> BarrierWaitResult {
        let gen = self.generation.load(core::sync::atomic::Ordering::Acquire);
        let count = self
            .count
            .fetch_add(1, core::sync::atomic::Ordering::AcqRel);

        if count + 1 >= self.expected {
            self.count.store(0, core::sync::atomic::Ordering::Release);
            self.generation
                .fetch_add(1, core::sync::atomic::Ordering::Release);
            BarrierWaitResult {
                is_leader: true,
                generation: gen,
            }
        } else {
            // Spin until generation changes
            while self.generation.load(core::sync::atomic::Ordering::Acquire) == gen {
                core::hint::spin_loop();
            }
            BarrierWaitResult {
                is_leader: false,
                generation: gen,
            }
        }
    }

    /// Reset barrier
    pub fn reset(&self) {
        self.count.store(0, core::sync::atomic::Ordering::Release);
        self.generation
            .store(0, core::sync::atomic::Ordering::Release);
    }
}

impl fmt::Debug for Barrier {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Barrier")
            .field("count", &self.count)
            .field("expected", &self.expected)
            .finish()
    }
}

/// Technical implementation of the BarrierWaitResult structure.
pub struct BarrierWaitResult {
    pub is_leader: bool,
    pub generation: usize,
}

/// Technical implementation of the CondVar structure.
pub struct CondVar {
    waiters: AtomicUsize,
    signal_count: AtomicUsize,
}

impl CondVar {
    /// Create new condition variable
    pub const fn new() -> Self {
        Self {
            waiters: AtomicUsize::new(0),
            signal_count: AtomicUsize::new(0),
        }
    }

    /// Wait on condition (spins until signaled)
    pub fn wait(&self) {
        self.waiters
            .fetch_add(1, core::sync::atomic::Ordering::AcqRel);
        let signal = self
            .signal_count
            .load(core::sync::atomic::Ordering::Acquire);

        // Spinlock until signaled
        while self
            .signal_count
            .load(core::sync::atomic::Ordering::Acquire)
            == signal
            && self.waiters.load(core::sync::atomic::Ordering::Acquire) > 0
        {
            core::hint::spin_loop();
        }

        self.waiters
            .fetch_sub(1, core::sync::atomic::Ordering::AcqRel);
    }

    /// Notify one waiter
    pub fn notify_one(&self) {
        if self.waiters.load(core::sync::atomic::Ordering::Acquire) > 0 {
            self.signal_count
                .fetch_add(1, core::sync::atomic::Ordering::Release);
        }
    }

    /// Notify all waiters
    pub fn notify_all(&self) {
        let w = self.waiters.load(core::sync::atomic::Ordering::Acquire);
        if w > 0 {
            self.signal_count
                .fetch_add(w, core::sync::atomic::Ordering::Release);
        }
    }

    /// Get waiter count
    pub fn waiter_count(&self) -> usize {
        self.waiters.load(Ordering::Acquire)
    }
}

impl Default for CondVar {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CondVar {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CondVar")
            .field("waiters", &self.waiters)
            .finish()
    }
}

/// Technical implementation of the Semaphore structure.
pub struct Semaphore {
    permits: AtomicUsize,
    initial_permits: usize,
}

impl Semaphore {
    /// Create new semaphore with N permits
    pub const fn new(permits: usize) -> Self {
        Self {
            permits: AtomicUsize::new(permits),
            initial_permits: permits,
        }
    }

    /// Acquire a permit (blocking)
    pub fn acquire(&self) -> bool {
        loop {
            let p = self.permits.load(core::sync::atomic::Ordering::Acquire);
            if p > 0 {
                if self
                    .permits
                    .compare_exchange_weak(
                        p,
                        p - 1,
                        core::sync::atomic::Ordering::AcqRel,
                        core::sync::atomic::Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    return true;
                }
            }
            core::hint::spin_loop();
        }
    }

    /// Try to acquire non-blocking
    pub fn try_acquire(&self) -> bool {
        let p = self.permits.load(core::sync::atomic::Ordering::Acquire);
        if p > 0 {
            self.permits
                .compare_exchange(
                    p,
                    p - 1,
                    core::sync::atomic::Ordering::AcqRel,
                    core::sync::atomic::Ordering::Relaxed,
                )
                .is_ok()
        } else {
            false
        }
    }

    /// Release a permit
    pub fn release(&self) {
        let p = self.permits.load(core::sync::atomic::Ordering::Acquire);
        if p < self.initial_permits {
            self.permits
                .fetch_add(1, core::sync::atomic::Ordering::Release);
        }
    }

    /// Get available permits
    pub fn available(&self) -> usize {
        self.permits.load(Ordering::Acquire)
    }
}

impl fmt::Debug for Semaphore {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Semaphore")
            .field("permits", &self.permits)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_mutex logic.
    fn test_mutex() {
        let m = Mutex::new(42);
        let mut guard = m.lock();
        *guard = 43;
        assert_eq!(*guard, 43);
    }

    #[test]
    /// Technical implementation of the test_rwlock logic.
    fn test_rwlock() {
        let lock = RwLock::new(100);
        {
            let read = lock.read();
            assert_eq!(*read, 100);
        }
        {
            let mut write = lock.write();
            *write = 200;
        }
        assert_eq!(*lock.read(), 200);
    }

    #[test]
    /// Technical implementation of the test_once logic.
    fn test_once() {
        let once = Once::new();
        let mut count = 0;
        once.call_once(|| {
            count += 1;
        });
        once.call_once(|| {
            count += 1;
        });
        assert_eq!(count, 1);
        assert!(once.is_completed());
    }

    #[test]
    /// Technical implementation of the test_barrier logic.
    fn test_barrier() {
        let barrier = Barrier::new(1); // Single thread barrier test
        let result = barrier.wait();
        assert!(result.is_leader);
    }

    #[test]
    /// Technical implementation of the test_semaphore logic.
    fn test_semaphore() {
        let sem = Semaphore::new(3);
        assert!(sem.try_acquire());
        assert_eq!(sem.available(), 2);
    }

    #[test]
    /// Technical implementation of the test_condvar logic.
    fn test_condvar() {
        let cv = CondVar::new();
        assert_eq!(cv.waiter_count(), 0);
        // Only test notification logic in single thread
        cv.notify_one();
    }
}
