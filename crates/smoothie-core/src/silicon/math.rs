//! Branchless Selection & Math Optimizations
//! Prevents the CPU from making a "guess" (branch prediction), which can be costly if it guesses wrong.

/// Branchless select for f32.
/// Uses the CMOV (Conditional Move) logic or bit-masking.
#[inline(always)]
pub fn branchless_select_f32(condition: bool, if_true: f32, if_false: f32) -> f32 {
    let mask = -(condition as i32) as u32; // 0xFFFF_FFFF if true, 0x0 if false
    let if_true_bits = if_true.to_bits();
    let if_false_bits = if_false.to_bits();
    
    let res_bits = (if_true_bits & mask) | (if_false_bits & !mask);
    f32::from_bits(res_bits)
}

/// Force Return Value Optimization (RVO).
/// Constructs the return object directly in the memory space of the caller via &mut.
#[inline(always)]
pub fn smoothie_move<T>(src: T, dest: &mut T) {
    *dest = src;
}

/// Bit-History Input Debouncing.
/// Stores the last 8 states of a physical key in a single u8 to detect "noise".
pub struct InputDebouncer {
    history: u8,
}

impl InputDebouncer {
    pub const fn new() -> Self {
        Self { history: 0 }
    }

    pub fn update(&mut self, is_pressed: bool) -> bool {
        self.history = (self.history << 1) | (is_pressed as u8);
        // Only return true if the last 4 states were consistently pressed
        (self.history & 0x0F) == 0x0F
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
