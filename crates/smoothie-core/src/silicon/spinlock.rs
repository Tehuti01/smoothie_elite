//! Mutex-Free Spinlocks & Ticket Locks
//! Builds atomic locks using Compare-And-Swap (CAS) operations to bypass OS mutexes.

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::cell::UnsafeCell;

/// A simple spinlock using AtomicBool and atomic swap.
pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Yield or backoff to save CPU cycles (simulating mm_pause or yield)
            core::hint::spin_loop();
        }
        SpinlockGuard { lock: self }
    }
}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<'a, T> core::ops::Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// A fair ticket lock.
pub struct TicketLock<T> {
    serving: AtomicUsize,
    next_ticket: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for TicketLock<T> {}

impl<T> TicketLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            serving: AtomicUsize::new(0),
            next_ticket: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> TicketLockGuard<'_, T> {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        while self.serving.load(Ordering::Acquire) != ticket {
            core::hint::spin_loop();
        }
        TicketLockGuard { lock: self }
    }
}

pub struct TicketLockGuard<'a, T> {
    lock: &'a TicketLock<T>,
}

impl<'a, T> core::ops::Deref for TicketLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for TicketLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for TicketLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.serving.fetch_add(1, Ordering::Release);
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
