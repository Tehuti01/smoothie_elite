/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x67c2b023 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/engine/stiff_string.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;
use smoothie_dsp::prelude::DelayLine;

/// Enforces Engineering Phase 21: Acoustic modeling stability.
#[repr(align(64))]
/// Technical implementation of the StiffString structure.
pub struct StiffString {
    /// Left-traveling wave delay line
    left_wave: DelayLine,
    /// Right-traveling wave delay line
    right_wave: DelayLine,
    /// All-pass filter for fractional delay
    all_pass_state: f64,
    all_pass_coeff: f64,
    /// String fundamental frequency
    frequency: f64,
    /// Feedback gain (loss modeling)
    feedback: f64,
    /// Fixed delay length for this string
    delay_samples: usize,
}

impl StiffString {
    /// Create a new string during the Initialization Phase.
    pub fn new(freq: f64, sample_rate: f64) -> Self {
        let period = sample_rate / freq;
        let delay_len = (period / 2.0) as usize;
        let frac = (period / 2.0) - delay_len as f64;

        // [Engineering Phase 20]: All-pass coefficient for fractional delay
        let ap_coeff = (1.0 - frac) / (1.0 + frac);

        Self {
            left_wave: DelayLine::new(delay_len.max(64)),
            right_wave: DelayLine::new(delay_len.max(64)),
            all_pass_state: 0.0,
            all_pass_coeff: ap_coeff,
            frequency: freq,
            feedback: 0.999,
            delay_samples: delay_len.max(64),
        }
    }

    /// [Engineering Phase 21]: Waveguide scattering step.
    #[seraphic_specification(L0, A0, PHI)]
    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, excitation: f64) -> f64 {
        // 1. Read traveling waves
        let mut left = self.left_wave.read(self.delay_samples) as f64;
        let right = self.right_wave.read(self.delay_samples) as f64;

        // 2. Fractional Delay (All-pass)
        let ap_out = self.all_pass_coeff * left + self.all_pass_state;
        self.all_pass_state = left - self.all_pass_coeff * ap_out;
        left = ap_out;

        // 3. Loss & Dispersion (Feedback)
        let feedback_sample = (left + excitation) * -self.feedback;

        // 4. Update Waves
        self.right_wave.write(feedback_sample as f32);
        self.left_wave.write(right as f32);

        // 5. Output displacement (sum of waves)
        (left + right) * 0.5
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.left_wave.clear();
        self.right_wave.clear();
        self.all_pass_state = 0.0;
    }
}
