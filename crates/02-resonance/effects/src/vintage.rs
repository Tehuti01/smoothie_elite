/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x56dadabe | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/vintage.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::exp_approx;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

/// Technical implementation of the VintageEq structure.
pub struct VintageEq {
    low_shelf: ShelfFilter,
    low_mid: PeakingFilter,
    high_mid: PeakingFilter,
    high_shelf: ShelfFilter,
    sample_rate: f32,
}

struct ShelfFilter {
    freq: f32,
    gain: f32,
    coeff: [f32; 3],
    state: [f32; 2],
}

struct PeakingFilter {
    freq: f32,
    gain: f32,
    q: f32,
    a: f32,
    coeff: [f32; 6],
    state: [f32; 4],
}

impl VintageEq {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            low_shelf: ShelfFilter::new(sample_rate, 100.0, 0.0, true),
            low_mid: PeakingFilter::new(sample_rate, 400.0, 0.0, 1.0),
            high_mid: PeakingFilter::new(sample_rate, 2500.0, 0.0, 1.0),
            high_shelf: ShelfFilter::new(sample_rate, 8000.0, 0.0, false),
            sample_rate,
        }
    }

    /// Technical implementation of the set_low_gain logic.
    pub fn set_low_gain(&mut self, gain_db: f32) {
        self.low_shelf.gain = gain_db;
        self.low_shelf.update_coeffs();
    }
    /// Technical implementation of the set_low_mid_gain logic.
    pub fn set_low_mid_gain(&mut self, gain_db: f32) {
        self.low_mid.gain = gain_db;
        self.low_mid.update_coeffs();
    }
    /// Technical implementation of the set_low_mid_freq logic.
    pub fn set_low_mid_freq(&mut self, freq: f32) {
        self.low_mid.freq = freq.max(20.0).min(self.sample_rate * 0.4);
        self.low_mid.update_coeffs();
    }
    /// Technical implementation of the set_high_mid_gain logic.
    pub fn set_high_mid_gain(&mut self, gain_db: f32) {
        self.high_mid.gain = gain_db;
        self.high_mid.update_coeffs();
    }
    /// Technical implementation of the set_high_mid_freq logic.
    pub fn set_high_mid_freq(&mut self, freq: f32) {
        self.high_mid.freq = freq.max(20.0).min(self.sample_rate * 0.4);
        self.high_mid.update_coeffs();
    }
    /// Technical implementation of the set_high_gain logic.
    pub fn set_high_gain(&mut self, gain_db: f32) {
        self.high_shelf.gain = gain_db;
        self.high_shelf.update_coeffs();
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut s = self.low_shelf.process(input);
        s = self.low_mid.process(s);
        s = self.high_mid.process(s);
        self.high_shelf.process(s)
    }

    /// Technical implementation of the sample_rate logic.
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

impl ShelfFilter {
    /// Initializes a new instance of the associated type.
    fn new(_sr: f32, freq: f32, gain: f32, _is_low: bool) -> Self {
        let mut s = Self {
            freq,
            gain,
            coeff: [0.0; 3],
            state: [0.0; 2],
        };
        s.update_coeffs();
        s
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / 44100.0;
        let alpha = w.sin() / (1.0 + 1.0 / (1.0 + self.gain.abs() * 0.5));
        self.coeff[0] = alpha;
        self.coeff[1] = -alpha;
        self.coeff[2] = 0.0;
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        let b0 = 1.0 + self.gain * 0.1;
        let out = b0 * input + self.coeff[0] * self.state[0] + self.coeff[1] * self.state[1];
        self.state[1] = self.state[0];
        self.state[0] = input - out;
        out
    }
}

impl PeakingFilter {
    /// Initializes a new instance of the associated type.
    fn new(_sr: f32, freq: f32, gain: f32, q: f32) -> Self {
        let mut p = Self {
            freq,
            gain,
            q,
            a: 1.0,
            coeff: [0.0; 6],
            state: [0.0; 4],
        };
        p.update_coeffs();
        p
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / 44100.0;
        let alpha = w.sin() / (2.0 * self.q);
        let a = 10.0_f32.powf(self.gain / 40.0);
        self.a = a;
        self.coeff[0] = 1.0 + alpha * a;
        self.coeff[1] = -2.0 * w.cos();
        self.coeff[2] = 1.0 - alpha * a;
        self.coeff[3] = 1.0 - alpha / a;
        self.coeff[4] = 2.0 * w.cos();
        self.coeff[5] = 1.0 + alpha / a;
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        let b0 = 1.0 + self.a;
        let _d = self.coeff[2] + self.coeff[0] * self.coeff[4] + self.coeff[1];
        let out = (b0 * input + self.coeff[0] * self.state[0] + self.coeff[1] * self.state[1])
            / (self.coeff[2] + 1.0);
        self.state[1] = self.state[0];
        self.state[0] = input - out;
        out
    }
}

/// Technical implementation of the TapeEmulator structure.
pub struct TapeEmulator {
    saturation: f32,
    bias: f32,
    hiss: f32,
    tape_speed: f32,
    high_freq: f32,
    state: [f32; 4],
    sample_rate: f32,
}

impl TapeEmulator {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            saturation: 0.5,
            bias: 0.0,
            hiss: 0.01,
            tape_speed: 1.0,
            high_freq: 10000.0,
            state: [0.0; 4],
            sample_rate,
        }
    }

    /// Technical implementation of the set_saturation logic.
    pub fn set_saturation(&mut self, sat: f32) {
        self.saturation = sat.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_bias logic.
    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias.clamp(-1.0, 1.0);
    }
    /// Technical implementation of the set_hiss logic.
    pub fn set_hiss(&mut self, hiss: f32) {
        self.hiss = hiss.clamp(0.0, 0.1);
    }
    /// Technical implementation of the set_tape_speed logic.
    pub fn set_tape_speed(&mut self, speed: f32) {
        self.tape_speed = speed.clamp(0.5, 2.0);
    }
    /// Technical implementation of the set_high_freq logic.
    pub fn set_high_freq(&mut self, freq: f32) {
        self.high_freq = freq.max(1000.0).min(20000.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let biased = input + self.bias * 0.1;
        let sat_curve = 1.0 + self.saturation * 2.0;
        let saturated = (biased * sat_curve).tanh() / sat_curve.tanh();

        let _fs = 0.5 * self.tape_speed;
        let mut out = saturated;

        for i in 0..2 {
            let alpha = exp_approx(-2.0 * core::f32::consts::PI * 50.0 / self.sample_rate);
            out = out * (1.0 - alpha) + self.state[i] * alpha;
            self.state[i] = out;
        }

        let hiss_noise =
            ((self.state[2] * 12.9898 + self.state[3] * 78.233).sin() * 43758.5453).fract() * 2.0
                - 1.0;
        self.state[2] = self.state[3];
        self.state[3] = hiss_noise;

        out + hiss_noise * self.hiss
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

/// Technical implementation of the MicPreamp structure.
pub struct MicPreamp {
    gain: f32,
    impedance: f32,
    color: f32,
    input_stage: f32,
    output_stage: f32,
    high_pass: f32,
    sample_rate: f32,
}

impl MicPreamp {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            gain: 0.5,
            impedance: 0.5,
            color: 0.3,
            input_stage: 0.0,
            output_stage: 0.0,
            high_pass: 80.0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_gain logic.
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_impedance logic.
    pub fn set_impedance(&mut self, imp: f32) {
        self.impedance = imp.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_color logic.
    pub fn set_color(&mut self, color: f32) {
        self.color = color.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_high_pass logic.
    pub fn set_high_pass(&mut self, freq: f32) {
        self.high_pass = freq.max(20.0).min(500.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let pre_gain = 1.0 + self.gain * 20.0;
        let mut amplified = input * pre_gain;

        let saturation = 1.0 + self.color * 3.0;
        amplified = (amplified * saturation).tanh() / saturation.tanh();

        let rc = 1.0 / (2.0 * core::f32::consts::PI * self.high_pass * self.sample_rate);
        let alpha = rc / (rc + 1.0);
        let hp = amplified - self.input_stage;
        self.input_stage += alpha * hp;

        let eq_boost = 1.0 + self.color * 0.5;
        let out_gain = 1.0 / (1.0 + self.gain * 10.0);

        amplified * eq_boost * out_gain
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

impl Default for VintageEq {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for TapeEmulator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for MicPreamp {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
