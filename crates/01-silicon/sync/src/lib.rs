/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd481a217 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/sync/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate smoothie_core;
///
/// golden ratio cache-line stabilization.
use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};
use smoothie_core::constants::{F_13, F_89};

/// Technical implementation of the SmoothieMutex structure.
pub struct SmoothieMutex<T: ?Sized> {
    lock: AtomicBool,
    // Golden ratio padding to prevent false sharing in multi-core substrates
    _padding: [u8; F_89],
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for SmoothieMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for SmoothieMutex<T> {}

impl<T> SmoothieMutex<T> {
    /// Initializes a new instance of the associated type.
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            _padding: [0; F_89],
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> SmoothieMutex<T> {
    /// Lock the mutex with progressive Fibonacci backoff
    pub fn lock(&self) -> SmoothieMutexGuard<'_, T> {
        let mut fib_prev = 1;
        let mut fib_curr = 2;

        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Harmonic backoff: increase spin count by Fibonacci steps
            for _ in 0..fib_curr {
                spin_loop();
            }

            // Advance Fibonacci sequence up to its Silicon boundary (F_13 = 377)
            if fib_curr < F_13 {
                let next = fib_prev + fib_curr;
                fib_prev = fib_curr;
                fib_curr = next;
            }
        }

        SmoothieMutexGuard { mutex: self }
    }

    /// Technical implementation of the try_lock logic.
    pub fn try_lock(&self) -> Option<SmoothieMutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SmoothieMutexGuard { mutex: self })
        } else {
            None
        }
    }
}

/// Technical implementation of the SmoothieMutexGuard structure.
pub struct SmoothieMutexGuard<'a, T: ?Sized> {
    mutex: &'a SmoothieMutex<T>,
}

impl<'a, T: ?Sized> Deref for SmoothieMutexGuard<'a, T> {
    type Target = T;
    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for SmoothieMutexGuard<'a, T> {
    /// Technical implementation of the deref_mut logic.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for SmoothieMutexGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}

/// Technical implementation of the SmoothieRwLock structure.
pub struct SmoothieRwLock<T: ?Sized> {
    state: core::sync::atomic::AtomicIsize, // -1 for write, >0 for readers
    _padding: [u8; F_89],
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send + Sync> Send for SmoothieRwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for SmoothieRwLock<T> {}

impl<T> SmoothieRwLock<T> {
    /// Initializes a new instance of the associated type.
    pub const fn new(data: T) -> Self {
        Self {
            state: core::sync::atomic::AtomicIsize::new(0),
            _padding: [0; F_89],
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> SmoothieRwLock<T> {
    /// Technical implementation of the read logic.
    pub fn read(&self) -> SmoothieReadGuard<'_, T> {
        let mut fib_prev = 1;
        let mut fib_curr = 2;

        loop {
            let s = self.state.load(Ordering::Acquire);
            if s >= 0 {
                if self
                    .state
                    .compare_exchange_weak(s, s + 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    return SmoothieReadGuard { lock: self };
                }
            }

            for _ in 0..fib_curr {
                spin_loop();
            }
            if fib_curr < F_13 {
                let next = fib_prev + fib_curr;
                fib_prev = fib_curr;
                fib_curr = next;
            }
        }
    }

    /// Technical implementation of the write logic.
    pub fn write(&self) -> SmoothieWriteGuard<'_, T> {
        let mut fib_prev = 1;
        let mut fib_curr = 2;

        while self
            .state
            .compare_exchange_weak(0, -1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            for _ in 0..fib_curr {
                spin_loop();
            }
            if fib_curr < F_13 {
                let next = fib_prev + fib_curr;
                fib_prev = fib_curr;
                fib_curr = next;
            }
        }
        SmoothieWriteGuard { lock: self }
    }
}

/// Technical implementation of the SmoothieReadGuard structure.
pub struct SmoothieReadGuard<'a, T: ?Sized> {
    lock: &'a SmoothieRwLock<T>,
}

impl<'a, T: ?Sized> Deref for SmoothieReadGuard<'a, T> {
    type Target = T;
    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for SmoothieReadGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

/// Technical implementation of the SmoothieWriteGuard structure.
pub struct SmoothieWriteGuard<'a, T: ?Sized> {
    lock: &'a SmoothieRwLock<T>,
}

impl<'a, T: ?Sized> Deref for SmoothieWriteGuard<'a, T> {
    type Target = T;
    /// Technical implementation of the deref logic.
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> DerefMut for SmoothieWriteGuard<'a, T> {
    /// Technical implementation of the deref_mut logic.
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T: ?Sized> Drop for SmoothieWriteGuard<'a, T> {
    /// Technical implementation of the drop logic.
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
    }
}

/// Technical implementation of the SmoothieOnce structure.
pub struct SmoothieOnce {
    initialized: AtomicBool,
}

impl SmoothieOnce {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
        }
    }

    /// Technical implementation of the call_once logic.
    pub fn call_once<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        if self
            .initialized
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            f();
        }
    }

    /// Technical implementation of the is_completed logic.
    pub fn is_completed(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }
}
