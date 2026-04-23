/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4f1e301b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/transient/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// signal's transient envelope. Uses the classic "differential envelope"
/// transient designer plugin.
/// # Algorithm
/// Two envelope followers track the signal with different time constants:
/// - `env_fast` — tracks rapid changes (short attack).
///
/// growing, a transient is in progress. If negative, we're in the sustain
/// tail. Apply separate gain curves to each region.

/// Parameters for the transient shaping engine.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the TransientParams structure.
pub struct TransientParams {
    /// Level adjustment for the detected transient attack phase (dB).
    pub attack_boost_db: f32,
    /// Level adjustment for the detected sustain phase (dB).
    pub sustain_boost_db: f32,
    /// Detection threshold sensitivity; higher values react to smaller transients.
    pub sensitivity: f32,
    /// Dry/Wet blend ratio for parallel transient sculpting.
    pub mix: f32,
}

impl Default for TransientParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            attack_boost_db: 6.0,
            sustain_boost_db: -3.0,
            sensitivity: 0.5,
            mix: 1.0,
        }
    }
}

/// Technical implementation of the TransientShaper structure.
pub struct TransientShaper {
    /// Active configuration parameters.
    params: TransientParams,
    /// Fast-tracking envelope state (left channel).
    env_fast_l: f32,
    /// Fast-tracking envelope state (right channel).
    env_fast_r: f32,
    /// Slow-tracking envelope state (left channel).
    env_slow_l: f32,
    /// Slow-tracking envelope state (right channel).
    env_slow_r: f32,
    /// Fast attack time constant coefficient.
    fast_coeff: f32,
    /// Slow attack time constant coefficient.
    slow_coeff: f32,
    /// Linear gain factor applied during attack phases.
    attack_gain: f32,
    /// Linear gain factor applied during sustain phases.
    sustain_gain: f32,
}

impl TransientShaper {
    /// Initializes a new instance of the associated type.
    pub fn new(params: TransientParams, sample_rate: f32) -> Self {
        let db_to_lin = |db: f32| smoothie_core::math::exp_approx(db * 0.115_129_255);

        // Fast envelope: ~1ms attack
        let fast_ms = 1.0 * (1.0 - params.sensitivity * 0.8);
        // Slow envelope: ~30ms attack, tracks program level
        let slow_ms = 30.0 + params.sensitivity * 100.0;

        let coeff = |ms: f32| -> f32 { 1.0 - 1.0 / ((ms / 1000.0) * sample_rate + 1.0) };

        Self {
            params,
            env_fast_l: 0.0,
            env_fast_r: 0.0,
            env_slow_l: 0.0,
            env_slow_r: 0.0,
            fast_coeff: coeff(fast_ms),
            slow_coeff: coeff(slow_ms),
            attack_gain: db_to_lin(params.attack_boost_db),
            sustain_gain: db_to_lin(params.sustain_boost_db),
        }
    }

    /// Updates a framework parameter value.
    pub fn set_params(&mut self, params: TransientParams, sample_rate: f32) {
        let db_to_lin = |db: f32| smoothie_core::math::exp_approx(db * 0.115_129_255);
        self.attack_gain = db_to_lin(params.attack_boost_db);
        self.sustain_gain = db_to_lin(params.sustain_boost_db);
        self.params = params;
    }

    /// Process one stereo sample. Returns shaped `(out_l, out_r)`.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let (gl, gr) = (
            self.compute_gain(in_l, true),
            self.compute_gain(in_r, false),
        );
        let mix = self.params.mix;
        (
            in_l * (1.0 - mix) + in_l * gl * mix,
            in_r * (1.0 - mix) + in_r * gr * mix,
        )
    }

    #[inline(always)]
    /// Technical implementation of the compute_gain logic.
    fn compute_gain(&mut self, sample: f32, is_left: bool) -> f32 {
        let rect = sample.abs();
        let fc = self.fast_coeff;
        let sc = self.slow_coeff;

        let (ef, es) = if is_left {
            (self.env_fast_l, self.env_slow_l)
        } else {
            (self.env_fast_r, self.env_slow_r)
        };

        let new_ef = ef * fc + rect * (1.0 - fc);
        let new_es = es * sc + rect * (1.0 - sc);

        if is_left {
            self.env_fast_l = new_ef;
            self.env_slow_l = new_es;
        } else {
            self.env_fast_r = new_ef;
            self.env_slow_r = new_es;
        }

        // Differential: positive = transient region, negative = sustain region
        let diff = new_ef - new_es;
        if diff > 0.0 {
            self.attack_gain
        } else {
            self.sustain_gain
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.env_fast_l = 0.0;
        self.env_fast_r = 0.0;
        self.env_slow_l = 0.0;
        self.env_slow_r = 0.0;
    }
}
