/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x51b0e364 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/synthesis/modal/higher_order.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// 
/// and complex decay envelopes.

use smoothie_core::primitives::Sample;
use smoothie_core::constants::TAU;
use smoothie_core::math::{sine_approx, exp_approx};
use alloc::vec::Vec;
use alloc::vec;

#[repr(align(64))]
/// Technical implementation of the CoupledModes structure.
pub struct CoupledModes {
    pub frequency: f32,
    pub decay: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub coupling: f32,
    pub modulation_index: f32,
    pub modulation_freq: f32,
    pub mod_phase: f32,
}

impl CoupledModes {
    /// Initializes a new instance of the associated type.
    pub fn new(freq: f32, decay: f32, amp: f32) -> Self {
        Self {
            frequency: freq,
            decay,
            amplitude: amp,
            phase: 0.0,
            coupling: 0.0,
            modulation_index: 0.0,
            modulation_freq: 0.0,
            mod_phase: 0.0,
        }
    }

    /// Technical implementation of the with_coupling logic.
    pub fn with_coupling(&mut self, freq2: f32, coupling: f32) {
        self.coupling = coupling;
        self.modulation_freq = freq2;
    }

    /// Technical implementation of the with_fm logic.
    pub fn with_fm(&mut self, mod_freq: f32, index: f32) {
        self.modulation_freq = mod_freq;
        self.modulation_index = index;
    }

    /// Technical implementation of the strike logic.
    pub fn strike(&mut self, energy: f32) {
        self.amplitude = energy;
        self.phase = 0.0;
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample_rate: f32) -> Sample {
        if self.amplitude.abs() < 1e-8 {
            return 0.0;
        }

        let mod = if self.modulation_freq > 0.0 {
            self.modulation_index * sine_approx(self.mod_phase)
        } else {
            0.0
        };
        self.mod_phase += self.modulation_freq / sample_rate;

        let freq = self.frequency * (1.0 + mod);
        let out = sine_approx(self.phase) * self.amplitude;

        self.phase += freq / sample_rate;
        self.amplitude *= self.decay;

        if self.coupling > 0.0 {
            self.frequency += self.coupling * 0.01;
        }

        out
    }
}

#[repr(align(64))]
/// Technical implementation of the ModalBank structure.
pub struct ModalBank {
    modes: Vec<CoupledModes>,
    sample_rate: f32,
    output: f32,
}

impl ModalBank {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, max_modes: usize) -> Self {
        Self {
            modes: Vec::with_capacity(max_modes),
            sample_rate,
            output: 0.0,
        }
    }

    /// Performs vector addition logic.
    pub fn add_mode(&mut self, freq: f32, decay: f32, amp: f32) {
        self.modes.push(CoupledModes::new(freq, decay, amp));
    }

    /// Technical implementation of the strike logic.
    pub fn strike(&mut self, energy: f32) {
        for mode in self.modes.iter_mut() {
            mode.strike(energy);
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> Sample {
        let sr = self.sample_rate;
        let mut sum = 0.0;
        for mode in self.modes.iter_mut() {
            sum += mode.process(sr);
        }
        self.output = sum;
        sum
    }

    /// Technical implementation of the get_output logic.
    pub fn get_output(&self) -> Sample {
        self.output
    }
}

#[repr(align(64))]
/// Technical implementation of the DecayEnvelope structure.
pub struct DecayEnvelope {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub level: f32,
    pub state: EnvelopeState,
}

#[derive(Clone, Copy)]
/// Technical implementation of the EnvelopeState enumeration.
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl DecayEnvelope {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            attack: 0.001,
            decay: 0.5,
            sustain: 0.7,
            release: 1.0,
            level: 0.0,
            state: EnvelopeState::Idle,
        }
    }

    /// Technical implementation of the trigger logic.
    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
    }

    /// Technical implementation of the release logic.
    pub fn release(&mut self) {
        self.state = EnvelopeState::Release;
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample_rate: f32) -> f32 {
        match self.state {
            EnvelopeState::Idle => 0.0,
            EnvelopeState::Attack => {
                self.level += self.attack * sample_rate;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.state = EnvelopeState::Decay;
                }
                self.level
            }
            EnvelopeState::Decay => {
                self.level = self.level * (1.0 - self.decay / sample_rate);
                if (self.level - self.sustain).abs() < 0.01 {
                    self.state = EnvelopeState::Sustain;
                }
                self.level
            }
            EnvelopeState::Sustain => self.sustain,
            EnvelopeState::Release => {
                self.level *= (1.0 - self.release / sample_rate);
                if self.level < 0.001 {
                    self.state = EnvelopeState::Idle;
                    self.level = 0.0;
                }
                self.level
            }
        }
    }
}

impl Default for DecayEnvelope {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
