//! # smoothie-math
//!
//! Fast, allocation-free mathematics for audio DSP.
//! Every function here is `#[inline]` and operates on `f32` unless noted.

pub mod windows;
pub mod interp;
pub mod freq;
pub mod fast;
pub mod decibels;
pub mod scales;
pub mod stats;
pub mod envelope;
pub mod trig;
pub mod random;
pub mod matrix;

pub use windows::*;
pub use interp::*;
pub use freq::*;
pub use fast::*;
pub use decibels::*;
pub use scales::*;
pub use stats::*;
pub use envelope::*;
pub use trig::*;
pub use random::*;
pub use matrix::*;


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
