//! Low-Frequency Oscillator for modulation.

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LfoShape {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Random,
}

/// Low-Frequency Oscillator for parameter modulation.
///
/// Generates modulation signals at sub-audio frequencies (typically 0.1–20 Hz).
pub struct Lfo {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    shape: LfoShape,
    last_random: f32,
}

impl Lfo {
    /// Create a new LFO.
    pub fn new(frequency: f32, shape: LfoShape, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            frequency: frequency.clamp(0.1, 1000.0),
            sample_rate,
            shape,
            last_random: 0.0,
        }
    }

    /// Get the current LFO output (0.0–1.0).
    pub fn value(&self) -> f32 {
        self.generate(self.phase)
    }

    /// Generate output for a given phase (0.0–1.0).
    fn generate(&self, phase: f32) -> f32 {
        let p = phase % 1.0;
        match self.shape {
            LfoShape::Sine => {
                // Sine from 0.0 to 1.0
                ((2.0 * PI * p).sin() + 1.0) * 0.5
            }
            LfoShape::Triangle => {
                // Triangle wave 0.0–1.0
                if p < 0.5 {
                    p * 2.0
                } else {
                    2.0 - p * 2.0
                }
            }
            LfoShape::Sawtooth => {
                // Sawtooth 0.0–1.0
                p
            }
            LfoShape::Square => {
                // Square wave 0.0–1.0
                if p < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            LfoShape::Random => {
                // Random sample-and-hold
                self.last_random
            }
        }
    }

    /// Advance the LFO by one sample and return output.
    pub fn next_sample(&mut self) -> f32 {
        let phase_inc = self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_inc) % 1.0;

        // Update random on rising edge of square wave
        if self.shape == LfoShape::Random && self.phase < phase_inc {
            self.last_random = random_f32();
        }

        self.generate(self.phase)
    }

    /// Set LFO frequency (Hz).
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq.clamp(0.1, 1000.0);
    }

    /// Set LFO shape.
    pub fn set_shape(&mut self, shape: LfoShape) {
        self.shape = shape;
    }

    /// Set sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }

    /// Reset phase to 0.
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

impl Default for Lfo {
    fn default() -> Self {
        Self::new(5.0, LfoShape::Sine, 44100.0)
    }
}

/// Simple pseudo-random number generator (LCG).
fn random_f32() -> f32 {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = Cell::new(12345);
    }

    SEED.with(|seed| {
        let mut s = seed.get();
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        seed.set(s);
        ((s >> 8) as f32) * (1.0 / (1u32 << 24) as f32)
    })
}
