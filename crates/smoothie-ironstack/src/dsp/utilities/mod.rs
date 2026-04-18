//! # DSP Utility Modules
//!
//! This module provides infrastructure and helper tools used by more complex
//! DSP modules. These include oversampling for alias reduction, crossovers
//! for multiband processing, and basic signal routing tools like mixers and panners.

pub mod crossover;
pub mod mixer;
pub mod oversampler;
pub mod panner;

pub use crossover::Crossover;
pub use mixer::Mixer;
pub use oversampler::Oversampler;
pub use panner::Panner;


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
