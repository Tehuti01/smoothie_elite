//! # smoothie-fx
//!
//! Production-quality audio effects — all zero-allocation at runtime.

pub mod reverb;
pub mod delay;
pub mod chorus;
pub mod phaser;
pub mod flanger;
pub mod tremolo;
pub mod vibrato;
pub mod autowah;
pub mod bitcrusher;
pub mod ringmod;
pub mod pitchshift;
pub mod stereo;
pub mod transient;
pub mod exciter;
pub mod tape;
pub mod cabinet;
pub mod compressor;
pub mod expander;
pub mod gate;
pub mod limiter;
pub mod eq;
pub mod distortion;
pub mod decimator;
pub mod freqshift;

pub use reverb::*;
pub use delay::*;
pub use chorus::*;
pub use phaser::*;
pub use flanger::*;
pub use tremolo::*;
pub use vibrato::*;
pub use autowah::*;
pub use bitcrusher::*;
pub use ringmod::*;
pub use pitchshift::*;
pub use stereo::*;
pub use transient::*;
pub use exciter::*;
pub use tape::*;
pub use cabinet::*;
pub use compressor::*;
pub use expander::*;
pub use gate::*;
pub use limiter::*;
pub use eq::*;
pub use distortion::*;
pub use decimator::*;
pub use freqshift::*;


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
