//! Small String Optimization (SSO) Implementation
//! Avoids heap allocation for short strings by storing data directly in the pointer's stack space.
//! Eliminates allocator pressure during rapid UI text updates.

use core::str;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

const INLINE_CAPACITY: usize = 23;

pub enum SsoString {
    Inline {
        len: u8,
        data: [u8; INLINE_CAPACITY],
    },
    Heap {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    },
}

impl SsoString {
    pub fn new() -> Self {
        Self::Inline {
            len: 0,
            data: [0; INLINE_CAPACITY],
        }
    }

    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if len <= INLINE_CAPACITY {
            let mut data = [0; INLINE_CAPACITY];
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), data.as_mut_ptr(), len);
            }
            Self::Inline {
                len: len as u8,
                data,
            }
        } else {
            let layout = Layout::array::<u8>(len).unwrap();
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
            }
            Self::Heap { ptr, len, cap: len }
        }
    }

    pub fn as_str(&self) -> &str {
        let (ptr, len) = match self {
            Self::Inline { len, data } => (data.as_ptr(), *len as usize),
            Self::Heap { ptr, len, .. } => (*ptr as *const u8, *len),
        };
        unsafe {
            let slice = core::slice::from_raw_parts(ptr, len);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl Drop for SsoString {
    fn drop(&mut self) {
        if let Self::Heap { ptr, cap, .. } = self {
            let layout = Layout::array::<u8>(*cap).unwrap();
            unsafe { dealloc(*ptr, layout) };
        }
    }
}

impl Default for SsoString {
    fn default() -> Self {
        Self::new()
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
