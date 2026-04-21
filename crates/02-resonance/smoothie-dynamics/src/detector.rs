/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2719acdf | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/detector.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// the side-chain level detector. Supports peak, RMS, and true-RMS modes,
/// all processed sample-by-sample without any memory allocation.

/// The detection mode used to compute the input level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the DetectionMode enumeration.
pub enum DetectionMode {
    /// Tracks instantaneous absolute peak values with zero lookahead.
    Peak,
    /// Tracks Root-Mean-Square energy using a leaky integrator.
    Rms,
    /// Weighted combination of peak and RMS to mimic human psychoacoustic response.
    Hybrid {
        /// Weighting factor for peak detection (0-100%).
        peak_weight_pct: u8,
    },
}

/// Technical implementation of the LevelDetector structure.
pub struct LevelDetector {
    /// The current operational detection mode.
    mode: DetectionMode,
    /// Internal filter state for linear level tracking.
    state: f32,
    /// Internal filter state for squared level tracking (RMS).
    state_sq: f32,
    /// Attack time constant coefficient.
    attack_coeff: f32,
    /// Release time constant coefficient.
    release_coeff: f32,
}

impl LevelDetector {
    /// Construct a level detector.
    ///
    /// - `attack_ms`  — time constant for the rising edge (0.01 ms – 500 ms)
    /// - `release_ms` — time constant for the falling edge (1 ms – 4000 ms)
    /// - `sample_rate` — session sample rate in Hz
    pub fn new(mode: DetectionMode, attack_ms: f32, release_ms: f32, sample_rate: f32) -> Self {
        let tau_to_coeff = |ms: f32| {
            // RC-equivalent time constant coefficient
            // coeff = 1 − e^(−1 / (ms / 1000 · Fs))
            // Approximated cheaply: for reasonable ms values, 1 − 1/(1 + τ·Fs)
            let tau_samples = (ms / 1000.0) * sample_rate;
            if tau_samples < 0.5 {
                1.0
            } else {
                1.0 - 1.0 / (tau_samples + 1.0)
            }
        };

        Self {
            mode,
            state: 0.0,
            state_sq: 0.0,
            attack_coeff: tau_to_coeff(attack_ms),
            release_coeff: tau_to_coeff(release_ms),
        }
    }

    /// Process one sample and return the current level estimate (linear, ≥ 0).
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, sample: f32) -> f32 {
        let rect = sample.abs();

        match self.mode {
            DetectionMode::Peak => {
                let coeff = if rect > self.state {
                    self.attack_coeff
                } else {
                    self.release_coeff
                };
                self.state = self.state * coeff + rect * (1.0 - coeff);
                self.state
            }

            DetectionMode::Rms => {
                let sq = sample * sample;
                let coeff = if sq > self.state_sq {
                    self.attack_coeff
                } else {
                    self.release_coeff
                };
                self.state_sq = self.state_sq * coeff + sq * (1.0 - coeff);
                fast_sqrt(self.state_sq)
            }

            DetectionMode::Hybrid { peak_weight_pct } => {
                let p = peak_weight_pct as f32 / 100.0;
                let coeff = if rect > self.state {
                    self.attack_coeff
                } else {
                    self.release_coeff
                };
                self.state = self.state * coeff + rect * (1.0 - coeff);

                let sq = sample * sample;
                let coefr = if sq > self.state_sq {
                    self.attack_coeff
                } else {
                    self.release_coeff
                };
                self.state_sq = self.state_sq * coefr + sq * (1.0 - coefr);

                let rms = fast_sqrt(self.state_sq);
                self.state * p + rms * (1.0 - p)
            }
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.state = 0.0;
        self.state_sq = 0.0;
    }
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    let y = y * (1.5 - xhalf * y * y);
    x * y
}
