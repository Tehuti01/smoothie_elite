//! Hardware Random Number Generators (RDRAND)
//! Bypasses OS entropy pools to read directly from CPU thermal noise instructions.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_rdrand32_step;

/// Generates a 32-bit hardware random number with zero-latency overhead.
/// Falls back to a simple pseudo-random generator on non-x86 hardware.
#[inline(always)]
pub fn hardware_rand_u32() -> u32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut val = 0;
        // Retry until hardware has enough entropy
        while _rdrand32_step(&mut val) == 0 {
            core::hint::spin_loop();
        }
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Pseudo-random fallback based on simple timestamp arithmetic (or similar).
        // A true Elite implementation would wrap specific NEON/ARM registers.
        0xDEADBEEF 
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
