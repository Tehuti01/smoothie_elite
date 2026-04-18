//! 'Elite' SIMD-accelerated DSP utilities.
//! Leverages LLVM auto-vectorization through idiomatic Rust iterator patterns.

/// Multiplies a buffer by a gain factor using SIMD-friendly patterns.
#[inline(always)]
pub fn apply_gain_simd(data: &mut [f64], gain: f64) {
    // LLVM will vectorize this automatically if target-cpu=native
    data.iter_mut().for_each(|x| *x *= gain);
}

/// Accumulates (sums) one buffer into another using SIMD-friendly patterns.
#[inline(always)]
pub fn accumulate_simd(dst: &mut [f64], src: &[f64]) {
    let len = dst.len().min(src.len());
    for i in 0..len {
        dst[i] += src[i];
    }
}

/// Computes the energy (sum of squares) of a buffer.
#[inline(always)]
pub fn energy_simd(data: &[f64]) -> f64 {
    data.iter().map(|&x| x * x).sum()
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
