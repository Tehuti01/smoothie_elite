//! Envelope detectors and shapers used for dynamics processing.

/// Attack/release envelope follower (peak detector).
pub struct EnvelopeFollower {
    pub attack_coeff:  f32,
    pub release_coeff: f32,
    level: f32,
}

impl EnvelopeFollower {
    /// Create from attack/release times (ms) and sample rate.
    pub fn new(attack_ms: f32, release_ms: f32, sample_rate: f32) -> Self {
        Self {
            attack_coeff:  Self::ms_to_coeff(attack_ms,  sample_rate),
            release_coeff: Self::ms_to_coeff(release_ms, sample_rate),
            level: 0.0,
        }
    }

    fn ms_to_coeff(ms: f32, sr: f32) -> f32 {
        (-1000.0 / (sr * ms)).exp()
    }

    /// Process one sample, returns envelope level.
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let abs = x.abs();
        let coeff = if abs > self.level { self.attack_coeff } else { self.release_coeff };
        self.level = coeff * self.level + (1.0 - coeff) * abs;
        self.level
    }

    /// Current level without updating.
    #[inline]
    pub fn level(&self) -> f32 { self.level }

    /// Reset to zero.
    pub fn reset(&mut self) { self.level = 0.0; }
}

/// RMS envelope follower with a sliding window (exponential averaging).
pub struct RmsFollower {
    coeff: f32,
    mean_sq: f32,
}

impl RmsFollower {
    pub fn new(window_ms: f32, sample_rate: f32) -> Self {
        let coeff = (-1000.0 / (sample_rate * window_ms)).exp();
        Self { coeff, mean_sq: 0.0 }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.mean_sq = self.coeff * self.mean_sq + (1.0 - self.coeff) * x * x;
        self.mean_sq.sqrt()
    }

    pub fn reset(&mut self) { self.mean_sq = 0.0; }
}

/// Ballistics: VU meter (300ms integration, -20 VU = 0 dBFS nominal).
pub struct VuMeter {
    follower: RmsFollower,
}

impl VuMeter {
    pub fn new(sample_rate: f32) -> Self {
        Self { follower: RmsFollower::new(300.0, sample_rate) }
    }

    /// Returns VU level in dBVU (0 VU = 0 dBFS).
    pub fn process(&mut self, x: f32) -> f32 {
        let rms = self.follower.process(x);
        if rms <= 0.0 { return -120.0; }
        20.0 * rms.log10()
    }
}
