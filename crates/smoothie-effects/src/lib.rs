//! Professional audio effects suite for Smoothie Elite.
//!
//! Includes:
//! - Reverb (algorithmic, diffuse)
//! - Delay (multi-tap, feedback)
//! - Compressor/Limiter
//! - 3-band EQ
//! - Chorus/Flanger
//! - Distortion/Saturation
//! - Utility processors

pub mod reverb;
pub mod delay;
pub mod compressor;
pub mod eq;
pub mod chorus;
pub mod distortion;
pub mod envelope;
pub mod lfo;
pub mod filters;

pub use reverb::ReverbProcessor;
pub use delay::DelayProcessor;
pub use compressor::CompressorProcessor;
pub use eq::EqProcessor;
pub use chorus::ChorusProcessor;
pub use distortion::{DistortionProcessor, DistortionType};
pub use envelope::{Envelope, EnvelopeStage};
pub use lfo::{Lfo, LfoShape};
pub use filters::{BiquadFilter, FilterType};

/// Common trait for all effect processors.
pub trait EffectProcessor: Send + Sync {
    /// Process a single sample.
    fn process(&mut self, input: f32) -> f32;

    /// Process a stereo pair.
    fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }

    /// Reset internal state.
    fn reset(&mut self);

    /// Update sample rate (call on sample rate change).
    fn set_sample_rate(&mut self, sr: f32);
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
