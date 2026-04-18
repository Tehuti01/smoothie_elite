extern crate num_complex;

pub mod filters;
pub mod dynamics;
pub mod delay;
pub mod oscillator;
pub mod envelope;
pub mod oversampling;
pub mod distortion;
pub mod spectral;
pub mod modulation;
pub mod physical;
pub mod convolution;
pub mod simd;

pub use filters::{BiquadFilter, FilterType, OnePoleFilter};
pub use dynamics::{Compressor, Limiter};
pub use delay::DelayLine;
pub use oscillator::{Oscillator, WaveShape};
pub use envelope::{AdsrEnvelope, EnvelopeState};
pub use distortion::{softclip, hardclip, tanh_shaper, foldback, TapeSaturator};
pub use spectral::analyze_harmonicity;


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
