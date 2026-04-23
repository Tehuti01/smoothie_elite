/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf3b674f0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/envelope/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// with independently configurable curve shapes per stage.
/// # Stage Transitions
/// ```text
///                                                        │
///                                                        │
/// ```
/// # Curve Shapes
/// Each time-domain stage supports three curve shapes via `EnvelopeCurve`:
/// - `Linear` — constant-rate ramp.
/// - `Logarithmic` — fast initial movement, slow approach.
/// This allows the attack to feel "punchy" (logarithmic) while the release
/// sounds natural (exponential decay).

/// The active stage of the DAHDSR state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the EnvelopeStage enumeration.
pub enum EnvelopeStage {
    Idle,
    Delay,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
}

/// The curve shape applied to a time-domain envelope stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the EnvelopeCurve enumeration.
pub enum EnvelopeCurve {
    Linear,
    /// Exponential decay/rise: `y = 1 − e^(−x·k)`.
    Exponential,
    /// Logarithmic: fast initial sweep, slow approach.
    Logarithmic,
}

/// Complete DAHDSR parameter set.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the EnvelopeParams structure.
pub struct EnvelopeParams {
    pub delay_ms: f32,
    pub attack_ms: f32,
    pub attack_curve: EnvelopeCurve,
    pub hold_ms: f32,
    pub decay_ms: f32,
    pub decay_curve: EnvelopeCurve,
    pub sustain_level: f32,
    pub release_ms: f32,
    pub release_curve: EnvelopeCurve,
    /// Velocity sensitivity [0.0, 1.0]. 0 = velocity has no effect.
    pub velocity_sensitivity: f32,
}

impl Default for EnvelopeParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            delay_ms: 0.0,
            attack_ms: 5.0,
            attack_curve: EnvelopeCurve::Exponential,
            hold_ms: 0.0,
            decay_ms: 200.0,
            decay_curve: EnvelopeCurve::Exponential,
            sustain_level: 0.7,
            release_ms: 300.0,
            release_curve: EnvelopeCurve::Exponential,
            velocity_sensitivity: 0.5,
        }
    }
}

/// Technical implementation of the Envelope structure.
pub struct Envelope {
    params: EnvelopeParams,
    stage: EnvelopeStage,
    /// Current envelope output level [0.0, 1.0].
    level: f32,
    /// Level at which the current stage began (used for exponential curves).
    stage_start_level: f32,
    /// Elapsed samples within the current stage.
    stage_samples: u64,
    sample_rate: f32,
    velocity: f32,
}

impl Envelope {
    /// Initializes a new instance of the associated type.
    pub fn new(params: EnvelopeParams, sample_rate: f32) -> Self {
        Self {
            params,
            stage: EnvelopeStage::Idle,
            level: 0.0,
            stage_start_level: 0.0,
            stage_samples: 0,
            sample_rate,
            velocity: 1.0,
        }
    }

    /// Trigger a note-on event with the given velocity [0.0, 1.0].
    pub fn note_on(&mut self, velocity: f32) {
        self.velocity = velocity;
        self.stage = if self.params.delay_ms > 0.0 {
            EnvelopeStage::Delay
        } else {
            EnvelopeStage::Attack
        };
        self.stage_start_level = self.level;
        self.stage_samples = 0;
    }

    /// Trigger a note-off event.
    pub fn note_off(&mut self) {
        if self.stage != EnvelopeStage::Idle {
            self.stage = EnvelopeStage::Release;
            self.stage_start_level = self.level;
            self.stage_samples = 0;
        }
    }

    /// Advance one sample and return the current output level.
    #[inline(always)]
    /// Technical implementation of the tick logic.
    pub fn tick(&mut self) -> f32 {
        match self.stage {
            EnvelopeStage::Idle => {
                self.level = 0.0;
            }

            EnvelopeStage::Delay => {
                let duration = ms_to_samples(self.params.delay_ms, self.sample_rate);
                self.stage_samples += 1;
                if self.stage_samples >= duration {
                    self.stage = EnvelopeStage::Attack;
                    self.stage_start_level = self.level;
                    self.stage_samples = 0;
                }
            }

            EnvelopeStage::Attack => {
                let target = self.velocity_scaled_peak();
                let duration = ms_to_samples(self.params.attack_ms.max(0.01), self.sample_rate);
                let t = (self.stage_samples as f32) / (duration as f32);
                self.level =
                    apply_curve(self.stage_start_level, target, t, self.params.attack_curve);
                self.stage_samples += 1;
                if self.stage_samples >= duration {
                    self.level = target;
                    self.stage = if self.params.hold_ms > 0.0 {
                        EnvelopeStage::Hold
                    } else {
                        EnvelopeStage::Decay
                    };
                    self.stage_start_level = self.level;
                    self.stage_samples = 0;
                }
            }

            EnvelopeStage::Hold => {
                let duration = ms_to_samples(self.params.hold_ms, self.sample_rate);
                self.stage_samples += 1;
                if self.stage_samples >= duration {
                    self.stage = EnvelopeStage::Decay;
                    self.stage_start_level = self.level;
                    self.stage_samples = 0;
                }
            }

            EnvelopeStage::Decay => {
                let target = self.params.sustain_level;
                let duration = ms_to_samples(self.params.decay_ms.max(0.01), self.sample_rate);
                let t = (self.stage_samples as f32) / (duration as f32);
                self.level =
                    apply_curve(self.stage_start_level, target, t, self.params.decay_curve);
                self.stage_samples += 1;
                if self.stage_samples >= duration {
                    self.level = target;
                    self.stage = EnvelopeStage::Sustain;
                }
            }

            EnvelopeStage::Sustain => {
                self.level = self.params.sustain_level;
            }

            EnvelopeStage::Release => {
                let duration = ms_to_samples(self.params.release_ms.max(0.01), self.sample_rate);
                let t = (self.stage_samples as f32) / (duration as f32);
                self.level = apply_curve(self.stage_start_level, 0.0, t, self.params.release_curve);
                self.stage_samples += 1;
                if self.stage_samples >= duration || self.level < 1e-5 {
                    self.level = 0.0;
                    self.stage = EnvelopeStage::Idle;
                }
            }
        }

        self.level
    }

    /// Technical implementation of the velocity_scaled_peak logic.
    fn velocity_scaled_peak(&self) -> f32 {
        let vs = self.params.velocity_sensitivity;
        1.0 - vs * (1.0 - self.velocity)
    }

    /// Technical implementation of the is_active logic.
    pub fn is_active(&self) -> bool {
        self.stage != EnvelopeStage::Idle
    }
    /// Technical implementation of the stage logic.
    pub fn stage(&self) -> EnvelopeStage {
        self.stage
    }
    /// Technical implementation of the level logic.
    pub fn level(&self) -> f32 {
        self.level
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.stage = EnvelopeStage::Idle;
        self.level = 0.0;
        self.stage_samples = 0;
    }
}

#[inline(always)]
/// Technical implementation of the ms_to_samples logic.
fn ms_to_samples(ms: f32, sample_rate: f32) -> u64 {
    ((ms / 1000.0) * sample_rate).max(1.0) as u64
}

/// with the given curve shape.
#[inline(always)]
/// Technical implementation of the apply_curve logic.
fn apply_curve(start: f32, target: f32, t: f32, curve: EnvelopeCurve) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t_shaped = match curve {
        EnvelopeCurve::Linear => t,
        EnvelopeCurve::Exponential => {
            // y = 1 − e^(−5t) normalised — fast initial growth, slow approach
            1.0 - smoothie_core::math::exp_approx(-5.0 * t)
        }
        EnvelopeCurve::Logarithmic => {
            // y = ln(1 + 9t) / ln(10) — fast initial movement
            let ln10_inv = 0.434_294_5_f32;
            smoothie_core::math::exp_approx(fast_ln(1.0 + 9.0 * t) * ln10_inv)
        }
    };
    start + (target - start) * t_shaped.clamp(0.0, 1.0)
}

/// Fast natural log approximation
#[inline(always)]
/// Technical implementation of the fast_ln logic.
fn fast_ln(x: f32) -> f32 {
    let n = x.to_bits();
    let exp = ((n >> 23) & 0xFF) as i32 - 127;
    let mantissa = f32::from_bits((n & 0x7FFFFF) | 0x3F800000) - 1.0;
    exp as f32 * core::f32::consts::LN_2 + mantissa * (1.0 - mantissa * 0.5)
}
