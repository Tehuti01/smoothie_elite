/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd5beac98 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/time_stretch.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::{vec, vec::Vec};
use smoothie_core::primitives::Sample;

#[repr(align(64))]
/// Technical implementation of the TimeStretch structure.
pub struct TimeStretch {
    stretch_factor: f32,
    window_size: usize,
    overlap: usize,
    search_range: usize,
    output_buffer: Vec<Sample>,
    input_buffer: Vec<Sample>,
    read_pos: f32,
    write_pos: usize,
    window: Vec<f32>,
    sample_rate: f32,
}

impl TimeStretch {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let window_size = 1024;
        let overlap = 4;

        let mut window = vec![0.0; window_size];
        for i in 0..window_size {
            window[i] = (core::f32::consts::PI * i as f32 / window_size as f32).sin();
        }

        Self {
            stretch_factor: 1.0,
            window_size,
            overlap,
            search_range: 256,
            output_buffer: Vec::new(),
            input_buffer: vec![0.0; sample_rate as usize],
            read_pos: 0.0,
            write_pos: 0,
            window,
            sample_rate,
        }
    }

    /// Technical implementation of the set_stretch logic.
    pub fn set_stretch(&mut self, factor: f32) {
        self.stretch_factor = factor.clamp(0.25, 4.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        // Write input
        self.input_buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.input_buffer.len();

        // Check if we need to produce output
        let hop_out = self.window_size / self.overlap;
        let hop_in = (hop_out as f32 * self.stretch_factor) as usize;

        let output = if (self.read_pos as usize) >= self.window_size / 2 {
            let out = self.wsola_process();
            self.read_pos -= self.window_size as f32 / 2.0;
            Some(out)
        } else {
            self.read_pos += hop_in as f32;
            None
        };

        output.unwrap_or(0.0)
    }

    /// Technical implementation of the wsola_process logic.
    fn wsola_process(&mut self) -> Sample {
        let read_idx = (self.read_pos as usize) % self.input_buffer.len();

        // SimpleOLA for now - WSOLA would find best match
        let mut sum = 0.0;
        let half_win = self.window_size / 2;

        for i in 0..self.window_size {
            let idx = (read_idx + i).saturating_sub(half_win) % self.input_buffer.len();
            sum += self.input_buffer[idx] * self.window[i];
        }

        sum * 2.0 / self.window_size as f32
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// WSOLA time stretcher with overlap-add synthesis
#[repr(align(64))]
/// Technical implementation of the WsolaStretcher structure.
pub struct WsolaStretcher {
    factor: f32,
    window: usize,
    overlap_factor: usize,
    search_width: usize,
    analysis_buf: Vec<Sample>,
    synthesis_buf: Vec<Sample>,
    analysis_pos: f32,
    synthesis_pos: usize,
    window_env: Vec<f32>,
    sample_rate: f32,
}

impl WsolaStretcher {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let window = 2048;

        let mut window_env = vec![0.0; window];
        for i in 0..window {
            let hann = (core::f32::consts::PI * i as f32 / window as f32).sin();
            window_env[i] = hann * hann;
        }

        Self {
            factor: 1.0,
            window,
            overlap_factor: 4,
            search_width: 256,
            analysis_buf: vec![0.0; sample_rate as usize],
            synthesis_buf: vec![0.0; sample_rate as usize],
            analysis_pos: 0.0,
            synthesis_pos: 0,
            window_env,
            sample_rate,
        }
    }

    /// Technical implementation of the set_stretch logic.
    pub fn set_stretch(&mut self, stretch: f32) {
        self.factor = stretch.clamp(0.5, 2.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        // Write to analysis buffer
        let write_idx = self.synthesis_pos % self.analysis_buf.len();
        self.analysis_buf[write_idx] = input;

        // Advance analysis position
        let hop = self.window / self.overlap_factor;
        self.analysis_pos += 1.0;

        // When we've advanced enough, produce output
        let out_pos = (self.analysis_pos / self.factor) as usize;
        let output = if out_pos >= self.synthesis_pos + hop / 2 {
            let idx = (out_pos - self.synthesis_pos) % self.synthesis_buf.len();

            // Compute OLA
            let mut sum = 0.0;
            let start = out_pos.saturating_sub(self.window / 2);

            for i in 0..self.window {
                let pos = (start + i) % self.analysis_buf.len();
                sum += self.analysis_buf[pos] * self.window_env[i];
            }

            self.synthesis_buf[idx] = sum;
            self.synthesis_pos += 1;

            Some(self.synthesis_buf[(self.synthesis_pos - 1) % self.synthesis_buf.len()])
        } else {
            None
        };

        output.unwrap_or(0.0)
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// Elastane-style time stretching
#[repr(align(64))]
/// Technical implementation of the ElastaneStretch structure.
pub struct ElastaneStretch {
    tempo: f32,
    window: usize,
    buffer: Vec<Sample>,
    write_idx: usize,
    read_idx: f32,
    overlap: usize,
    window_func: Vec<f32>,
    sample_rate: f32,
}

impl ElastaneStretch {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let window = 4096;

        let mut wf = vec![0.0; window];
        for i in 0..window {
            wf[i] = 0.5 * (1.0 - (2.0 * core::f32::consts::PI * i as f32 / window as f32).cos());
        }

        Self {
            tempo: 1.0,
            window,
            buffer: vec![0.0; sample_rate as usize * 2],
            write_idx: 0,
            read_idx: 0.0,
            overlap: 8,
            window_func: wf,
            sample_rate,
        }
    }

    /// Technical implementation of the set_tempo logic.
    pub fn set_tempo(&mut self, tempo: f32) {
        self.tempo = tempo.clamp(0.25, 4.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        self.buffer[self.write_idx] = input;
        self.write_idx = (self.write_idx + 1) % self.buffer.len();

        let hop = self.window / self.overlap;

        // Read position advances at different rate based on tempo
        self.read_idx += hop as f32 / self.tempo;

        let read_int = self.read_idx as usize;
        let fraction = self.read_idx - read_int as f32;

        // Linear interpolation
        let s1 = self.buffer[(read_int) % self.buffer.len()];
        let s2 = self.buffer[(read_int + 1) % self.buffer.len()];

        let output = s1 * (1.0 - fraction) + s2 * fraction;

        // Window envelope
        let env_pos = (read_int % self.window) as f32 / self.window as f32;
        let env = (env_pos * core::f32::consts::PI).sin();

        output * env
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

impl Default for TimeStretch {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for WsolaStretcher {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for ElastaneStretch {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
