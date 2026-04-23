/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8d409496 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/super_osc.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::constants::{STANDARD_PITCH, TAU};
use smoothie_core::math::{sine_approx, PowiApprox};
use smoothie_core::primitives::Sample;

/// Super oscillator waveform selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SuperWaveform enumeration.
pub enum SuperWaveform {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

/// Super oscillator configuration.
#[derive(Clone, Copy, Debug)]
#[repr(align(64))]
/// Technical implementation of the SuperOscConfig structure.
pub struct SuperOscConfig {
    pub main_wave: SuperWaveform,
    pub sub_wave: SuperWaveform,
    pub sub_octave: i8, // -2 to +2
    pub sub_level: f32, // 0.0 to 1.0
    pub mix: f32,       // 0.0 = sub only, 1.0 = main only
}

impl Default for SuperOscConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            main_wave: SuperWaveform::Sawtooth,
            sub_wave: SuperWaveform::Square,
            sub_octave: -1,
            sub_level: 0.5,
            mix: 0.7,
        }
    }
}

/// Super oscillator (main + sub).
#[repr(align(64))]
/// Technical implementation of the SuperOsc structure.
pub struct SuperOsc {
    pub main_phase: f32,
    pub sub_phase: f32,
    pub main_phase_inc: f32,
    config: SuperOscConfig,
}

impl SuperOsc {
    /// Initializes a new instance of the associated type.
    pub fn new(config: SuperOscConfig, sample_rate: f32) -> Self {
        Self {
            main_phase: 0.0,
            sub_phase: 0.0,
            main_phase_inc: STANDARD_PITCH / sample_rate,
            config,
        }
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32, sample_rate: f32) {
        self.main_phase_inc = freq / sample_rate;
    }

    #[inline(always)]
    /// Technical implementation of the waveform logic.
    fn waveform(&self, phase: f32, wave: SuperWaveform) -> f32 {
        let _p = phase * TAU;
        match wave {
            SuperWaveform::Sine => sine_approx(phase),
            SuperWaveform::Triangle => {
                if phase < 0.5 {
                    4.0 * phase - 1.0
                } else {
                    3.0 - 4.0 * phase
                }
            }
            SuperWaveform::Sawtooth => 2.0 * phase - 1.0,
            SuperWaveform::Square => {
                if phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    }

    /// Technical implementation of the next logic.
    pub fn process(&mut self) -> Sample {
        // Main oscillator
        let main = self.waveform(self.main_phase, self.config.main_wave);
        self.main_phase += self.main_phase_inc;
        if self.main_phase >= 1.0 {
            self.main_phase -= 1.0;
        }

        // Sub oscillator (octave offset)
        let sub_freq_mult = 2.0_f32.powi_approx(-self.config.sub_octave as i32);
        let sub_inc = self.main_phase_inc * sub_freq_mult;
        let sub = self.waveform(self.sub_phase, self.config.sub_wave);
        self.sub_phase += sub_inc;
        if self.sub_phase >= 1.0 {
            self.sub_phase -= 1.0;
        }

        // Mix main and sub
        let mixed = main * self.config.mix + sub * self.config.sub_level * (1.0 - self.config.mix);
        mixed * 0.5
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.main_phase = 0.0;
        self.sub_phase = 0.0;
    }
}
