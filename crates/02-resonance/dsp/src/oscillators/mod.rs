/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9906b87b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/dsp/src/oscillators.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::{F_233, STANDARD_PITCH, TAU};
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;

pub mod noise;
pub use noise::*;

#[derive(Clone, Copy, Debug)]
/// Technical implementation of the OscillatorMode enumeration.
pub enum OscillatorMode {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Pulse { duty_cycle: f32 },
    Noise,
}

/// Technical implementation of the Oscillator structure.
pub struct Oscillator {
    pub phase: f32,
    pub frequency: f32,
    pub mode: OscillatorMode,
    pub sample_rate: f32,
}

impl Oscillator {
    /// Initializes a new instance of the associated type.
    pub fn new(frequency: f32, sample_rate: f32, mode: OscillatorMode) -> Self {
        Self {
            phase: 0.0,
            frequency,
            mode,
            sample_rate,
        }
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    /// Technical implementation of the set_mode logic.
    pub fn set_mode(&mut self, mode: OscillatorMode) {
        self.mode = mode;
    }
    /// Resets the internal state of the component.
    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }
    /// Technical implementation of the phase logic.
    pub fn phase(&self) -> f32 {
        self.phase
    }

    #[inline(always)]
    pub fn process(&mut self) -> Sample {
        let dt = self.frequency / self.sample_rate;
        let sample = match self.mode {
            OscillatorMode::Sine => sine_approx(self.phase * TAU),
            OscillatorMode::Triangle => {
                if self.phase < 0.5 {
                    self.phase * 4.0 - 1.0
                } else {
                    3.0 - self.phase * 4.0
                }
            }
            OscillatorMode::Sawtooth => {
                let mut out = self.phase * 2.0 - 1.0;
                out -= self.poly_blep(self.phase, dt);
                out
            }
            OscillatorMode::Square => {
                let mut out = if self.phase < 0.5 { 1.0 } else { -1.0 };
                out += self.poly_blep(self.phase, dt);
                out -= self.poly_blep(self.wrap(self.phase + 0.5), dt);
                out
            }
            OscillatorMode::Pulse { duty_cycle } => {
                let mut out = if self.phase < duty_cycle { 1.0 } else { -1.0 };
                out += self.poly_blep(self.phase, dt);
                out -= self.poly_blep(self.wrap(self.phase + (1.0 - duty_cycle)), dt);
                out
            }
            OscillatorMode::Noise => self.generate_noise(),
        };
        self.phase = self.wrap(self.phase + dt);
        sample
    }

    /// Technical implementation of the wrap logic.
    fn wrap(&self, mut p: f32) -> f32 {
        while p >= 1.0 {
            p -= 1.0;
        }
        while p < 0.0 {
            p += 1.0;
        }
        p
    }

    /// Technical implementation of the poly_blep logic.
    fn poly_blep(&self, t: f32, dt: f32) -> f32 {
        if t < dt {
            let t = t / dt;
            t + t - t * t - 1.0
        } else if t > 1.0 - dt {
            let t = (t - 1.0) / dt;
            t * t + t + t + 1.0
        } else {
            0.0
        }
    }

    /// Technical implementation of the generate_noise logic.
    fn generate_noise(&self) -> Sample {
        let mut x = self.phase.to_bits();
        if x == 0 {
            x = 0xdeadbeef;
        }
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        (x as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

impl Default for Oscillator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(STANDARD_PITCH, 44100.0, OscillatorMode::Sine)
    }
}

/// Technical implementation of the WavetableOscillator structure.
pub struct WavetableOscillator {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    wavetable: [Sample; F_233],
}

impl WavetableOscillator {
    /// Initializes a new instance of the associated type.
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        let mut osc = Self {
            phase: 0.0,
            frequency,
            sample_rate,
            wavetable: [0.0; F_233],
        };
        osc.init_sine();
        osc
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }
    /// Resets the internal state of the component.
    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }

    /// Technical implementation of the init_sine logic.
    fn init_sine(&mut self) {
        for i in 0..F_233 {
            self.wavetable[i] = sine_approx((i as f32 / F_233 as f32) * TAU);
        }
    }

    #[inline(always)]
    pub fn process(&mut self) -> Sample {
        let pos = self.phase * (F_233 as f32);
        let i0 = pos as usize;
        let i1 = (i0 + 1) % F_233;
        let frac = pos - (i0 as f32);

        let sample = self.wavetable[i0] * (1.0 - frac) + self.wavetable[i1] * frac;

        self.phase += self.frequency / self.sample_rate;
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

impl Default for WavetableOscillator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(STANDARD_PITCH, 44100.0)
    }
}
