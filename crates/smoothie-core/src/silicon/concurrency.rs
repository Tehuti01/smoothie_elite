//! Advanced Concurrency: Work Stealing & Lock-Free Hazard Pointers

use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::ptr;

/// A node in the hazard pointer linked list.
pub struct HazardRecord {
    pub pointer: AtomicPtr<u8>,
    pub next: *mut HazardRecord,
}

/// Lock-Free Hazard Pointers
/// Tracks which threads are "looking" at a piece of data so you know when it's safe to delete it.
/// Faster than Arc and safer than raw pointers for backend memory management.
pub struct HazardTracker {
    head: AtomicPtr<HazardRecord>,
}

unsafe impl Sync for HazardTracker {}
unsafe impl Send for HazardTracker {}

impl HazardTracker {
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Thread claims a hazard pointer slot to protect an object while reading it.
    pub fn acquire(&self) -> *mut HazardRecord {
        let mut curr = self.head.load(Ordering::Acquire);
        loop {
            if curr.is_null() {
                // If no available records, create a new one and prepend it.
                // In a true implementation, this uses a pre-allocated Thread-Local Arena
                let new_record = Box::into_raw(Box::new(HazardRecord {
                    pointer: AtomicPtr::new(ptr::null_mut()),
                    next: ptr::null_mut(),
                }));
                
                let mut head = self.head.load(Ordering::Relaxed);
                loop {
                    unsafe { (*new_record).next = head; }
                    match self.head.compare_exchange_weak(head, new_record, Ordering::Release, Ordering::Relaxed) {
                        Ok(_) => return new_record,
                        Err(h) => head = h,
                    }
                }
            } else {
                unsafe {
                    // Try to claim an empty hazard pointer (null)
                    let p = (*curr).pointer.load(Ordering::Relaxed);
                    if p.is_null() && (*curr).pointer.compare_exchange(ptr::null_mut(), 1 as *mut u8, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                        return curr;
                    }
                    curr = (*curr).next;
                }
            }
        }
    }

    /// Thread releases the hazard pointer slot after reading.
    pub fn release(&self, record: *mut HazardRecord) {
        unsafe {
            (*record).pointer.store(ptr::null_mut(), Ordering::Release);
        }
    }
}

/// Lock-Free Stealing Scheduler Deque
/// Allows idle CPU cores to "steal" tasks from the back of an overworked core's local queue.
/// Prevents the "Long Tail" latency problem.
pub struct WorkStealingDeque<T> {
    buffer: *mut T,
    mask: usize,
    bottom: AtomicUsize,
    top: AtomicUsize,
}

unsafe impl<T: Send> Sync for WorkStealingDeque<T> {}
unsafe impl<T: Send> Send for WorkStealingDeque<T> {}

impl<T> WorkStealingDeque<T> {
    pub fn new(capacity: usize) -> Self {
        // Assert power of two
        assert!(capacity > 0 && capacity & (capacity - 1) == 0);
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        
        Self {
            buffer: ptr,
            mask: capacity - 1,
            bottom: AtomicUsize::new(0),
            top: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, item: T) {
        let b = self.bottom.load(Ordering::Relaxed);
        let t = self.top.load(Ordering::Acquire);
        
        if b.wrapping_sub(t) > self.mask {
            return; // Queue is full. Real implementation would resize.
        }

        unsafe {
            core::ptr::write(self.buffer.add(b & self.mask), item);
        }
        
        // Ensure write is visible before bottom is incremented
        core::sync::atomic::fence(Ordering::Release);
        self.bottom.store(b.wrapping_add(1), Ordering::Relaxed);
    }

    /// Owner pops from the bottom of the deque.
    pub fn pop(&self) -> Option<T> {
        let b = self.bottom.load(Ordering::Relaxed);
        if b == 0 {
            return None;
        }
        let b = b.wrapping_sub(1);
        self.bottom.store(b, Ordering::Relaxed);
        
        core::sync::atomic::fence(Ordering::SeqCst);
        let t = self.top.load(Ordering::Relaxed);
        
        if t <= b {
            let item = unsafe { core::ptr::read(self.buffer.add(b & self.mask)) };
            if t == b {
                // Last item, check for stealing contention
                if self.top.compare_exchange(t, t.wrapping_add(1), Ordering::SeqCst, Ordering::Relaxed).is_err() {
                    // Steal won, we lost
                    self.bottom.store(b.wrapping_add(1), Ordering::Relaxed);
                    return None;
                }
                self.bottom.store(b.wrapping_add(1), Ordering::Relaxed);
            }
            Some(item)
        } else {
            self.bottom.store(b.wrapping_add(1), Ordering::Relaxed);
            None
        }
    }

    /// Thief core steals from the top of the deque.
    pub fn steal(&self) -> Option<T> {
        let t = self.top.load(Ordering::Acquire);
        core::sync::atomic::fence(Ordering::SeqCst);
        let b = self.bottom.load(Ordering::Acquire);
        
        if t < b {
            let item = unsafe { core::ptr::read(self.buffer.add(t & self.mask)) };
            if self.top.compare_exchange(t, t.wrapping_add(1), Ordering::SeqCst, Ordering::Relaxed).is_ok() {
                Some(item)
            } else {
                None // Contention with owner or another thief
            }
        } else {
            None // Empty
        }
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
