//! Band-limited wavetable oscillator.

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaveShape { Sine, Triangle, Sawtooth, Square, Pulse(f32) }

/// A simple phase-accumulator oscillator. Not band-limited — use for LFO/mod.
/// For audio-rate anti-aliased synthesis, use the BLEP variant (coming soon).
pub struct Oscillator {
    phase:       f32,
    phase_inc:   f32,
    shape:       WaveShape,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(shape: WaveShape, sample_rate: f32) -> Self {
        Self { phase: 0.0, phase_inc: 0.0, shape, sample_rate }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        self.phase_inc = hz / self.sample_rate;
    }

    pub fn set_shape(&mut self, shape: WaveShape) { self.shape = shape; }

    /// Advance one sample and return output in [-1, 1].
    #[inline]
    pub fn next_sample(&mut self) -> f32 {
        let p = self.phase;
        self.phase = (self.phase + self.phase_inc) % 1.0;
        match self.shape {
            WaveShape::Sine     => (2.0 * PI * p).sin(),
            WaveShape::Triangle => if p < 0.5 { 4.0 * p - 1.0 } else { 3.0 - 4.0 * p },
            WaveShape::Sawtooth => 2.0 * p - 1.0,
            WaveShape::Square   => if p < 0.5 { 1.0 } else { -1.0 },
            WaveShape::Pulse(w) => if p < w   { 1.0 } else { -1.0 },
        }
    }

    pub fn reset(&mut self) { self.phase = 0.0; }
}
