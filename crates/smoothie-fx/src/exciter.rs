//! Harmonic exciter — generates upper harmonics to add brightness/air.

pub struct HarmonicExciter {
    /// High-pass cutoff coefficient (one-pole).
    hp_state: f32,
    hp_coeff: f32,
    /// Distortion amount [0, 1].
    pub drive: f32,
    /// Blend of generated harmonics [0, 1].
    pub blend: f32,
    /// Low-pass on generated harmonics (anti-aliasing coefficient).
    lp_state: f32,
    lp_coeff: f32,
}

impl HarmonicExciter {
    pub fn new(sample_rate: f32) -> Self {
        let cutoff = 3000.0_f32;
        let hp_coeff = (-2.0 * std::f32::consts::PI * cutoff / sample_rate).exp();
        let lp_coeff = (-2.0 * std::f32::consts::PI * 12000.0 / sample_rate).exp();
        Self {
            hp_state: 0.0,
            hp_coeff,
            drive: 0.5,
            blend: 0.3,
            lp_state: 0.0,
            lp_coeff,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // High-pass to isolate HF content
        self.hp_state += (1.0 - self.hp_coeff) * (x - self.hp_state);
        let hf = x - self.hp_state;

        // Soft-clip to generate harmonics
        let driven = hf * (1.0 + self.drive * 10.0);
        let clipped = driven.tanh();

        // Low-pass to smooth the aliasing
        self.lp_state += (1.0 - self.lp_coeff) * (clipped - self.lp_state);

        // Mix harmonics back in
        x + self.lp_state * self.blend
    }
}

/// Aural exciter with Chebyshev polynomial harmonic generation.
pub struct AuralExciter {
    hp_state: f32,
    lp_state: f32,
    hp_coeff: f32,
    lp_coeff: f32,
    /// Amount of 2nd harmonic.
    pub h2: f32,
    /// Amount of 3rd harmonic.
    pub h3: f32,
    pub blend: f32,
}

impl AuralExciter {
    pub fn new(sample_rate: f32) -> Self {
        let hp = (-2.0 * std::f32::consts::PI * 2000.0 / sample_rate).exp();
        let lp = (-2.0 * std::f32::consts::PI * 15000.0 / sample_rate).exp();
        Self {
            hp_state: 0.0, lp_state: 0.0,
            hp_coeff: hp, lp_coeff: lp,
            h2: 0.3, h3: 0.15, blend: 0.2,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.hp_state += (1.0 - self.hp_coeff) * (x - self.hp_state);
        let hf = x - self.hp_state;

        // Chebyshev 2nd and 3rd harmonics
        let harm2 = 2.0 * hf * hf - 1.0;
        let harm3 = 4.0 * hf * hf * hf - 3.0 * hf;
        let harmonics = hf * self.h2 * harm2 + hf * self.h3 * harm3;

        self.lp_state += (1.0 - self.lp_coeff) * (harmonics - self.lp_state);

        x + self.lp_state * self.blend
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
