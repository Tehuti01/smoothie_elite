//! Zero-Cost Object Pools (Free-lists)
//! Pre-allocates a continuous block of memory and maintains an intrusive singly-linked list of free slots.
//! Avoids runtime allocations while keeping `Drop` semantics safe.

use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr::{self, NonNull};

/// An intrusive free-list node.
#[repr(C)]
struct FreeNode<T> {
    next: *mut FreeNode<T>,
    _phantom: core::marker::PhantomData<T>,
}

/// A zero-cost thread-safe object pool.
pub struct ObjectPool<T> {
    memory: NonNull<T>,
    capacity: usize,
    free_head: AtomicPtr<FreeNode<T>>,
}

unsafe impl<T: Send> Sync for ObjectPool<T> {}
unsafe impl<T: Send> Send for ObjectPool<T> {}

impl<T> ObjectPool<T> {
    /// Pre-allocates a continuous block of memory.
    pub fn new(capacity: usize) -> Self {
        assert!(core::mem::size_of::<T>() >= core::mem::size_of::<FreeNode<T>>(), "T is too small to contain a FreeNode");
        
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        let pool = Self {
            memory: NonNull::new(ptr).unwrap(),
            capacity,
            free_head: AtomicPtr::new(ptr::null_mut()),
        };

        // Initialize the free list
        unsafe {
            for i in 0..capacity {
                let node_ptr = ptr.add(i) as *mut FreeNode<T>;
                if i < capacity - 1 {
                    (*node_ptr).next = ptr.add(i + 1) as *mut FreeNode<T>;
                } else {
                    (*node_ptr).next = ptr::null_mut();
                }
            }
            pool.free_head.store(ptr as *mut FreeNode<T>, Ordering::Release);
        }

        pool
    }

    /// Borrows a slot from the pool. Returns `None` if out of memory.
    pub fn alloc(&self, value: T) -> Option<&mut T> {
        let mut current_head = self.free_head.load(Ordering::Acquire);
        loop {
            if current_head.is_null() {
                return None; // Pool is empty
            }
            let next_head = unsafe { (*current_head).next };
            match self.free_head.compare_exchange_weak(
                current_head,
                next_head,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    let ptr = current_head as *mut T;
                    unsafe { core::ptr::write(ptr, value); }
                    return Some(unsafe { &mut *ptr });
                }
                Err(new_head) => current_head = new_head,
            }
        }
    }

    /// Returns a slot to the pool.
    pub fn dealloc(&self, ptr: &mut T) {
        unsafe { core::ptr::drop_in_place(ptr); }
        let node_ptr = ptr as *mut _ as *mut FreeNode<T>;
        
        let mut current_head = self.free_head.load(Ordering::Relaxed);
        loop {
            unsafe { (*node_ptr).next = current_head; }
            match self.free_head.compare_exchange_weak(
                current_head,
                node_ptr,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(new_head) => current_head = new_head,
            }
        }
    }
}

impl<T> Drop for ObjectPool<T> {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
        unsafe { std::alloc::dealloc(self.memory.as_ptr() as *mut u8, layout); }
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
