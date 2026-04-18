//! Page-Aligned HugePages & Memory-Mapped I/O
//! Requests memory from the kernel in aligned manifold chunks.

use std::ptr::NonNull;
use crate::silicon::geometry::RATIO_0;

/// A massive, uninterrupted highway of memory using aligned manifolds.
pub struct HugePageMemory {
    ptr: NonNull<u8>,
    size: usize,
}

impl HugePageMemory {
    /// Allocate memory using aligned 2MB chunks.
    pub fn new(base_size: usize) -> Option<Self> {
        // Calculate size based on the growth ratio for optimal manifold density
        let manifold_size = (base_size as f64 * RATIO_0) as usize;
        let page_size = 2 * 1024 * 1024;
        let size = (manifold_size + page_size - 1) & !(page_size - 1);

        #[cfg(target_os = "linux")]
        {
            use libc::{mmap, MAP_ANONYMOUS, MAP_PRIVATE, MAP_HUGETLB, PROT_READ, PROT_WRITE, MAP_FAILED};
            let ptr = unsafe {
                mmap(
                    core::ptr::null_mut(),
                    size,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB,
                    -1,
                    0,
                )
            };
            
            if ptr == MAP_FAILED {
                return None;
            }
            
            Some(Self {
                ptr: NonNull::new(ptr as *mut u8).unwrap(),
                size,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            let layout = std::alloc::Layout::from_size_align(size, page_size).ok()?;
            let ptr = unsafe { std::alloc::alloc(layout) };
            if ptr.is_null() {
                return None;
            }
            Some(Self {
                ptr: NonNull::new(ptr).unwrap(),
                size,
            })
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Treat the manifold memory as a slice of type T.
    pub unsafe fn as_slice_mut<T>(&self) -> &mut [T] {
        let count = self.size / core::mem::size_of::<T>();
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr.as_ptr() as *mut T, count)
        }
    }
}

impl Drop for HugePageMemory {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        unsafe {
            libc::munmap(self.ptr.as_ptr() as *mut libc::c_void, self.size);
        }
        #[cfg(not(target_os = "linux"))]
        unsafe {
            let layout = std::alloc::Layout::from_size_align(self.size, 2 * 1024 * 1024).unwrap();
            std::alloc::dealloc(self.ptr.as_ptr(), layout);
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
