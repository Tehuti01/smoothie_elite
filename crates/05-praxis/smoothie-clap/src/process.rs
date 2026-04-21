/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbd3e9027 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/process.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::primitives::Sample;

/// Describes steady-state vs. active-render requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the RenderMode enumeration.
pub enum RenderMode {
    /// Normal real-time render — plugin may use real-time optimisations.
    Realtime,
    /// Offline / faster-than-realtime render — plugin may use higher quality.
    Offline,
}

/// Transport state delivered per-block from the host.
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the TransportInfo structure.
pub struct TransportInfo {
    pub is_playing: bool,
    pub is_recording: bool,
    pub is_loop_active: bool,
    pub tempo: f64,
    pub bar_start: f64,
    pub song_pos_beats: f64,
    pub time_sig_numerator: u16,
    pub time_sig_denominator: u16,
}

///
/// zero-allocation.
/// Technical implementation of the ClapAudioBuffer structure.
pub struct ClapAudioBuffer<'block> {
    pub channels: &'block mut [&'block mut [Sample]],
    pub channel_count: u32,
    pub latency: u32,
    pub constant_mask: u64,
}

/// Technical implementation of the ClapProcessContext structure.
pub struct ClapProcessContext<'block> {
    /// Monotonically increasing sample position since plugin instantiation.
    pub steady_time: i64,
    /// Number of samples in this block.
    pub frames_count: u32,
    /// Render mode requested by the host.
    pub render_mode: RenderMode,
    /// Optional transport / timeline information.
    pub transport: Option<TransportInfo>,
    /// Input audio buffers (borrowed from host).
    pub inputs: &'block [ClapAudioBuffer<'block>],
    /// Output audio buffers (borrowed from host, must be filled).
    pub outputs: &'block mut [ClapAudioBuffer<'block>],
}

impl<'block> ClapProcessContext<'block> {
    /// Convenience: write a single sample to output channel `ch`.
    #[inline(always)]
    /// Technical implementation of the write_output logic.
    pub fn write_output(&mut self, port: usize, channel: usize, frame: usize, value: Sample) {
        self.outputs[port].channels[channel][frame] = value;
    }

    /// Convenience: read a single sample from input channel `ch`.
    #[inline(always)]
    /// Technical implementation of the read_input logic.
    pub fn read_input(&self, port: usize, channel: usize, frame: usize) -> Sample {
        self.inputs[port].channels[channel][frame]
    }
}
