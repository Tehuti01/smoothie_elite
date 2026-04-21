/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x55c6fa33 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-granular/src/window.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Pre-computed window tables for zero-allocation grain envelope shaping.

/// Technical implementation of the WindowTable structure.
pub struct WindowTable {
    pub data: [f32; 4096],
    pub size: usize,
}

impl WindowTable {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            data: [0.0; 4096],
            size: 4096,
        }
    }

    /// Technical implementation of the generate_hanning logic.
    pub fn generate_hanning(&mut self) {
        let n = self.size;
        for i in 0..n {
            let phase = core::f32::consts::PI * (i as f32) / (n - 1) as f32;
            self.data[i] = phase.sin();
        }
    }

    /// Technical implementation of the generate_hamming logic.
    pub fn generate_hamming(&mut self) {
        let n = self.size;
        for i in 0..n {
            let phase = 2.0 * core::f32::consts::PI * (i as f32) / (n - 1) as f32;
            self.data[i] = 0.54 - 0.46 * phase.cos();
        }
    }

    /// Technical implementation of the generate_blackman logic.
    pub fn generate_blackman(&mut self) {
        let n = self.size;
        for i in 0..n {
            let phase = 2.0 * core::f32::consts::PI * (i as f32) / (n - 1) as f32;
            self.data[i] = 0.42 - 0.5 * phase.cos() + 0.08 * (phase * 2.0).cos();
        }
    }

    /// Technical implementation of the generate_blackman_harris logic.
    pub fn generate_blackman_harris(&mut self) {
        let n = self.size;
        for i in 0..n {
            let phase = 2.0 * core::f32::consts::PI * (i as f32) / (n - 1) as f32;
            self.data[i] = 0.35875 - 0.48829 * phase.cos() + 0.14128 * (phase * 2.0).cos()
                - 0.01168 * (phase * 3.0).cos();
        }
    }

    /// Technical implementation of the generate_gaussian logic.
    pub fn generate_gaussian(&mut self, sigma: f32) {
        let n = self.size;
        let center = (n - 1) as f32 / 2.0;
        for i in 0..n {
            let x = (i as f32 - center) / (sigma * center);
            self.data[i] = (-0.5 * x * x).exp();
        }
    }

    /// Technical implementation of the generate_cosine logic.
    pub fn generate_cosine(&mut self) {
        let n = self.size;
        for i in 0..n {
            let phase = core::f32::consts::PI * (i as f32) / (n - 1) as f32;
            self.data[i] = phase.sin().powi(2);
        }
    }

    /// Technical implementation of the get logic.
    pub fn get(&self, index: usize) -> f32 {
        if index < self.size {
            self.data[index]
        } else {
            0.0
        }
    }

    /// Technical implementation of the interpolate logic.
    pub fn interpolate(&self, position: f32) -> f32 {
        let pos = position * (self.size - 1) as f32;
        let i = pos as usize;
        let frac = pos - i as f32;

        if i + 1 < self.size {
            self.data[i] * (1.0 - frac) + self.data[i + 1] * frac
        } else {
            self.data[i]
        }
    }

    /// Technical implementation of the generate_all logic.
    pub fn generate_all(&mut self) {
        self.generate_hanning();
    }
}

impl Default for WindowTable {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        let mut table = Self::new();
        table.generate_all();
        table
    }
}

/// Technical implementation of the PitchShifter structure.
pub struct PitchShifter {
    pub window_size: usize,
    pub hop_size: usize,
    pub pitch_ratio: f32,
    pub phase: f32,
    pub input_buffer: [f32; 8192],
    pub output_buffer: [f32; 8192],
    pub input_pos: usize,
    pub output_pos: usize,
}

impl PitchShifter {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            window_size: 2048,
            hop_size: 512,
            pitch_ratio: 1.0,
            phase: 0.0,
            input_buffer: [0.0; 8192],
            output_buffer: [0.0; 8192],
            input_pos: 0,
            output_pos: 0,
        }
    }

    /// Technical implementation of the set_pitch_ratio logic.
    pub fn set_pitch_ratio(&mut self, ratio: f32) {
        self.pitch_ratio = ratio;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        for &sample in input.iter().take(self.input_buffer.len() - self.input_pos) {
            self.input_buffer[self.input_pos] = sample;
            self.input_pos += 1;
        }

        while self.input_pos >= self.window_size {
            self.process_window(output);
            self.input_pos -= self.hop_size;
        }
    }

    /// Primary real-time signal processing execution block.
    fn process_window(&mut self, output: &mut [f32]) {
        let ratio = self.pitch_ratio;

        for i in 0..self.window_size {
            let phase_in = i as f32 / self.window_size as f32;
            let phase_out = phase_in * ratio;
            let out_idx = (phase_out * self.window_size as f32) as usize;

            if out_idx < self.window_size {
                let sample = self.input_buffer[i];
                self.output_buffer[out_idx] += sample;
            }
        }

        for i in 0..self.hop_size {
            let idx = self.output_pos + i;
            if idx < self.output_buffer.len() {
                output[i] = self.output_buffer[idx];
                self.output_buffer[idx] = 0.0;
            }
        }

        self.output_pos = (self.output_pos + self.hop_size) & 8191;
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.input_pos = 0;
        self.output_pos = 0;
        self.phase = 0.0;
        self.input_buffer = [0.0; 8192];
        self.output_buffer = [0.0; 8192];
    }
}

impl Default for PitchShifter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
