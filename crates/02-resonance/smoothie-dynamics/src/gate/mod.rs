/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc432410e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/gate/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// with configurable open/close thresholds (hysteresis), attack, hold,
/// on signals hovering near the gate boundary.
/// # State Machine
/// ```text
///                                                             │
///                                       │
/// ```

/// Gate state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateState {
    Closed,
    Attack,
    Open,
    Hold,
    Release,
}

/// Parameters for the noise suppression and expansion engine.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the GateParams structure.
pub struct GateParams {
    /// Level required to open the gate (dB).
    pub open_threshold_db: f32,
    /// Level required to close the gate (dB), typically lower than open_threshold.
    pub close_threshold_db: f32,
    /// Time taken to reach unity gain when opening (ms).
    pub attack_ms: f32,
    /// Duration to maintain unity gain after signal falls below close_threshold (ms).
    pub hold_ms: f32,
    /// Time taken to reach floor gain when closing (ms).
    pub release_ms: f32,
    /// Minimum gain applied when the gate is fully closed (dB).
    pub floor_db: f32,
    /// Dynamic expansion ratio for signals below threshold (x:1).
    pub ratio: f32,
}

impl Default for GateParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            open_threshold_db: -40.0,
            close_threshold_db: -44.0,
            attack_ms: 0.5,
            hold_ms: 50.0,
            release_ms: 200.0,
            floor_db: -100.0,
            ratio: 1.0,
        }
    }
}

/// Technical implementation of the Gate structure.
pub struct Gate {
    /// Active configuration parameters.
    params: GateParams,
    /// Current internal state of the processor.
    state: GateState,
    /// Current calculated gain value (linear).
    gain: f32,
    /// Floor gain value (linear).
    floor_linear: f32,
    /// Linear activation threshold.
    open_linear: f32,
    /// Linear deactivation threshold.
    close_linear: f32,
    /// Gain increment amount per sample during attack.
    attack_step: f32,
    /// Gain decrement amount per sample during release.
    release_step: f32,
    /// Number of samples to hold after falling below threshold.
    hold_samples: usize,
    /// Current hold counter decrementing towards zero.
    hold_counter: usize,
    /// Internal level detector state.
    detector_state: f32,
    /// Coeffient controlling the detector decay.
    detector_release: f32,
}

impl Gate {
    /// Initializes a new instance of the associated type.
    pub fn new(params: GateParams, sample_rate: f32) -> Self {
        let db_lin = |db: f32| -> f32 { smoothie_core::math::exp_approx(db * 0.1151292546) };
        let ms_step = |ms: f32| -> f32 { 1.0 / ((ms / 1000.0) * sample_rate).max(1.0) };

        Self {
            floor_linear: db_lin(params.floor_db),
            open_linear: db_lin(params.open_threshold_db),
            close_linear: db_lin(params.close_threshold_db),
            attack_step: ms_step(params.attack_ms),
            release_step: ms_step(params.release_ms),
            hold_samples: ((params.hold_ms / 1000.0) * sample_rate) as usize,
            hold_counter: 0,
            detector_state: 0.0,
            detector_release: 1.0 - ms_step(50.0),
            gain: 0.0,
            state: GateState::Closed,
            params,
        }
    }

    /// Process one stereo sample. Returns `(out_l, out_r)`.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let level = (in_l.abs()).max(in_r.abs());
        self.detector_state = self.detector_state.max(level);
        self.detector_state *= self.detector_release;

        let level = self.detector_state;

        self.state = match self.state {
            GateState::Closed => {
                if level > self.open_linear {
                    GateState::Attack
                } else {
                    GateState::Closed
                }
            }
            GateState::Attack => {
                self.gain = (self.gain + self.attack_step).min(1.0);
                if self.gain >= 1.0 {
                    GateState::Open
                } else {
                    GateState::Attack
                }
            }
            GateState::Open => {
                if level < self.close_linear {
                    self.hold_counter = self.hold_samples;
                    GateState::Hold
                } else {
                    GateState::Open
                }
            }
            GateState::Hold => {
                if level > self.open_linear {
                    GateState::Open
                } else if self.hold_counter == 0 {
                    GateState::Release
                } else {
                    self.hold_counter -= 1;
                    GateState::Hold
                }
            }
            GateState::Release => {
                self.gain = (self.gain - self.release_step).max(self.floor_linear);
                if self.gain <= self.floor_linear {
                    GateState::Closed
                } else {
                    GateState::Release
                }
            }
        };

        (in_l * self.gain, in_r * self.gain)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.gain = 0.0;
        self.state = GateState::Closed;
        self.hold_counter = 0;
        self.detector_state = 0.0;
    }
}
