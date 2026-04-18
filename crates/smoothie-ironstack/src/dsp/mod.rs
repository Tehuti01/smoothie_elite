//! # Digital Signal Processing Modules
//!
//! This module serves as the primary container for all audio processing logic. 
//! It is organized into several sub-modules specializing in different parts of
//! the guitar signal chain.
//!
//! ## Sub-Modules
//! - **amplifiers**: Preamp, poweramp, and distortion models.
//! - **cabinets**: Cabinet impulse responses and frequency modeling.
//! - **dynamics**: Noise gates, compressors, and limiters.
//! - **effects**: Modulation (chorus, flanger) and time-based (delay, reverb) effects.
//! - **eq**: High-grade parametric and graphic equalizers.
//! - **utilities**: Infrastructure tools like oversamplers, crossovers, and mixers.

pub mod amplifiers;
pub mod cabinets;
pub mod dynamics;
pub mod effects;
pub mod eq;
pub mod utilities;

pub use amplifiers::*;
pub use cabinets::*;
pub use dynamics::*;
pub use effects::*;
pub use eq::*;
pub use utilities::*;


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
