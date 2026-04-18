//! Granular synthesis engine.

const MAX_GRAINS: usize = 32;
const MAX_GRAIN_BUFFER: usize = 96000; // 2 seconds @ 48kHz

/// A single grain.
#[derive(Clone)]
pub struct Grain {
    active: bool,
    position: f32,
    duration_samples: u32,
    elapsed_samples: u32,
    pitch: f32,
    pitch_mod: f32,
    envelope_shape: GrainEnvelope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrainEnvelope {
    Hann,
    Gaussian,
    Triangle,
}

impl Grain {
    fn new() -> Self {
        Self {
            active: false,
            position: 0.0,
            duration_samples: 0,
            elapsed_samples: 0,
            pitch: 1.0,
            pitch_mod: 0.0,
            envelope_shape: GrainEnvelope::Hann,
        }
    }

    fn trigger(&mut self, duration: u32, pitch: f32, shape: GrainEnvelope) {
        self.active = true;
        self.duration_samples = duration;
        self.elapsed_samples = 0;
        self.pitch = pitch;
        self.envelope_shape = shape;
    }

    fn get_envelope(&self) -> f32 {
        let progress = self.elapsed_samples as f32 / self.duration_samples.max(1) as f32;
        match self.envelope_shape {
            GrainEnvelope::Hann => {
                // Hann window
                (1.0 - (2.0 * std::f32::consts::PI * progress).cos()) * 0.5
            }
            GrainEnvelope::Gaussian => {
                // Gaussian with σ=0.2
                let x = (progress - 0.5) * 5.0;
                (-x * x / 2.0).exp()
            }
            GrainEnvelope::Triangle => {
                // Triangle window
                if progress < 0.5 {
                    progress * 2.0
                } else {
                    (1.0 - progress) * 2.0
                }
            }
        }
    }

    fn advance(&mut self) -> bool {
        self.elapsed_samples += 1;
        if self.elapsed_samples >= self.duration_samples {
            self.active = false;
            false
        } else {
            true
        }
    }
}

/// Granular synthesis engine.
pub struct GranularEngine {
    grains: [Grain; MAX_GRAINS],
    buffer: Vec<f32>,
    buffer_pos: usize,
    grain_duration: u32,
    grain_density: f32, // Grains per second
    grain_pitch: f32,
    grain_shape: GrainEnvelope,
    sample_rate: f32,
}

impl GranularEngine {
    /// Create a new granular engine.
    pub fn new(sample_rate: f32) -> Self {
        let grains = std::array::from_fn(|_| Grain::new());

        Self {
            grains,
            buffer: vec![0.0; MAX_GRAIN_BUFFER],
            buffer_pos: 0,
            grain_duration: (sample_rate * 0.1) as u32, // 100ms default
            grain_density: 10.0, // 10 grains per second
            grain_pitch: 1.0,
            grain_shape: GrainEnvelope::Hann,
            sample_rate,
        }
    }

    /// Feed samples into the grain buffer.
    pub fn feed_sample(&mut self, sample: f32) {
        self.buffer[self.buffer_pos] = sample;
        self.buffer_pos = (self.buffer_pos + 1) % MAX_GRAIN_BUFFER;
    }

    /// Process one sample of granular synthesis.
    pub fn process(&mut self) -> f32 {
        let mut output = 0.0;
        let active_grain_count = self.grains.iter().filter(|g| g.active).count();

        // Check if we should spawn a new grain
        let grains_needed = (self.grain_density * self.grain_duration as f32 / self.sample_rate).ceil() as usize;
        if active_grain_count < grains_needed.min(MAX_GRAINS) {
            // Find an inactive grain
            for grain in self.grains.iter_mut() {
                if !grain.active {
                    grain.trigger(self.grain_duration, self.grain_pitch, self.grain_shape);
                    break;
                }
            }
        }

        // Process active grains
        for grain in self.grains.iter_mut() {
            if grain.active {
                let read_pos = (self.buffer_pos + (grain.position * MAX_GRAIN_BUFFER as f32) as usize) % MAX_GRAIN_BUFFER;
                let sample = self.buffer[read_pos];
                let envelope = grain.get_envelope();
                output += sample * envelope;

                grain.position += grain.pitch / self.sample_rate / self.grain_duration as f32;
                if grain.position > 1.0 {
                    grain.position -= 1.0;
                }

                grain.advance();
            }
        }

        // Normalize by grain count
        let count = self.grains.iter().filter(|g| g.active).count().max(1);
        output / count as f32
    }

    /// Set grain duration in milliseconds.
    pub fn set_grain_duration(&mut self, ms: f32) {
        self.grain_duration = ((ms / 1000.0) * self.sample_rate) as u32;
    }

    /// Set grain density (grains per second).
    pub fn set_grain_density(&mut self, density: f32) {
        self.grain_density = density.max(1.0).min(100.0);
    }

    /// Set grain pitch transposition ratio.
    pub fn set_grain_pitch(&mut self, ratio: f32) {
        self.grain_pitch = ratio;
    }

    /// Set grain envelope shape.
    pub fn set_grain_shape(&mut self, shape: GrainEnvelope) {
        self.grain_shape = shape;
    }

    /// Set sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }

    /// Get current sample rate.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Clear all grains and buffer.
    pub fn reset(&mut self) {
        for grain in self.grains.iter_mut() {
            grain.active = false;
        }
        self.buffer.fill(0.0);
        self.buffer_pos = 0;
    }
}

impl Default for GranularEngine {
    fn default() -> Self {
        Self::new(44100.0)
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
