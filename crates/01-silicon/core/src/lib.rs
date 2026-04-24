/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xca1a745b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * │ AI DIRECTIVE: See docs/AI_AGENT_GUIDE.md for "Fort Real" plugin design.  │
 * │ Ensure all synthesized plugins use advanced DSP over basic primitives.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

// Foundational audio processing primitives.
pub mod audio;
// High-performance buffer management.
pub mod buffer;
// Universal mathematical and temporal constants.
pub mod constants;
// Error types and recovery protocols.
pub mod error;
// Frequency-domain analysis and transformation.
pub mod frequency;
// Extended floating-point mathematics.
pub mod math;
// Fundamental silicon-direct primitives.
pub mod primitives;
// Non-blocking ring buffer implementations.
pub mod ring_buffer;
// High-precision sample rate orchestration.
pub mod sample;
// Real-time synchronization primitives.
pub mod sync;
// Core system types and definitions.
pub mod types;
// Word length and bit-depth orchestration.
pub mod word_length;
// Master plugin traits.
pub mod plugin;

pub mod prelude {
    pub use crate::audio::AudioFrame;
    pub use crate::buffer::DelayLine as CoreDelayLine;
    pub use crate::constants::{PHI, PHI_F64, PI, TAU};
    pub use crate::error::{Result as SmoothieResult, SmoothieError};
    pub use crate::math::{FloatExt, FloatExt64};
    pub use crate::primitives::*;
    pub use crate::ring_buffer::{DelayLine, RingBuffer, SampleRingBuffer, StereoRingBuffer};
    pub use crate::sample::SampleRate;
    pub use crate::seraphic_specification;
    pub use crate::sync::*;
    pub use crate::types::*;
    pub use crate::word_length::WordLength;
    pub use crate::PluginOsNode;
    pub use crate::plugin::{Reset, Latency, TailTime, ProcessBlock, ParamHandle};
}

pub use smoothie_macros::{seraphic_specification, SmoothieParams, build_timestamp};
pub use crate::plugin::{SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus, AudioProcessor, Reset, Latency, TailTime, ProcessBlock, ParamHandle};

/// Returns the current version of the Smoothie Elite framework.
pub fn version() -> &'static str {
    crate::constants::FRAMEWORK_VERSION
}

/// The Autonomous Node Trait: Defines the contract for all real-time processing blocks.
pub trait PluginOsNode: Send + Sync + Reset {
    /// Processes a single sample through the node.
    #[seraphic_specification(L0, A0)]
    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f64) -> f64;
}
