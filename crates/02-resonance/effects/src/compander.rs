/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x07831e0d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/compander.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::{amplitude_to_db, db_to_amplitude, exp_approx};
use smoothie_core::primitives::Sample;

/// Technical implementation of the Compander structure.
pub struct Compander {
    threshold: f32,
    ratio_comp: f32,
    ratio_expand: f32,
    attack_coeff: f32,
    release_coeff: f32,
    makeup_gain: f32,
    range: f32,
    envelope: f32,
    sample_rate: f32,
}

impl Compander {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut c = Self {
            threshold: -20.0,
            ratio_comp: 3.0,
            ratio_expand: 2.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            makeup_gain: 0.0,
            range: -40.0,
            envelope: -60.0,
            sample_rate,
        };
        c.update_coeffs(0.005, 0.1);
        c
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self, attack: f32, release: f32) {
        self.attack_coeff = 1.0 - exp_approx(-1.0 / (attack * self.sample_rate));
        self.release_coeff = 1.0 - exp_approx(-1.0 / (release * self.sample_rate));
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
    /// Technical implementation of the set_comp_ratio logic.
    pub fn set_comp_ratio(&mut self, ratio: f32) {
        self.ratio_comp = ratio.max(1.0);
    }
    /// Technical implementation of the set_expand_ratio logic.
    pub fn set_expand_ratio(&mut self, ratio: f32) {
        self.ratio_expand = ratio.max(1.0);
    }
    /// Technical implementation of the set_attack logic.
    pub fn set_attack(&mut self, attack: f32) {
        self.attack_coeff = 1.0 - exp_approx(-1.0 / (attack.max(0.0001) * self.sample_rate));
    }
    /// Technical implementation of the set_release logic.
    pub fn set_release(&mut self, release: f32) {
        self.release_coeff = 1.0 - exp_approx(-1.0 / (release.max(0.001) * self.sample_rate));
    }
    /// Technical implementation of the set_makeup_gain logic.
    pub fn set_makeup_gain(&mut self, gain: f32) {
        self.makeup_gain = gain;
    }
    /// Technical implementation of the set_range logic.
    pub fn set_range(&mut self, range: f32) {
        self.range = range.clamp(-60.0, 0.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let abs_input = input.abs();
        let input_db = amplitude_to_db(abs_input);

        // Envelope follower
        let diff = input_db - self.envelope;
        if diff > 0.0 {
            self.envelope += self.attack_coeff * diff;
        } else {
            self.envelope += self.release_coeff * diff;
        }

        // Gain computer - expansion below threshold, compression above
        let mut gain_change = if self.envelope < self.threshold {
            // Expansion
            let excess = self.threshold - self.envelope;
            excess * (1.0 - 1.0 / self.ratio_expand)
        } else {
            // Compression
            let excess = self.envelope - self.threshold;
            excess * (1.0 - 1.0 / self.ratio_comp)
        };

        // Apply range limit
        gain_change = gain_change.max(self.range).min(-self.range);

        let total_gain = db_to_amplitude(gain_change + self.makeup_gain);
        input * total_gain
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        // Detect from peak for true stereo linking
        let peak = left.abs().max(right.abs());
        let peak_db = amplitude_to_db(peak);

        let diff = peak_db - self.envelope;
        if diff > 0.0 {
            self.envelope += self.attack_coeff * diff;
        } else {
            self.envelope += self.release_coeff * diff;
        }

        let mut gain_change = if self.envelope < self.threshold {
            let excess = self.threshold - self.envelope;
            excess * (1.0 - 1.0 / self.ratio_expand)
        } else {
            let excess = self.envelope - self.threshold;
            excess * (1.0 - 1.0 / self.ratio_comp)
        };

        gain_change = gain_change.max(self.range).min(-self.range);

        let total_gain = db_to_amplitude(gain_change + self.makeup_gain);

        (left * total_gain, right * total_gain)
    }

    /// Technical implementation of the get_envelope logic.
    pub fn get_envelope(&self) -> f32 {
        self.envelope
    }
}

/// Technical implementation of the SplitCompander structure.
pub struct SplitCompander {
    above: Compander,
    below: Compander,
}

impl SplitCompander {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            above: Compander::new(sample_rate),
            below: Compander::new(sample_rate),
        }
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold: f32) {
        self.above.set_threshold(threshold);
        self.below.set_threshold(threshold);
    }

    /// Technical implementation of the set_above_ratio logic.
    pub fn set_above_ratio(&mut self, ratio: f32) {
        self.above.set_comp_ratio(ratio);
    }
    /// Technical implementation of the set_below_ratio logic.
    pub fn set_below_ratio(&mut self, ratio: f32) {
        self.below.set_expand_ratio(ratio);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let abs_input = input.abs();
        let input_db = amplitude_to_db(abs_input);

        if input_db > self.above.threshold {
            self.above.process(input)
        } else {
            self.below.process(input)
        }
    }
}

impl Default for Compander {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for SplitCompander {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
