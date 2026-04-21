/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x925afc49 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/exciter.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

/// Technical implementation of the Exciter structure.
pub struct Exciter {
    harmonic_amount: f32,
    freq1: f32,
    freq2: f32,
    freq3: f32,
    phase1: f32,
    phase2: f32,
    phase3: f32,
    mix: f32,
    drive: f32,
    sample_rate: f32,
}

impl Exciter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            harmonic_amount: 0.5,
            freq1: 2500.0,
            freq2: 5000.0,
            freq3: 7500.0,
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
            mix: 0.5,
            drive: 0.3,
            sample_rate,
        }
    }

    /// Technical implementation of the set_harmonic_amount logic.
    pub fn set_harmonic_amount(&mut self, amount: f32) {
        self.harmonic_amount = amount.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_freq1 logic.
    pub fn set_freq1(&mut self, freq: f32) {
        self.freq1 = freq.max(500.0).min(15000.0);
    }
    /// Technical implementation of the set_freq2 logic.
    pub fn set_freq2(&mut self, freq: f32) {
        self.freq2 = freq.max(1000.0).min(18000.0);
    }
    /// Technical implementation of the set_freq3 logic.
    pub fn set_freq3(&mut self, freq: f32) {
        self.freq3 = freq.max(1500.0).min(20000.0);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let orig = input;

        let drive_sig = 1.0 + self.drive * 5.0;
        let preprocessed = (input * drive_sig).tanh() / drive_sig.tanh();

        let phase_inc1 = 2.0 * core::f32::consts::PI * self.freq1 / self.sample_rate;
        let phase_inc2 = 2.0 * core::f32::consts::PI * self.freq2 / self.sample_rate;
        let phase_inc3 = 2.0 * core::f32::consts::PI * self.freq3 / self.sample_rate;

        self.phase1 += phase_inc1;
        self.phase2 += phase_inc2;
        self.phase3 += phase_inc3;

        if self.phase1 > 2.0 * core::f32::consts::PI {
            self.phase1 -= 2.0 * core::f32::consts::PI;
        }
        if self.phase2 > 2.0 * core::f32::consts::PI {
            self.phase2 -= 2.0 * core::f32::consts::PI;
        }
        if self.phase3 > 2.0 * core::f32::consts::PI {
            self.phase3 -= 2.0 * core::f32::consts::PI;
        }

        let harm_level = self.harmonic_amount * 0.33;

        let harm1 = self.phase1.sin() * preprocessed.abs() * harm_level;
        let harm2 = self.phase2.sin() * preprocessed.abs() * harm_level;
        let harm3 = self.phase3.sin() * preprocessed.abs() * harm_level;

        let harmonics = harm1 + harm2 + harm3;

        orig * (1.0 - self.mix) + (orig + harmonics) * self.mix
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// Technical implementation of the PresenceBooster structure.
pub struct PresenceBooster {
    freq: f32,
    boost: f32,
    q: f32,
    state: [f32; 4],
    coeff: [f32; 5],
}

impl PresenceBooster {
    /// Initializes a new instance of the associated type.
    pub fn new(_sample_rate: f32) -> Self {
        let mut p = Self {
            freq: 3000.0,
            boost: 0.0,
            q: 2.0,
            state: [0.0; 4],
            coeff: [0.0; 5],
        };
        p.update_coeffs();
        p
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.max(1000.0).min(12000.0);
        self.update_coeffs();
    }

    /// Technical implementation of the set_boost logic.
    pub fn set_boost(&mut self, boost_db: f32) {
        self.boost = boost_db.clamp(-12.0, 12.0);
        self.update_coeffs();
    }

    /// Technical implementation of the set_q logic.
    pub fn set_q(&mut self, q: f32) {
        self.q = q.clamp(0.5, 10.0);
        self.update_coeffs();
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / 44100.0;
        let a = 10.0_f32.powf(self.boost / 40.0);
        let alpha = w.sin() / (2.0 * self.q);

        self.coeff[0] = 1.0 + alpha * a;
        self.coeff[1] = -2.0 * w.cos();
        self.coeff[2] = 1.0 - alpha * a;
        self.coeff[3] = 1.0 - alpha / a;
        self.coeff[4] = 2.0 * w.cos();
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let out = (self.coeff[0] * input + self.coeff[1] * self.state[0] + self.state[1])
            / (self.coeff[2] + 1.0);
        self.state[1] = self.state[0];
        self.state[0] = input;
        out
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

impl Default for Exciter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for PresenceBooster {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
