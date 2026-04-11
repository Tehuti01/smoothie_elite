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
