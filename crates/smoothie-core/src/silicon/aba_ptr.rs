//! Tagged Pointers for ABA Prevention
//! Uses the upper 16 bits of a 64-bit pointer as a version counter to prevent the ABA problem in lock-free CAS loops.

use core::sync::atomic::{AtomicU64, Ordering};

/// An atomic pointer that embeds a version counter.
pub struct AbaPointer<T> {
    data: AtomicU64,
    _phantom: core::marker::PhantomData<T>,
}

impl<T> AbaPointer<T> {
    /// Create a new ABA-protected pointer.
    pub fn new(ptr: *mut T) -> Self {
        Self {
            data: AtomicU64::new(ptr as u64), // Tag is initially 0
            _phantom: core::marker::PhantomData,
        }
    }

    /// Loads the current pointer and its version tag.
    pub fn load(&self, order: Ordering) -> (*mut T, u16) {
        let val = self.data.load(order);
        let ptr = (val & 0x0000_FFFF_FFFF_FFFF) as *mut T;
        let tag = ((val & 0xFFFF_0000_0000_0000) >> 48) as u16;
        (ptr, tag)
    }

    /// Safely attempts to swap the pointer, incrementing the version tag automatically.
    /// Fails if either the pointer or the version tag has changed since it was loaded.
    pub fn compare_exchange(
        &self, 
        current_ptr: *mut T, 
        current_tag: u16, 
        new_ptr: *mut T, 
        success_order: Ordering,
        failure_order: Ordering,
    ) -> Result<(*mut T, u16), (*mut T, u16)> {
        let current_val = (current_ptr as u64 & 0x0000_FFFF_FFFF_FFFF) | ((current_tag as u64) << 48);
        let new_tag = current_tag.wrapping_add(1);
        let new_val = (new_ptr as u64 & 0x0000_FFFF_FFFF_FFFF) | ((new_tag as u64) << 48);

        match self.data.compare_exchange(current_val, new_val, success_order, failure_order) {
            Ok(_) => Ok((new_ptr, new_tag)),
            Err(val) => {
                let ptr = (val & 0x0000_FFFF_FFFF_FFFF) as *mut T;
                let tag = ((val & 0xFFFF_0000_0000_0000) >> 48) as u16;
                Err((ptr, tag))
            }
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
