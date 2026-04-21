/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2cb0fa1c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-granular/src/grain.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Zero-allocation grain creation with configurable envelope shaping.

#[derive(Clone, Copy)]
/// Technical implementation of the Grain structure.
pub struct Grain {
    pub source_offset: usize,
    pub length_samples: usize,
    pub pitch_ratio: f32,
    pub amplitude: f32,
    pub pan: f32,
    pub window_type: WindowType,
    pub position: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Technical implementation of the WindowType enumeration.
pub enum WindowType {
    Hanning,
    Hamming,
    Blackman,
    BlackmanHarris,
    Gaussian,
    Rectangular,
    Cosine,
}

impl Grain {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            source_offset: 0,
            length_samples: 1024,
            pitch_ratio: 1.0,
            amplitude: 1.0,
            pan: 0.0,
            window_type: WindowType::Hanning,
            position: 0.0,
        }
    }

    /// Technical implementation of the with_source_offset logic.
    pub fn with_source_offset(mut self, offset: usize) -> Self {
        self.source_offset = offset;
        self
    }

    /// Technical implementation of the with_length logic.
    pub fn with_length(mut self, length: usize) -> Self {
        self.length_samples = length;
        self
    }

    /// Technical implementation of the with_pitch logic.
    pub fn with_pitch(mut self, ratio: f32) -> Self {
        self.pitch_ratio = ratio;
        self
    }

    /// Technical implementation of the with_amplitude logic.
    pub fn with_amplitude(mut self, amp: f32) -> Self {
        self.amplitude = amp;
        self
    }

    /// Technical implementation of the with_pan logic.
    pub fn with_pan(mut self, pan: f32) -> Self {
        self.pan = pan;
        self
    }

    /// Technical implementation of the with_window logic.
    pub fn with_window(mut self, window: WindowType) -> Self {
        self.window_type = window;
        self
    }
}

impl Default for Grain {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the GrainGenerator structure.
pub struct GrainGenerator {
    pub grain_size: usize,
    pub overlap: usize,
    pub pitch_variance: f32,
    pub amplitude_variance: f32,
    pub density: f32,
    pub random_seed: u32,
}

impl GrainGenerator {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            grain_size: 1024,
            overlap: 4,
            pitch_variance: 0.0,
            amplitude_variance: 0.0,
            density: 1.0,
            random_seed: 12345,
        }
    }

    /// Technical implementation of the generate_grain logic.
    pub fn generate_grain(&mut self, position: f32) -> Grain {
        let offset = (position * self.grain_size as f32) as usize;
        let pitch = 1.0 + (self.pitch_variance * self.linear_prng());
        let amp = 1.0 - (self.amplitude_variance * self.linear_prng() * 0.3);

        Grain {
            source_offset: offset,
            length_samples: self.grain_size,
            pitch_ratio: pitch,
            amplitude: amp,
            pan: 0.0,
            window_type: WindowType::Hanning,
            position,
        }
    }

    /// Technical implementation of the next_position logic.
    pub fn next_position(&self, current: f32, source_length: usize) -> f32 {
        let step = self.grain_size as f32 / self.overlap as f32;
        let next = current + step;
        if next as usize >= source_length {
            0.0
        } else {
            next
        }
    }

    /// Technical implementation of the linear_prng logic.
    fn linear_prng(&mut self) -> f32 {
        self.random_seed = self
            .random_seed
            .wrapping_mul(1103515245)
            .wrapping_add(12345);
        ((self.random_seed >> 16) & 0x7FFF) as f32 / 32768.0
    }
}

impl Default for GrainGenerator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the GrainScheduler structure.
pub struct GrainScheduler {
    generator: GrainGenerator,
    current_position: f32,
    buffer_size: usize,
    hop_size: usize,
    grain_buffer: [Grain; 16],
    grain_count: usize,
}

impl GrainScheduler {
    /// Initializes a new instance of the associated type.
    pub const fn new(buffer_size: usize) -> Self {
        let grain_size = 1024;
        let hop_size = grain_size / 4;

        Self {
            generator: GrainGenerator::new(),
            current_position: 0.0,
            buffer_size,
            hop_size,
            grain_buffer: [Grain::new(); 16],
            grain_count: 0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, source: &[f32], output: &mut [f32]) {
        self.grain_count = 0;

        while (self.current_position as usize) + self.generator.grain_size < source.len()
            && self.grain_count < 16
        {
            let grain = self.generator.generate_grain(self.current_position);
            self.grain_buffer[self.grain_count] = grain;
            self.grain_count += 1;
            self.current_position = self
                .generator
                .next_position(self.current_position, source.len());
        }

        for i in 0..output.len() {
            output[i] = 0.0;
        }

        for g in 0..self.grain_count {
            let grain = &self.grain_buffer[g];
            self.write_grain(source, output, grain);
        }
    }

    /// Technical implementation of the write_grain logic.
    fn write_grain(&self, source: &[f32], output: &mut [f32], grain: &Grain) {
        let length = grain.length_samples.min(output.len());

        for i in 0..length {
            let pos = i as f32 / length as f32;
            let window = compute_window(pos, grain.window_type);
            let source_idx = grain.source_offset + i;

            if source_idx < source.len() {
                let sample = source[source_idx] * grain.amplitude * window;
                output[i] += sample;
            }
        }
    }
}

#[inline(always)]
/// Technical implementation of the compute_window logic.
pub fn compute_window(position: f32, window_type: WindowType) -> f32 {
    let phase = position * core::f32::consts::PI;
    match window_type {
        WindowType::Hanning => (phase).sin(),
        WindowType::Hamming => 0.54 - 0.46 * (phase * 2.0).cos(),
        WindowType::Blackman => 0.42 - 0.5 * (phase * 2.0).cos() + 0.08 * (phase * 4.0).cos(),
        WindowType::BlackmanHarris => {
            0.35875 - 0.48829 * (phase * 2.0).cos() + 0.14128 * (phase * 4.0).cos()
                - 0.01168 * (phase * 6.0).cos()
        }
        WindowType::Gaussian => ((position - 0.5) * 8.0).exp(),
        WindowType::Rectangular => 1.0,
        WindowType::Cosine => phase.sin().powi(2),
    }
}
