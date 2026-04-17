//! # smoothie-dsp
//!
//! Production-grade DSP primitives — zero allocation on audio thread.
//! Everything JUCE's `dsp::` namespace has, in safe Rust.

pub mod filters;
pub mod dynamics;
pub mod delay;
pub mod oscillator;
pub mod envelope;
pub mod oversampling;
pub mod distortion;
pub mod spectral;

pub use filters::{BiquadFilter, FilterType, OnePoleFilter};
pub use dynamics::{Compressor, Limiter};
pub use delay::DelayLine;
pub use oscillator::{Oscillator, WaveShape};
pub use envelope::{AdsrEnvelope, EnvelopeState};
pub use distortion::{softclip, hardclip, tanh_shaper, foldback, TapeSaturator};
pub use spectral::analyze_harmonicity;
