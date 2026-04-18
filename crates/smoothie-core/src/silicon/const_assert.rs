//! Compile-Time Layout Assertions
//! Validating cache-line alignments, sizes, and padding offsets purely during the compilation phase.

/// Asserts that a struct is exactly the expected size in bytes.
/// Ensures you haven't accidentally bloated a performance-critical type.
#[macro_export]
macro_rules! const_assert_size {
    ($struct:ty, $size:expr) => {
        const _: [(); $size] = [(); core::mem::size_of::<$struct>()];
    };
}

/// Asserts that a struct has the expected alignment.
/// Ensures `CacheAligned` types are actually placed correctly.
#[macro_export]
macro_rules! const_assert_align {
    ($struct:ty, $align:expr) => {
        const _: [(); $align] = [(); core::mem::align_of::<$struct>()];
    };
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
