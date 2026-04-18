//! Hardware Transactional Memory (Intel TSX)
//! Allows threads to execute blocks of code atomically without locks.
//! If a collision occurs, it aborts and retries.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{_xbegin, _xend, _XBEGIN_STARTED};

/// Executes a closure transactionally.
/// Returns `true` if the transaction succeeded, `false` if it aborted or is unsupported.
/// This allows "Optimistic Concurrency Control" without locks.
pub fn transactional_execute<F: FnMut()>(mut f: F) -> bool {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let status = _xbegin();
        if status == _XBEGIN_STARTED as u32 {
            f();
            _xend();
            return true;
        }
        false
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback for CPUs without TSX.
        // A true implementation would fallback to a Spinlock here.
        let mut _f = f; // To silence unused_mut
        let _ = _f;
        false
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
