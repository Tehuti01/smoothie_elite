//! Direct-to-Hardware (D2H) Primitives
//! Stripping away the final layers of OS abstraction.


use core::sync::atomic::{AtomicU64, Ordering};


/// VRR Pacing (Point 151)
/// Predicting frame duration to eliminate stutter.
pub struct SiliconPacer {
    pub frame_deltas: [u64; 3],
}


impl SiliconPacer {
    pub fn predict_next_interval(&self) -> u64 {
        (self.frame_deltas[0] + self.frame_deltas[1] + self.frame_deltas[2]) / 3
    }
}


/// Lock-Free Atomic Bitset Schedulers (Point 155)
/// BLSR instruction for 1-cycle task claiming.
pub struct SiliconDispatcher {
    pub state: AtomicU64,
}


impl SiliconDispatcher {
    #[inline(always)]
    pub fn claim_task(&self) -> Option<u32> {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            let mut val = self.state.load(Ordering::Acquire);
            while val != 0 {
                // Point 155: BLSR resets lowest set bit
                let next = val & (val.wrapping_sub(1));
                if self.state.compare_exchange_weak(val, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                    return Some(val.trailing_zeros());
                }
                val = self.state.load(Ordering::Relaxed);
            }
            None
        }
        #[cfg(not(target_arch = "x86_64"))]
        { None }
    }
}


/// Cache-Line "Prefetch-W" (Write-Intent) (Point 157)
/// Prevent pipeline stalls by claiming exclusive ownership early.
#[inline(always)]
pub unsafe fn prefetch_write_intent(ptr: *const u8) {
    #[cfg(target_arch = "x86_64")]
    core::arch::x86_64::_mm_prefetch(ptr as *const i8, core::arch::x86_64::_MM_HINT_ET0);
}


/// Branchless Hex-to-Binary (Point 158)
/// Subtract-and-mask immune to timing attacks.
#[inline(always)]
pub fn fast_hex_to_nibble(c: u8) -> u8 {
    let mask = -( ((c >> 6) & 1) as i8 ) as u8;
    (c & 0x0F) + (mask & 9)
}


/// Explicit Instruction Alignment (Point 159)
/// Maximizing instruction fetcher efficiency.
#[repr(align(16))]
pub struct AlignedInstructionBlock;


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
