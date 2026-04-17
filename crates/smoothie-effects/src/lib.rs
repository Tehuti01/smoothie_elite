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

pub use reverb::ReverbProcessor;
pub use delay::DelayProcessor;
pub use compressor::CompressorProcessor;
pub use eq::EqProcessor;
pub use chorus::ChorusProcessor;
pub use distortion::DistortionProcessor;
pub use envelope::{Envelope, EnvelopeStage};
pub use lfo::{Lfo, LfoShape};

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
