//! Lock-Free Atomic Ring Buffers
//! Enables high-speed communication between network thread and worker threads without using mutexes.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

/// A Single-Producer Single-Consumer (SPSC) Lock-Free Ring Buffer.
/// Size `N` must be a power of two.
pub struct SpscQueue<T, const N: usize> {
    head: AtomicUsize,
    tail: AtomicUsize,
    data: [UnsafeCell<MaybeUninit<T>>; N],
}

unsafe impl<T: Send, const N: usize> Sync for SpscQueue<T, N> {}
unsafe impl<T: Send, const N: usize> Send for SpscQueue<T, N> {}

impl<T, const N: usize> SpscQueue<T, N> {
    pub const fn new() -> Self {
        // Assert N is a power of 2
        assert!(N > 0 && (N & (N - 1)) == 0, "SpscQueue size must be a power of 2");
        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            data: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn push(&self, value: T) -> Result<(), T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        
        if head.wrapping_sub(tail) >= N {
            return Err(value); // Queue is full
        }

        let idx = head & (N - 1);
        unsafe {
            (*self.data[idx].get()).write(value);
        }

        self.head.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        
        if head == tail {
            return None; // Queue is empty
        }

        let idx = tail & (N - 1);
        let value = unsafe {
            (*self.data[idx].get()).assume_init_read()
        };

        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(value)
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
