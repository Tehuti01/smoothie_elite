//! Auto-wah — envelope-following bandpass sweep.

use smoothie_math::envelope::EnvelopeFollower;

pub struct AutoWah {
    env:    EnvelopeFollower,
    /// Bandpass resonant filter state (2nd-order SVF).
    lp:     f32,
    bp:     f32,
    hp:     f32,
    /// Frequency range min/max (Hz).
    pub freq_lo:   f32,
    pub freq_hi:   f32,
    /// Resonance [0.1, 0.99].
    pub resonance: f32,
    /// Envelope sensitivity (drive of the follower).
    pub sensitivity: f32,
    /// Wet/dry [0, 1].
    pub mix: f32,
    sample_rate: f32,
}

impl AutoWah {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            env: EnvelopeFollower::new(5.0, 200.0, sample_rate),
            lp: 0.0, bp: 0.0, hp: 0.0,
            freq_lo: 300.0,
            freq_hi: 3000.0,
            resonance: 0.7,
            sensitivity: 4.0,
            mix: 1.0,
            sample_rate,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // Envelope detection
        let env_level = self.env.process(x * self.sensitivity).min(1.0);

        // Map envelope to frequency
        let freq = self.freq_lo + env_level * (self.freq_hi - self.freq_lo);

        // SVF (state variable filter) coefficients
        let f = 2.0 * (std::f32::consts::PI * freq / self.sample_rate).sin();
        let q = 1.0 - self.resonance;

        // SVF process
        self.lp += f * self.bp;
        self.hp = x - self.lp - q * self.bp;
        self.bp += f * self.hp;

        x * (1.0 - self.mix) + self.bp * self.mix
    }

    pub fn reset(&mut self) {
        self.lp = 0.0;
        self.bp = 0.0;
        self.hp = 0.0;
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
