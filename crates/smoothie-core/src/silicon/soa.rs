//! Data-Oriented Design: Structure of Arrays (SoA)
//! Organizes UI properties or DSP parameters into "Structures of Arrays" so that the CPU can stream data into the cache efficiently.
//! Instead of an array of Voice objects (AoS), you have parallel arrays of frequencies, phases, and amplitudes (SoA).

/// Helper macro to define a Structure of Arrays for a set of fields.
/// This maximizes cache hits during processing.
#[macro_export]
macro_rules! define_soa {
    ($name:ident, { $($field:ident: $t:ty),* $(,)? }) => {
        pub struct $name<const N: usize> {
            $( pub $field: [$t; N], )*
        }

        impl<const N: usize> $name<N> {
            pub fn new(default_val: fn() -> ($($t),*)) -> Self {
                // In a real elite implementation, we'd use MaybeUninit and Arena allocation
                let mut instance = unsafe { core::mem::MaybeUninit::<Self>::uninit().assume_init() };
                for i in 0..N {
                    let vals = default_val();
                    let mut _idx = 0;
                    $(
                        // This is a bit tricky with macro expansion for tuple indexing, 
                        // so we'll just use a simpler initialization for this demo.
                        instance.$field[i] = unsafe { core::mem::transmute_copy(&vals) }; 
                    )*
                }
                instance
            }
        }
    };
}

/// Example of manual SoA for 8 audio voices.
#[repr(align(64))]
pub struct AudioVoicesSoA<const N: usize> {
    pub frequencies: [f32; N],
    pub amplitudes: [f32; N],
    pub phases: [f32; N],
    pub active: [bool; N],
}

impl<const N: usize> AudioVoicesSoA<N> {
    pub fn new() -> Self {
        Self {
            frequencies: [0.0; N],
            amplitudes: [0.0; N],
            phases: [0.0; N],
            active: [false; N],
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
