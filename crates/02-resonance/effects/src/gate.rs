/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc76ec7bb | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/gate.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::db_to_amplitude;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

/// Noise gate state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GateState {
    /// Gate is closed (attenuating).
    Closed,
    /// Gate is opening (attack phase).
    Opening,
    /// Gate is fully open (passing signal).
    Open,
    /// Gate is closing (release phase).
    Closing,
}

/// Technical implementation of the Gate structure.
pub struct Gate {
    threshold_linear: f32,
    hysteresis_linear: f32,
    attack_coeff: f32,
    release_coeff: f32,
    hold_samples: usize,
    hold_counter: usize,
    envelope: f32,
    gain: f32,
    state: GateState,
    sample_rate: f32,
}

impl Gate {
    /// Create a new noise gate.
    ///
    /// - `threshold_db`: Open threshold in dB
    /// - `hysteresis_db`: Difference between open and close thresholds
    /// - `attack_ms`: Attack time in ms
    /// - `release_ms`: Release time in ms
    /// - `hold_ms`: Hold time in ms (keeps gate open after signal drops)
    /// - `sample_rate`: Audio sample rate in Hz
    pub fn new(
        threshold_db: f32,
        hysteresis_db: f32,
        attack_ms: f32,
        release_ms: f32,
        hold_ms: f32,
        sample_rate: f32,
    ) -> Self {
        let attack_coeff = 1.0 - (-1.0 / (attack_ms * 0.001 * sample_rate)).exp_gate();
        let release_coeff = 1.0 - (-1.0 / (release_ms * 0.001 * sample_rate)).exp_gate();
        let hold_samples = (hold_ms * 0.001 * sample_rate) as usize;

        Self {
            threshold_linear: db_to_amplitude(threshold_db),
            hysteresis_linear: db_to_amplitude(threshold_db - hysteresis_db),
            attack_coeff,
            release_coeff,
            hold_samples,
            hold_counter: 0,
            envelope: 0.0,
            gain: 0.0,
            state: GateState::Closed,
            sample_rate,
        }
    }

    /// Set threshold in dB.
    pub fn set_threshold(&mut self, db: f32) {
        self.threshold_linear = db_to_amplitude(db);
    }

    /// Process a single sample.
    pub fn process(&mut self, input: Sample) -> Sample {
        let abs_input = input.abs();

        // Envelope follower
        if abs_input > self.envelope {
            self.envelope += self.attack_coeff * (abs_input - self.envelope);
        } else {
            self.envelope += self.release_coeff * (abs_input - self.envelope);
        }

        // State machine
        match self.state {
            GateState::Closed => {
                if self.envelope > self.threshold_linear {
                    self.state = GateState::Opening;
                }
            }
            GateState::Opening => {
                self.gain += self.attack_coeff;
                if self.gain >= 1.0 {
                    self.gain = 1.0;
                    self.state = GateState::Open;
                }
            }
            GateState::Open => {
                if self.envelope < self.hysteresis_linear {
                    self.hold_counter = self.hold_samples;
                    self.state = GateState::Closing;
                }
            }
            GateState::Closing => {
                if self.envelope > self.threshold_linear {
                    self.state = GateState::Open;
                    self.gain = 1.0;
                } else if self.hold_counter > 0 {
                    self.hold_counter -= 1;
                } else {
                    self.gain -= self.release_coeff;
                    if self.gain <= 0.0 {
                        self.gain = 0.0;
                        self.state = GateState::Closed;
                    }
                }
            }
        }

        input * self.gain
    }

    /// Check if the gate is currently open.
    pub fn is_open(&self) -> bool {
        matches!(self.state, GateState::Open | GateState::Opening)
    }

    /// Reset gate state.
    pub fn reset(&mut self) {
        self.envelope = 0.0;
        self.gain = 0.0;
        self.state = GateState::Closed;
        self.hold_counter = 0;
    }
}

impl Default for Gate {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(-40.0, 6.0, 1.0, 50.0, 20.0, 44100.0)
    }
}

/// Trait for exp in no_std.
trait ExpGate {
    /// Technical implementation of the exp_gate logic.
    fn exp_gate(self) -> f32;
}

impl ExpGate for f32 {
    /// Technical implementation of the exp_gate logic.
    fn exp_gate(self) -> f32 {
        smoothie_core::math::exp_approx(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_gate_silence logic.
    fn test_gate_silence() {
        let mut gate = Gate::default();
        let output = gate.process(0.001);
        assert!(output.abs() < 0.01);
    }

    #[test]
    /// Technical implementation of the test_gate_passes_loud_signal logic.
    fn test_gate_passes_loud_signal() {
        let mut gate = Gate::new(-20.0, 6.0, 0.1, 50.0, 20.0, 44100.0);
        // Feed loud signal to open the gate
        for _ in 0..100 {
            gate.process(0.5);
        }
        let output = gate.process(0.5);
        assert!(output.abs() > 0.1);
    }
}
