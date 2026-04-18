//! Transient shaper — attack/sustain control via envelope differentiation.

use smoothie_math::envelope::EnvelopeFollower;

pub struct TransientShaper {
    fast_env: EnvelopeFollower,
    slow_env: EnvelopeFollower,
    /// Attack gain (positive = boost attack, negative = reduce).
    pub attack_gain:  f32,
    /// Sustain gain (positive = boost sustain, negative = reduce).
    pub sustain_gain: f32,
    /// Output gain (linear).
    pub output_gain:  f32,
}

impl TransientShaper {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            fast_env: EnvelopeFollower::new(1.0, 10.0, sample_rate),
            slow_env: EnvelopeFollower::new(20.0, 200.0, sample_rate),
            attack_gain:  1.0,
            sustain_gain: 1.0,
            output_gain:  1.0,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let fast = self.fast_env.process(x);
        let slow = self.slow_env.process(x);

        // Transient = fast - slow (the onset spike)
        let transient = (fast - slow).max(0.0);
        // Sustain portion = slow body
        let sustain   = slow;

        // Shape the gain
        let gain = 1.0
            + transient * (self.attack_gain  - 1.0)
            + sustain   * (self.sustain_gain - 1.0);

        x * gain * self.output_gain
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
