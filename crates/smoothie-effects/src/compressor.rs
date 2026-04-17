//! Dynamic range compressor/limiter.

use crate::EffectProcessor;

/// Compressor/Limiter processor.
///
/// Reduces dynamic range when signal exceeds threshold.
pub struct CompressorProcessor {
    threshold: f32,
    ratio: f32,
    attack_ms: f32,
    release_ms: f32,
    makeup_gain: f32,
    sample_rate: f32,

    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
}

impl CompressorProcessor {
    /// Create a new compressor.
    ///
    /// - `threshold`: dB (-60 to 0)
    /// - `ratio`: compression ratio (1:1 to ∞:1)
    /// - `attack_ms`: attack time
    /// - `release_ms`: release time
    pub fn new(
        threshold: f32,
        ratio: f32,
        attack_ms: f32,
        release_ms: f32,
        sample_rate: f32,
    ) -> Self {
        let mut comp = Self {
            threshold: threshold.clamp(-60.0, 0.0),
            ratio: ratio.max(1.0),
            attack_ms: attack_ms.max(0.1),
            release_ms: release_ms.max(1.0),
            makeup_gain: 1.0,
            sample_rate,
            envelope: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
        };
        comp.recalc_coeffs();
        comp
    }

    fn recalc_coeffs(&mut self) {
        // Exponential envelope followers
        self.attack_coeff = (-1.0 / (self.attack_ms / 1000.0 * self.sample_rate)).exp();
        self.release_coeff = (-1.0 / (self.release_ms / 1000.0 * self.sample_rate)).exp();

        // Calculate makeup gain
        let threshold_linear = self.db_to_linear(self.threshold);
        self.makeup_gain = 1.0 / ((threshold_linear / self.ratio).max(0.001));
    }

    fn db_to_linear(&self, db: f32) -> f32 {
        10.0_f32.powf(db / 20.0)
    }

    fn linear_to_db(&self, linear: f32) -> f32 {
        20.0 * linear.max(0.00001).log10()
    }

    /// Set threshold in dB.
    pub fn set_threshold(&mut self, db: f32) {
        self.threshold = db.clamp(-60.0, 0.0);
        self.recalc_coeffs();
    }

    /// Set compression ratio.
    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.max(1.0);
        self.recalc_coeffs();
    }

    /// Set attack time in milliseconds.
    pub fn set_attack(&mut self, ms: f32) {
        self.attack_ms = ms.max(0.1);
        self.recalc_coeffs();
    }

    /// Set release time in milliseconds.
    pub fn set_release(&mut self, ms: f32) {
        self.release_ms = ms.max(1.0);
        self.recalc_coeffs();
    }
}

impl EffectProcessor for CompressorProcessor {
    fn process(&mut self, input: f32) -> f32 {
        let input_level = input.abs();

        // Envelope follower (attack/release)
        if input_level > self.envelope {
            self.envelope = self.attack_coeff * self.envelope + (1.0 - self.attack_coeff) * input_level;
        } else {
            self.envelope = self.release_coeff * self.envelope + (1.0 - self.release_coeff) * input_level;
        }

        // Convert to dB
        let level_db = self.linear_to_db(self.envelope);

        // Calculate gain reduction
        let gain_reduction = if level_db > self.threshold {
            let excess = level_db - self.threshold;
            let compressed = excess / self.ratio;
            self.db_to_linear(-excess + compressed)
        } else {
            1.0
        };

        input * gain_reduction * self.makeup_gain
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.recalc_coeffs();
    }
}

impl Default for CompressorProcessor {
    fn default() -> Self {
        Self::new(-20.0, 4.0, 5.0, 50.0, 44100.0)
    }
}
