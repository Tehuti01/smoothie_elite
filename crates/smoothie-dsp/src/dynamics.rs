//! Compressor, limiter, gate, expander — zero allocation.

fn db_to_lin(db: f32) -> f32 { 10.0_f32.powf(db / 20.0) }
fn lin_to_db(lin: f32) -> f32 { 20.0 * lin.max(1e-10).log10() }

/// Feed-forward compressor with peak detection.
pub struct Compressor {
    threshold_lin:  f32,
    ratio:          f32,
    attack_coeff:   f32,
    release_coeff:  f32,
    makeup_gain:    f32,
    envelope:       f32,
}

impl Compressor {
    pub fn new(sample_rate: f32) -> Self {
        let mut c = Self {
            threshold_lin: db_to_lin(-12.0),
            ratio:         4.0,
            attack_coeff:  0.0,
            release_coeff: 0.0,
            makeup_gain:   1.0,
            envelope:      0.0,
        };
        c.set_times(10.0, 100.0, sample_rate);
        c
    }

    pub fn set_threshold_db(&mut self, db: f32) { self.threshold_lin = db_to_lin(db); }
    pub fn set_ratio(&mut self, ratio: f32)      { self.ratio = ratio.max(1.0); }
    pub fn set_makeup_db(&mut self, db: f32)     { self.makeup_gain = db_to_lin(db); }

    pub fn set_times(&mut self, attack_ms: f32, release_ms: f32, sample_rate: f32) {
        self.attack_coeff  = (-1.0 / (attack_ms  * 0.001 * sample_rate)).exp();
        self.release_coeff = (-1.0 / (release_ms * 0.001 * sample_rate)).exp();
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let abs = x.abs();
        // Envelope follower
        let coeff = if abs > self.envelope { self.attack_coeff } else { self.release_coeff };
        self.envelope = abs + coeff * (self.envelope - abs);

        // Gain computation
        let gain = if self.envelope > self.threshold_lin {
            let over_db = lin_to_db(self.envelope / self.threshold_lin);
            db_to_lin(-over_db * (1.0 - 1.0 / self.ratio))
        } else {
            1.0
        };

        x * gain * self.makeup_gain
    }

    #[inline]
    pub fn process_block(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() { *s = self.process(*s); }
    }
}

/// Brick-wall limiter (compressor with ∞:1 ratio + lookahead envelope).
pub struct Limiter {
    ceiling: f32,
    release_coeff: f32,
    envelope: f32,
}

impl Limiter {
    pub fn new(ceiling_db: f32, sample_rate: f32) -> Self {
        Self {
            ceiling: db_to_lin(ceiling_db),
            release_coeff: (-1.0_f32 / (50.0 * 0.001 * sample_rate)).exp(),
            envelope: 0.0,
        }
    }

    pub fn set_ceiling_db(&mut self, db: f32) { self.ceiling = db_to_lin(db); }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let abs = x.abs();
        self.envelope = if abs > self.envelope { abs } else { abs + self.release_coeff * (self.envelope - abs) };
        let gain = if self.envelope > self.ceiling { self.ceiling / self.envelope } else { 1.0 };
        x * gain
    }

    #[inline]
    pub fn process_block(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() { *s = self.process(*s); }
    }
}
