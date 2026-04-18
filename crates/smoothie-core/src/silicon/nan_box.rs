//! Pointer Tagging (NaN Boxing)
//! Stores small pieces of metadata (like type info or reference counts) in the unused high bits of a 64-bit pointer.
//! Since modern CPUs only use 48 bits for addressing, you have 16 bits of "free real estate" to play with.

use core::marker::PhantomData;

/// A tagged pointer that uses the upper 16 bits for metadata.
pub struct TaggedPtr<T> {
    data: usize,
    _phantom: PhantomData<T>,
}

impl<T> TaggedPtr<T> {
    const TAG_MASK: usize = 0xFFFF_0000_0000_0000;
    const PTR_MASK: usize = 0x0000_FFFF_FFFF_FFFF;

    /// Create a new tagged pointer from a raw pointer and a 16-bit tag.
    pub fn new(ptr: *mut T, tag: u16) -> Self {
        let ptr_val = ptr as usize;
        // Ensure the pointer doesn't use the upper bits already
        assert!((ptr_val & Self::TAG_MASK) == 0, "Pointer uses upper 16 bits");
        
        let tag_val = (tag as usize) << 48;
        Self {
            data: ptr_val | tag_val,
            _phantom: PhantomData,
        }
    }

    /// Extract the raw pointer.
    pub fn ptr(&self) -> *mut T {
        (self.data & Self::PTR_MASK) as *mut T
    }

    /// Extract the 16-bit tag.
    pub fn tag(&self) -> u16 {
        ((self.data & Self::TAG_MASK) >> 48) as u16
    }

    /// Update the tag while keeping the pointer.
    pub fn set_tag(&mut self, tag: u16) {
        let ptr_val = self.data & Self::PTR_MASK;
        let tag_val = (tag as usize) << 48;
        self.data = ptr_val | tag_val;
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
