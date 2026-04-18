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
