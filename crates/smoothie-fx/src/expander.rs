//! Expander and noise gate.

use smoothie_math::envelope::EnvelopeFollower;
use smoothie_math::decibels::{linear_to_db, db_to_linear};

/// Downward expander — reduces gain below threshold.
pub struct Expander {
    env:           EnvelopeFollower,
    gain_db:       f32,
    attack_coeff:  f32,
    release_coeff: f32,
    /// Threshold in dBFS.
    pub threshold: f32,
    /// Ratio (e.g. 2.0 = 1:2 expansion). Must be >= 1.
    pub ratio:     f32,
    /// Range: maximum gain reduction in dB.
    pub range_db:  f32,
}

impl Expander {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            env:           EnvelopeFollower::new(1.0, 100.0, sample_rate),
            gain_db:       0.0,
            attack_coeff:  (-1000.0 / (sample_rate * 1.0)).exp(),
            release_coeff: (-1000.0 / (sample_rate * 100.0)).exp(),
            threshold:    -40.0,
            ratio:         2.0,
            range_db:     -60.0,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let env   = self.env.process(x);
        let db    = linear_to_db(env + 1e-30);
        let over  = db - self.threshold;
        let target = if over < 0.0 {
            (over * (self.ratio - 1.0)).max(self.range_db)
        } else {
            0.0
        };

        if target < self.gain_db {
            self.gain_db = self.attack_coeff  * self.gain_db + (1.0 - self.attack_coeff)  * target;
        } else {
            self.gain_db = self.release_coeff * self.gain_db + (1.0 - self.release_coeff) * target;
        }

        x * db_to_linear(self.gain_db)
    }
}

/// Noise gate with hysteresis.
pub struct NoiseGate {
    env:           EnvelopeFollower,
    gain:          f32,
    attack_coeff:  f32,
    release_coeff: f32,
    open:          bool,
    /// Open threshold in dBFS.
    pub threshold_open:  f32,
    /// Close threshold in dBFS (hysteresis; must be <= threshold_open).
    pub threshold_close: f32,
    /// Hold time in samples.
    pub hold_samples: usize,
    hold_count: usize,
}

impl NoiseGate {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            env:            EnvelopeFollower::new(0.5, 50.0, sample_rate),
            gain:           0.0,
            attack_coeff:   (-1000.0 / (sample_rate * 2.0)).exp(),
            release_coeff:  (-1000.0 / (sample_rate * 50.0)).exp(),
            open:           false,
            threshold_open:  -40.0,
            threshold_close: -50.0,
            hold_samples:    (0.05 * sample_rate) as usize,
            hold_count:      0,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let env = self.env.process(x);
        let db  = linear_to_db(env + 1e-30);

        if self.open {
            if db < self.threshold_close {
                if self.hold_count >= self.hold_samples {
                    self.open = false;
                } else {
                    self.hold_count += 1;
                }
            } else {
                self.hold_count = 0;
            }
        } else if db >= self.threshold_open {
            self.open = true;
            self.hold_count = 0;
        }

        let target = if self.open { 1.0 } else { 0.0 };
        if target > self.gain {
            self.gain = self.attack_coeff  * self.gain + (1.0 - self.attack_coeff)  * target;
        } else {
            self.gain = self.release_coeff * self.gain + (1.0 - self.release_coeff) * target;
        }

        x * self.gain
    }

    /// Is the gate currently open?
    pub fn is_open(&self) -> bool { self.open }
}
