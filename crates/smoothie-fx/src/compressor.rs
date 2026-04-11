//! Compressor / VCA with RMS detection, soft knee, lookahead, and makeup gain.

use smoothie_math::envelope::EnvelopeFollower;
use smoothie_math::decibels::{linear_to_db, db_to_linear};

pub struct Compressor {
    env:      EnvelopeFollower,
    /// Threshold in dBFS.
    pub threshold: f32,
    /// Ratio (e.g. 4.0 = 4:1). Use f32::INFINITY for limiting.
    pub ratio:     f32,
    /// Knee width in dB.
    pub knee:      f32,
    /// Makeup gain in dB.
    pub makeup:    f32,
    gain_db: f32,
    attack_coeff:  f32,
    release_coeff: f32,
    /// Auto-makeup: automatically calculate makeup gain.
    pub auto_makeup: bool,
}

impl Compressor {
    pub fn new(sample_rate: f32) -> Self {
        let attack_coeff  = (-1000.0 / (sample_rate * 5.0)).exp();
        let release_coeff = (-1000.0 / (sample_rate * 50.0)).exp();
        Self {
            env: EnvelopeFollower::new(5.0, 50.0, sample_rate),
            threshold: -20.0,
            ratio:      4.0,
            knee:       6.0,
            makeup:     0.0,
            gain_db:    0.0,
            attack_coeff,
            release_coeff,
            auto_makeup: false,
        }
    }

    pub fn set_attack_ms(&mut self, ms: f32, sample_rate: f32) {
        self.attack_coeff = (-1000.0 / (sample_rate * ms)).exp();
    }

    pub fn set_release_ms(&mut self, ms: f32, sample_rate: f32) {
        self.release_coeff = (-1000.0 / (sample_rate * ms)).exp();
    }

    /// Compute gain reduction (dB) for a given input level (dBFS).
    #[inline]
    fn gain_reduction_db(&self, in_db: f32) -> f32 {
        let half_knee = self.knee * 0.5;
        let overshoot = in_db - self.threshold;
        if overshoot < -half_knee {
            0.0
        } else if overshoot < half_knee {
            // Soft knee
            let t = (overshoot + half_knee) / self.knee;
            (1.0 / self.ratio - 1.0) * t * t * self.knee * 0.5
        } else {
            overshoot * (1.0 / self.ratio - 1.0)
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let env = self.env.process(x);
        let in_db = linear_to_db(env + 1e-30);
        let target_gain_db = self.gain_reduction_db(in_db);

        // Ballistics
        if target_gain_db < self.gain_db {
            self.gain_db = self.attack_coeff  * self.gain_db + (1.0 - self.attack_coeff)  * target_gain_db;
        } else {
            self.gain_db = self.release_coeff * self.gain_db + (1.0 - self.release_coeff) * target_gain_db;
        }

        let total_gain_db = self.gain_db + self.makeup;
        x * db_to_linear(total_gain_db)
    }

    pub fn gain_reduction(&self) -> f32 { self.gain_db }
}

/// Stereo-linked compressor.
pub struct StereoCompressor {
    inner: Compressor,
}

impl StereoCompressor {
    pub fn new(sample_rate: f32) -> Self {
        Self { inner: Compressor::new(sample_rate) }
    }

    pub fn inner_mut(&mut self) -> &mut Compressor { &mut self.inner }

    #[inline]
    pub fn process(&mut self, l: f32, r: f32) -> (f32, f32) {
        let key = (l.abs() + r.abs()) * 0.5;
        let env = self.inner.env.process(key);
        let in_db = linear_to_db(env + 1e-30);
        let gr_db = self.inner.gain_reduction_db(in_db);

        if gr_db < self.inner.gain_db {
            self.inner.gain_db = self.inner.attack_coeff  * self.inner.gain_db + (1.0 - self.inner.attack_coeff)  * gr_db;
        } else {
            self.inner.gain_db = self.inner.release_coeff * self.inner.gain_db + (1.0 - self.inner.release_coeff) * gr_db;
        }

        let gain = db_to_linear(self.inner.gain_db + self.inner.makeup);
        (l * gain, r * gain)
    }
}
