//! Process and initialization context passed to `SmoothiePlugin` callbacks.

use crate::buffer::AudioBuffer;

/// Context available during `SmoothiePlugin::process()`.
///
/// Provides access to the audio buffer, transport state, and timing info.
/// All methods on this type are zero-cost in release mode.
pub struct ProcessContext<'a> {
    buffer: AudioBuffer<'a>,
    transport: TransportInfo,
    sample_rate: f32,
    block_size: u32,
}

impl<'a> ProcessContext<'a> {
    /// Create — called by the host wrapper, not by plugin code.
    pub fn new(
        buffer: AudioBuffer<'a>,
        transport: TransportInfo,
        sample_rate: f32,
        block_size: u32,
    ) -> Self {
        Self { buffer, transport, sample_rate, block_size }
    }

    /// Immutable access to the audio buffer.
    #[inline]
    pub fn buffer(&self) -> &AudioBuffer<'a> { &self.buffer }

    /// Mutable access to the audio buffer.
    #[inline]
    pub fn buffer_mut(&mut self) -> &mut AudioBuffer<'a> { &mut self.buffer }

    /// Host sample rate in Hz.
    #[inline]
    pub fn sample_rate(&self) -> f32 { self.sample_rate }

    /// Number of samples in this processing block.
    #[inline]
    pub fn block_size(&self) -> u32 { self.block_size }

    /// DAW transport / timeline information.
    #[inline]
    pub fn transport(&self) -> &TransportInfo { &self.transport }
}

/// Context available during `SmoothiePlugin::initialize()`.
pub struct InitContext {
    pub sample_rate: f32,
    pub max_block_size: u32,
    pub channel_count_in: u32,
    pub channel_count_out: u32,
}

/// DAW transport and timeline state.
#[derive(Debug, Clone, Default)]
pub struct TransportInfo {
    /// Whether the transport is currently playing.
    pub is_playing: bool,
    /// Whether the transport is recording.
    pub is_recording: bool,
    /// Current position in samples from the start.
    pub position_samples: i64,
    /// Current position in beats.
    pub position_beats: Option<f64>,
    /// Beats per minute. `None` if unknown.
    pub bpm: Option<f64>,
    /// Time signature numerator.
    pub time_sig_num: Option<u32>,
    /// Time signature denominator.
    pub time_sig_denom: Option<u32>,
}

impl TransportInfo {
    /// Compute position in seconds.
    pub fn position_seconds(&self, sample_rate: f32) -> f64 {
        self.position_samples as f64 / sample_rate as f64
    }
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
