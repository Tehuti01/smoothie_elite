/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x750e0abc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/pitch_shift.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::{vec, vec::Vec};
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

#[repr(align(64))]
/// Technical implementation of the PitchShift structure.
pub struct PitchShift {
    pitch_ratio: f32,
    sample_rate: f32,
    window_size: usize,
    overlap: usize,
    phase: f32,
    input_buffer: Vec<Sample>,
    output_buffer: Vec<Sample>,
    read_pos: f32,
    write_pos: usize,
    analysis_hann: Vec<f32>,
    synthesis_hann: Vec<f32>,
}

impl PitchShift {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let window_size = 2048;
        let overlap = 4;

        let mut hann = vec![0.0; window_size];
        for i in 0..window_size {
            hann[i] = (core::f32::consts::PI * i as f32 / (window_size - 1) as f32)
                .sin()
                .powi(2);
        }

        Self {
            pitch_ratio: 1.0,
            sample_rate,
            window_size,
            overlap,
            phase: 0.0,
            input_buffer: vec![0.0; sample_rate as usize],
            output_buffer: vec![0.0; sample_rate as usize],
            read_pos: 0.0,
            write_pos: 0,
            analysis_hann: hann.clone(),
            synthesis_hann: hann,
        }
    }

    /// Technical implementation of the set_pitch logic.
    pub fn set_pitch(&mut self, semitones: f32) {
        self.pitch_ratio = 2.0_f32.powf(semitones / 12.0);
    }

    /// Technical implementation of the set_pitch_ratio logic.
    pub fn set_pitch_ratio(&mut self, ratio: f32) {
        self.pitch_ratio = ratio.clamp(0.25, 4.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        // Write input to buffer
        self.input_buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.input_buffer.len();

        // Calculate output position based on pitch ratio
        self.phase += self.pitch_ratio;

        if self.phase >= 1.0 {
            self.phase -= 1.0;

            // Process window at current position
            let window_sample = self.process_window();

            // Write to output buffer
            let out_idx = (self.read_pos as usize) % self.output_buffer.len();
            self.output_buffer[out_idx] = window_sample;
            self.read_pos = (self.read_pos + 1.0) % self.output_buffer.len() as f32;
        }

        // Read from output buffer
        let out_idx = (self.read_pos as usize) % self.output_buffer.len();
        let output = self.output_buffer[out_idx];

        // Fade in/out to avoid clicks
        let fade_len = 128;
        let fade_idx = self.read_pos as usize % self.output_buffer.len();

        let fade = if fade_idx < fade_len {
            fade_idx as f32 / fade_len as f32
        } else if fade_idx > self.output_buffer.len() - fade_len {
            (self.output_buffer.len() - fade_idx) as f32 / fade_len as f32
        } else {
            1.0
        };

        output * fade
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_window(&mut self) -> Sample {
        let start = (self.read_pos as usize).saturating_sub(self.window_size / 2);
        let mut sum = 0.0;

        for i in 0..self.window_size.min(self.input_buffer.len() - start) {
            let idx = (start + i) % self.input_buffer.len();
            sum = sum + self.input_buffer[idx] * self.analysis_hann[i] * self.synthesis_hann[i];
        }

        sum / (self.window_size as f32 * 0.5)
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// Simple pitch shifter using granular synthesis
#[repr(align(64))]
/// Technical implementation of the GranularPitchShift structure.
pub struct GranularPitchShift {
    pitch: f32,
    grain_size: usize,
    grain_spacing: usize,
    grains: Vec<Grain>,
    buffer: Vec<Sample>,
    buf_write: usize,
    buf_read: usize,
    sample_rate: f32,
}

struct Grain {
    position: usize,
    length: usize,
    env: [f32; 2],
}

impl GranularPitchShift {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            pitch: 1.0,
            grain_size: 512,
            grain_spacing: 256,
            grains: Vec::new(),
            buffer: vec![0.0; sample_rate as usize],
            buf_write: 0,
            buf_read: 0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_pitch logic.
    pub fn set_pitch(&mut self, semitones: f32) {
        self.pitch = 2.0_f32.powf(semitones / 12.0).clamp(0.5, 2.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        // Write input to circular buffer
        self.buffer[self.buf_write] = input;
        self.buf_write = (self.buf_write + 1) % self.buffer.len();

        // Advance read position by pitch ratio
        let advance = self.grain_spacing as f32 * self.pitch;
        self.buf_read = (self.buf_read as f32 + advance) as usize % self.buffer.len();

        // Simple crossfade output
        let idx1 = self.buf_read;
        let idx2 = (self.buf_read + self.grain_size / 2) % self.buffer.len();

        let s1 = self.buffer[idx1];
        let s2 = self.buffer[idx2];

        let env = ((self.buf_read % self.grain_size) as f32 / self.grain_size as f32
            * core::f32::consts::PI)
            .sin();

        s1 * env + s2 * (1.0 - env)
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// Phase vocoder pitch shifter
#[repr(align(64))]
/// Technical implementation of the PhaseVocoderPitchShift structure.
pub struct PhaseVocoderPitchShift {
    fft_size: usize,
    hop_size: usize,
    pitch_ratio: f32,
    phase_accum: f32,
    fft_buffer: Vec<Sample>,
    window: Vec<f32>,
    output_buffer: Vec<Sample>,
    write_idx: usize,
    read_idx: usize,
    sample_rate: f32,
}

impl PhaseVocoderPitchShift {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let fft_size = 2048;
        let hop_size = fft_size / 4;

        let mut window = vec![0.0; fft_size];
        for i in 0..fft_size {
            window[i] =
                0.5 * (1.0 - (2.0 * core::f32::consts::PI * i as f32 / fft_size as f32).cos());
        }

        Self {
            fft_size,
            hop_size,
            pitch_ratio: 1.0,
            phase_accum: 0.0,
            fft_buffer: vec![0.0; fft_size],
            window,
            output_buffer: vec![0.0; sample_rate as usize],
            write_idx: 0,
            read_idx: 0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_pitch logic.
    pub fn set_pitch(&mut self, semitones: f32) {
        self.pitch_ratio = 2.0_f32.powf(semitones / 12.0).clamp(0.25, 4.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample) -> Sample {
        // Simplified phase vocoder - just resample
        let output = if self.phase_accum >= 1.0 {
            self.phase_accum -= 1.0;

            let pos = self.read_idx % self.fft_buffer.len();
            let win_sample = self.fft_buffer[pos] * self.window[pos % self.window.len()];

            self.read_idx = (self.read_idx + (self.hop_size as f32 * self.pitch_ratio) as usize)
                % self.fft_buffer.len();

            Some(win_sample)
        } else {
            None
        };

        // Store input in analysis buffer
        self.fft_buffer[self.write_idx] = input;
        self.write_idx = (self.write_idx + 1) % self.fft_buffer.len();

        self.phase_accum += 1.0;

        output.unwrap_or(0.0)
    }
}

impl Default for PitchShift {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for GranularPitchShift {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for PhaseVocoderPitchShift {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
