/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc4d62b08 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/delay.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::hermite_interpolate;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

/// Technical implementation of the DelayEffect structure.
pub struct DelayEffect {
    buffer: [Sample; 4096], // ~92ms at 44.1kHz (Autonomous resolution)
    write_pos: usize,
    delay_samples: f32,
    feedback: f32,
    mix: f32,
}

impl DelayEffect {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            buffer: [0.0; 4096],
            write_pos: 0,
            delay_samples: 1000.0,
            feedback: 0.5,
            mix: 0.3,
        }
    }

    /// Set delay time in milliseconds (Silicon Warp)
    pub fn set_delay_ms(&mut self, delay_ms: f32, sample_rate: f32) {
        let samples = delay_ms * sample_rate / 1000.0;
        self.delay_samples = samples.clamp(1.0, 4094.0);
    }

    /// Technical implementation of the set_feedback logic.
    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 0.999);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Process sample with fractional Hermite interpolation
    pub fn process(&mut self, input: Sample) -> Sample {
        let mask = 4095;

        // Calculate fractional read position
        let read_pos = (self.write_pos as f32 - self.delay_samples + 4096.0) % 4096.0;
        let i0 = read_pos as usize;
        let i1 = (i0 + 1) & mask;
        let i2 = (i1 + 1) & mask;
        let im1 = if i0 > 0 { i0 - 1 } else { mask };
        let frac = read_pos - i0 as f32;

        // Hermite interpolation for absolute spectral purity
        let delayed = hermite_interpolate(
            self.buffer[im1],
            self.buffer[i0],
            self.buffer[i1],
            self.buffer[i2],
            frac,
        );

        // Feedback saturation (Soft clip to prevent runaway)
        let feedback_val = (delayed * self.feedback).clamp(-1.0, 1.0);
        self.buffer[self.write_pos] = input + feedback_val;
        self.write_pos = (self.write_pos + 1) & mask;

        input * (1.0 - self.mix) + delayed * self.mix
    }

    /// Primary real-time signal processing execution block.
    pub fn process_into(&mut self, input: &[Sample], output: &mut [Sample]) {
        for i in 0..input.len().min(output.len()) {
            output[i] = self.process(input[i]);
        }
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.buffer = [0.0; 4096];
    }
}

impl Default for DelayEffect {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
