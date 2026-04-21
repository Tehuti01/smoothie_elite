/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc46b35df | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/limiter.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::db_to_amplitude;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

///
/// release to apply gain reduction when the signal exceeds the threshold.
/// Technical implementation of the Limiter structure.
pub struct Limiter {
    threshold: f32,
    threshold_linear: f32,
    release_coeff: f32,
    envelope: f32,
    sample_rate: f32,
}

impl Limiter {
    /// Create a new limiter.
    ///
    /// - `threshold_db`: Maximum output level in dB (typically 0.0 or -0.1)
    /// - `release_ms`: Release time in milliseconds
    /// - `sample_rate`: Audio sample rate in Hz
    pub fn new(threshold_db: f32, release_ms: f32, sample_rate: f32) -> Self {
        let threshold_linear = db_to_amplitude(threshold_db);
        let release_coeff = (-1.0 / (release_ms * 0.001 * sample_rate)).exp_approx();
        Self {
            threshold: threshold_db,
            threshold_linear,
            release_coeff,
            envelope: 0.0,
            sample_rate,
        }
    }

    /// Set threshold in dB.
    pub fn set_threshold(&mut self, db: f32) {
        self.threshold = db;
        self.threshold_linear = db_to_amplitude(db);
    }

    /// Set release time in milliseconds.
    pub fn set_release(&mut self, ms: f32) {
        self.release_coeff = (-1.0 / (ms * 0.001 * self.sample_rate)).exp_approx();
    }

    /// Process a single sample.
    pub fn process(&mut self, input: Sample) -> Sample {
        let abs_input = input.abs();

        // Instant attack, smooth release envelope follower
        if abs_input > self.envelope {
            self.envelope = abs_input;
        } else {
            self.envelope = abs_input + self.release_coeff * (self.envelope - abs_input);
        }

        // Calculate gain reduction
        if self.envelope > self.threshold_linear {
            let gain = self.threshold_linear / self.envelope;
            input * gain
        } else {
            input
        }
    }

    /// Process a stereo pair.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        let abs_max = left.abs().max(right.abs());

        if abs_max > self.envelope {
            self.envelope = abs_max;
        } else {
            self.envelope = abs_max + self.release_coeff * (self.envelope - abs_max);
        }

        if self.envelope > self.threshold_linear {
            let gain = self.threshold_linear / self.envelope;
            (left * gain, right * gain)
        } else {
            (left, right)
        }
    }

    /// Get the current gain reduction in dB.
    pub fn gain_reduction_db(&self) -> f32 {
        if self.envelope > self.threshold_linear {
            smoothie_core::math::amplitude_to_db(self.threshold_linear / self.envelope)
        } else {
            0.0
        }
    }

    /// Reset the limiter state.
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}

impl Default for Limiter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0.0, 100.0, 44100.0)
    }
}

/// Trait extension for f32 to provide exp approximation in no_std.
trait ExpApprox {
    /// Technical implementation of the exp_approx logic.
    fn exp_approx(self) -> f32;
}

impl ExpApprox for f32 {
    /// Technical implementation of the exp_approx logic.
    fn exp_approx(self) -> f32 {
        smoothie_core::math::exp_approx(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_limiter_below_threshold logic.
    fn test_limiter_below_threshold() {
        let mut lim = Limiter::new(0.0, 100.0, 44100.0);
        let output = lim.process(0.5);
        assert!((output - 0.5).abs() < 0.01);
    }

    #[test]
    /// Technical implementation of the test_limiter_above_threshold logic.
    fn test_limiter_above_threshold() {
        let mut lim = Limiter::new(-6.0, 100.0, 44100.0);
        let output = lim.process(1.0);
        assert!(output.abs() < 1.0);
    }
}
