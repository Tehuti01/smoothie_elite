//! ADSR and other envelope generators.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeStage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// Attack-Decay-Sustain-Release envelope generator.
///
/// Classic ADSR envelope with exponential curves.
pub struct Envelope {
    stage: EnvelopeStage,
    value: f32,

    // Timings (seconds)
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,

    // Calculated per sample rate
    attack_coeff: f32,
    decay_coeff: f32,
    release_coeff: f32,

    sample_rate: f32,
}

impl Envelope {
    /// Create a new ADSR envelope.
    ///
    /// All times in seconds. Sustain level is 0.0–1.0.
    pub fn new(
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        sample_rate: f32,
    ) -> Self {
        let mut env = Self {
            stage: EnvelopeStage::Idle,
            value: 0.0,
            attack_time: attack.max(0.001),
            decay_time: decay.max(0.001),
            sustain_level: sustain.clamp(0.0, 1.0),
            release_time: release.max(0.001),
            attack_coeff: 0.0,
            decay_coeff: 0.0,
            release_coeff: 0.0,
            sample_rate,
        };
        env.recalc_coeffs();
        env
    }

    /// Recalculate coefficients when sample rate or timings change.
    fn recalc_coeffs(&mut self) {
        // Exponential envelope coefficients
        // target: value * (1 - coeff) + target * coeff
        self.attack_coeff = (-1.0 / (self.attack_time * self.sample_rate)).exp();
        self.decay_coeff = (-1.0 / (self.decay_time * self.sample_rate)).exp();
        self.release_coeff = (-1.0 / (self.release_time * self.sample_rate)).exp();
    }

    /// Start the envelope (enters attack stage).
    pub fn trigger(&mut self) {
        self.stage = EnvelopeStage::Attack;
        self.value = 0.0;
    }

    /// Release the envelope (enters release stage).
    pub fn release(&mut self) {
        self.stage = EnvelopeStage::Release;
    }

    /// Get current envelope output.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Get current stage.
    pub fn stage(&self) -> EnvelopeStage {
        self.stage
    }

    /// Is the envelope idle (fully released)?
    pub fn is_idle(&self) -> bool {
        self.stage == EnvelopeStage::Idle
    }

    /// Advance the envelope by one sample.
    pub fn next_sample(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Idle => {
                self.value = 0.0;
            }
            EnvelopeStage::Attack => {
                // Exponential attack toward 1.0
                self.value = self.attack_coeff * self.value + (1.0 - self.attack_coeff) * 1.0;
                if self.value > 0.99 {
                    self.stage = EnvelopeStage::Decay;
                }
            }
            EnvelopeStage::Decay => {
                // Exponential decay toward sustain level
                self.value = self.decay_coeff * self.value
                    + (1.0 - self.decay_coeff) * self.sustain_level;
                if (self.value - self.sustain_level).abs() < 0.001 {
                    self.stage = EnvelopeStage::Sustain;
                    self.value = self.sustain_level;
                }
            }
            EnvelopeStage::Sustain => {
                self.value = self.sustain_level;
            }
            EnvelopeStage::Release => {
                // Exponential release toward 0.0
                self.value = self.release_coeff * self.value;
                if self.value < 0.001 {
                    self.stage = EnvelopeStage::Idle;
                    self.value = 0.0;
                }
            }
        }
        self.value
    }

    /// Set attack time (seconds).
    pub fn set_attack(&mut self, seconds: f32) {
        self.attack_time = seconds.max(0.001);
        self.recalc_coeffs();
    }

    /// Set decay time (seconds).
    pub fn set_decay(&mut self, seconds: f32) {
        self.decay_time = seconds.max(0.001);
        self.recalc_coeffs();
    }

    /// Set sustain level (0.0–1.0).
    pub fn set_sustain(&mut self, level: f32) {
        self.sustain_level = level.clamp(0.0, 1.0);
    }

    /// Set release time (seconds).
    pub fn set_release(&mut self, seconds: f32) {
        self.release_time = seconds.max(0.001);
        self.recalc_coeffs();
    }

    /// Set sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.recalc_coeffs();
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new(0.01, 0.1, 0.7, 0.3, 44100.0)
    }
}
