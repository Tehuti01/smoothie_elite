//! Custom Allocators: Placement New & Arena
//! Constructs objects directly into a pre-allocated "arena" of memory.
//! Prevents heap fragmentation in long-running audio backend services.

use core::cell::UnsafeCell;
use std::alloc::Layout;
use std::ptr::NonNull;

/// A simple thread-local bump allocator (Arena).
pub struct Arena {
    start: NonNull<u8>,
    end: NonNull<u8>,
    current: UnsafeCell<NonNull<u8>>,
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::from_size_align(capacity, 64).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        
        let start = NonNull::new(ptr).unwrap();
        let end = NonNull::new(unsafe { ptr.add(capacity) }).unwrap();
        
        Self {
            start,
            end,
            current: UnsafeCell::new(start),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc<T>(&self, value: T) -> Option<&mut T> {
        let layout = Layout::new::<T>();
        let align = layout.align();
        let size = layout.size();
        
        unsafe {
            let current_ptr = *self.current.get();
            let mut ptr = current_ptr.as_ptr() as usize;
            
            // Align up
            let align_mask = align - 1;
            ptr = (ptr + align_mask) & !align_mask;
            
            let aligned_ptr = ptr as *mut u8;
            let end_ptr = aligned_ptr.add(size);
            
            if end_ptr > self.end.as_ptr() {
                return None; // Out of memory
            }
            
            *self.current.get() = NonNull::new_unchecked(end_ptr);
            
            let result_ptr = aligned_ptr as *mut T;
            core::ptr::write(result_ptr, value);
            Some(&mut *result_ptr)
        }
    }

    pub fn reset(&self) {
        unsafe {
            *self.current.get() = self.start;
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let size = unsafe { self.end.as_ptr().offset_from(self.start.as_ptr()) } as usize;
        let layout = Layout::from_size_align(size, 64).unwrap();
        unsafe {
            std::alloc::dealloc(self.start.as_ptr(), layout);
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
