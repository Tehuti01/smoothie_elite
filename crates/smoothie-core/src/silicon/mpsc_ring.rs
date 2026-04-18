//! Lock-Free Multi-Producer Single-Consumer (MPSC) Ring Buffer.
//! Specialized "Smoothie Queue" where multiple worker threads can throw tasks at a single high-speed collector.
//! Uses an atomic exchange on the head-link to ensure producers never block each other.

use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr;

/// A node in the intrusive MPSC linked list.
#[repr(align(64))]
struct Node<T> {
    next: AtomicPtr<Node<T>>,
    value: Option<T>,
}

/// A Multi-Producer Single-Consumer (MPSC) Lock-Free Queue.
/// This is the secret to a high-speed backend logger or task collector.
pub struct MpscQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: *mut Node<T>,
    stub: *mut Node<T>,
}

unsafe impl<T: Send> Sync for MpscQueue<T> {}
unsafe impl<T: Send> Send for MpscQueue<T> {}

impl<T> MpscQueue<T> {
    pub fn new() -> Self {
        let stub = Box::into_raw(Box::new(Node {
            next: AtomicPtr::new(ptr::null_mut()),
            value: None,
        }));
        Self {
            head: AtomicPtr::new(stub),
            tail: stub,
            stub,
        }
    }

    /// Pushes a value onto the queue. Safe for multiple producers.
    pub fn push(&self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            next: AtomicPtr::new(ptr::null_mut()),
            value: Some(value),
        }));
        
        // Atomic exchange on the head to "claim" the position
        let prev = self.head.swap(node, Ordering::AcqRel);
        
        // Link the previous node to this one
        unsafe {
            (*prev).next.store(node, Ordering::Release);
        }
    }

    /// Pops a value from the queue. Only safe for a single consumer.
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let mut tail = self.tail;
            let mut next = (*tail).next.load(Ordering::Acquire);
            
            if tail == self.stub {
                if next.is_null() {
                    return None;
                }
                self.tail = next;
                tail = next;
                next = (*next).next.load(Ordering::Acquire);
            }
            
            if !next.is_null() {
                self.tail = next;
                let value = (*tail).value.take();
                let _ = Box::from_raw(tail); // This is simplified; real elite impl would use an Arena
                return value;
            }
            
            let head = self.head.load(Ordering::Acquire);
            if tail != head {
                return None;
            }
            
            self.push_stub();
            
            next = (*tail).next.load(Ordering::Acquire);
            if !next.is_null() {
                self.tail = next;
                let value = (*tail).value.take();
                let _ = Box::from_raw(tail);
                return value;
            }
            
            None
        }
    }

    unsafe fn push_stub(&mut self) {
        // Real implementation would handle the stub rotation for zero-alloc
    }
}

impl<T> Drop for MpscQueue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        unsafe {
            let _ = Box::from_raw(self.stub);
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
