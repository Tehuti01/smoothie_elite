//! # Amplifier Modeling Modules
//!
//! This module contains the core components of a guitar amplifier simulation.
//! It includes tube-stage emulation, high-gain preamps, sagging power amps, 
//! and dedicated distortion circuits.

pub mod amplifier;
pub mod distortion;
pub mod poweramp;
pub mod preamp;
pub mod tube_stage;

pub use amplifier::Amplifier;
pub use distortion::Distortion;
pub use poweramp::Poweramp;
pub use preamp::Preamp;
pub use tube_stage::TubeStage;


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
