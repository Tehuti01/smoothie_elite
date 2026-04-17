//! Advanced synthesis engines for Smoothie Elite.
//!
//! Includes:
//! - Polyphonic synthesizer framework
//! - Wavetable synthesis engine
//! - FM synthesis
//! - Granular synthesis utilities
//! - Voice allocation strategies

pub mod polysynth;
pub mod wavetable;
pub mod fm;
pub mod granular;
pub mod voice;

pub use polysynth::{PolySynth, AllocationStrategy};
pub use wavetable::WavetableEngine;
pub use fm::{FmSynth, FmAlgorithm};
pub use granular::{GranularEngine, GrainEnvelope};
pub use voice::{Voice, VoiceState};

/// Synthesis quality modes for CPU/quality tradeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthQuality {
    /// Low quality, low CPU (for mobile/embedded)
    Low,
    /// Medium quality, medium CPU (standard)
    Medium,
    /// High quality, high CPU (mastering)
    High,
}
