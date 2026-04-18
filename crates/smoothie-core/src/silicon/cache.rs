//! Cache-Line Alignment & Bouncing Mitigation
//! Aligns UI property structs to 64-byte boundaries to prevent "false sharing".

use core::ops::{Deref, DerefMut};

/// Enforces 64-byte alignment to match typical L1 cache line sizes.
/// This prevents False Sharing in multi-threaded rendering.
#[repr(align(64))]
#[derive(Debug, Clone, Copy, Default)]
pub struct CacheAligned<T>(pub T);

impl<T> Deref for CacheAligned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CacheAligned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Padding specifically designed for "False Sharing Padding" (64 bytes).
/// Inserted between atomic variables accessed by different threads.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CachePadding([u8; 64]);

impl Default for CachePadding {
    fn default() -> Self {
        CachePadding([0; 64])
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
