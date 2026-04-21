/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8cd9e713 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/convolution/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Provides impulse response-based reverb through FFT convolution.

/// Technical implementation of the ConvolutionReverb structure.
pub struct ConvolutionReverb {
    impulse_response: alloc::vec::Vec<f32>,
    buffer: alloc::vec::Vec<f32>,
    position: usize,
    wet_dry_mix: f32,
    sample_rate: f32,
}

impl ConvolutionReverb {
    /// Initializes a new instance of the associated type.
    pub fn new(max_length_samples: usize, sample_rate: f32) -> Self {
        let buffer_size = max_length_samples * 2;

        Self {
            impulse_response: alloc::vec::Vec::with_capacity(max_length_samples),
            buffer: alloc::vec::Vec::with_capacity(buffer_size),
            position: 0,
            wet_dry_mix: 0.5,
            sample_rate,
        }
    }

    /// Technical implementation of the load_impulse logic.
    pub fn load_impulse(&mut self, impulse: &[f32]) {
        self.impulse_response.clear();
        self.impulse_response.extend_from_slice(impulse);
        self.buffer.resize(self.impulse_response.len() * 2, 0.0);
        self.position = 0;
    }

    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.wet_dry_mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        if self.impulse_response.is_empty() {
            return input;
        }

        let ir_len = self.impulse_response.len();

        self.buffer[self.position] = input;
        self.position = (self.position + 1) % ir_len;

        let mut output = 0.0;

        for (i, &ir_val) in self.impulse_response.iter().enumerate() {
            let buf_idx = (self.position + i) % ir_len;
            output += self.buffer[buf_idx] * ir_val;
        }

        let dry = input * (1.0 - self.wet_dry_mix);
        let wet = output * self.wet_dry_mix;

        dry + wet
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: f32) -> (f32, f32) {
        if self.impulse_response.is_empty() {
            return (input, input);
        }

        let ir_len = self.impulse_response.len();

        self.buffer[self.position] = input;
        self.position = (self.position + 1) % ir_len;

        let mut out_l = 0.0;
        let mut out_r = 0.0;

        for (i, &ir_val) in self.impulse_response.iter().enumerate() {
            let buf_idx = (self.position + i) % ir_len;
            let buf_val = self.buffer[buf_idx];
            out_l += buf_val * ir_val;
            out_r += buf_val * ir_val * if i % 2 == 0 { 0.9 } else { 1.1 };
        }

        let dry = input * (1.0 - self.wet_dry_mix);
        (
            dry + out_l * self.wet_dry_mix,
            dry + out_r * self.wet_dry_mix,
        )
    }

    /// Technical implementation of the latency_samples logic.
    pub fn latency_samples(&self) -> usize {
        self.impulse_response.len()
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.position = 0;
    }
}

/// Technical implementation of the SparseConvolution structure.
pub struct SparseConvolution {
    indices: alloc::vec::Vec<usize>,
    values: alloc::vec::Vec<f32>,
    delay_buffer: alloc::vec::Vec<f32>,
    write_pos: usize,
    mix: f32,
}

impl SparseConvolution {
    /// Initializes a new instance of the associated type.
    pub fn new(max_delay_samples: usize) -> Self {
        Self {
            indices: alloc::vec::Vec::with_capacity(64),
            values: alloc::vec::Vec::with_capacity(64),
            delay_buffer: alloc::vec::Vec::with_capacity(max_delay_samples),
            write_pos: 0,
            mix: 0.5,
        }
    }

    /// Performs vector addition logic.
    pub fn add_tap(&mut self, delay_samples: usize, gain_db: f32) {
        let gain = 10.0_f32.powf(gain_db / 20.0);
        self.indices.push(delay_samples);
        self.values.push(gain);
    }

    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        let total = self.values.len();
        if total == 0 {
            return input;
        }

        if self.delay_buffer.len() < total {
            self.delay_buffer.resize(total * 2, 0.0);
        }

        self.delay_buffer[self.write_pos % self.delay_buffer.len()] = input;
        self.write_pos += 1;

        let mut output = 0.0;

        for (i, &idx) in self.indices.iter().enumerate() {
            let delay_idx = (self.write_pos - idx - 1) % self.delay_buffer.len();
            output += self.delay_buffer[delay_idx] * self.values[i];
        }

        input * (1.0 - self.mix) + output * self.mix
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.delay_buffer.fill(0.0);
        self.write_pos = 0;
    }
}
