/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb252fc5f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/dsp/src/filters.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::TAU;
use smoothie_core::math::{cosine_approx, sine_approx, tan_approx};
use smoothie_core::primitives::Sample;

pub mod dc_blocker;
pub use dc_blocker::*;

use smoothie_core::plugin::Reset;

impl Reset for BiquadFilter {
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl Reset for StateVariableFilter {
    fn reset(&mut self) {
        self.lowpass = 0.0;
        self.bandpass = 0.0;
        self.highpass = 0.0;
    }
}

#[derive(Clone, Copy, Debug)]
/// Technical implementation of the FilterType enumeration.
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    Peaking,
    LowShelf,
    HighShelf,
}

/// Technical implementation of the BiquadFilter structure.
pub struct BiquadFilter {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadFilter {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Technical implementation of the set_lowpass logic.
    pub fn set_lowpass(&mut self, frequency: f32, sample_rate: f32, q: f32) {
        let omega = TAU * frequency / sample_rate;
        let sin_omega = sine_approx(omega);
        let cos_omega = cosine_approx(omega);
        let alpha = sin_omega / (2.0 * q);
        self.b0 = (1.0 - cos_omega) / 2.0;
        self.b1 = 1.0 - cos_omega;
        self.b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        self.a1 = -2.0 * cos_omega / a0;
        self.a2 = (1.0 - alpha) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    /// Technical implementation of the set_highpass logic.
    pub fn set_highpass(&mut self, frequency: f32, sample_rate: f32, q: f32) {
        let omega = TAU * frequency / sample_rate;
        let sin_omega = sine_approx(omega);
        let cos_omega = cosine_approx(omega);
        let alpha = sin_omega / (2.0 * q);
        self.b0 = (1.0 + cos_omega) / 2.0;
        self.b1 = -(1.0 + cos_omega);
        self.b2 = (1.0 + cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        self.a1 = -2.0 * cos_omega / a0;
        self.a2 = (1.0 - alpha) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    /// Technical implementation of the set_bandpass logic.
    pub fn set_bandpass(&mut self, frequency: f32, sample_rate: f32, q: f32) {
        let omega = TAU * frequency / sample_rate;
        let sin_omega = sine_approx(omega);
        let cos_omega = cosine_approx(omega);
        let alpha = sin_omega / (2.0 * q);
        self.b0 = q * alpha;
        self.b1 = 0.0;
        self.b2 = -q * alpha;
        let a0 = 1.0 + alpha;
        self.a1 = -2.0 * cos_omega / a0;
        self.a2 = (1.0 - alpha) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    /// Technical implementation of the set_notch logic.
    pub fn set_notch(&mut self, frequency: f32, sample_rate: f32, q: f32) {
        let omega = TAU * frequency / sample_rate;
        let sin_omega = sine_approx(omega);
        let cos_omega = cosine_approx(omega);
        let alpha = sin_omega / (2.0 * q);
        self.b0 = 1.0;
        self.b1 = -2.0 * cos_omega;
        self.b2 = 1.0;
        let a0 = 1.0 + alpha;
        self.a1 = -2.0 * cos_omega / a0;
        self.a2 = (1.0 - alpha) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    /// Technical implementation of the set_peaking logic.
    pub fn set_peaking(&mut self, frequency: f32, sample_rate: f32, q: f32, gain_db: f32) {
        use smoothie_core::math::fast_pow;
        let omega = TAU * frequency / sample_rate;
        let sin_omega = sine_approx(omega);
        let cos_omega = cosine_approx(omega);
        let alpha = sin_omega / (2.0 * q);
        let a = fast_pow(10.0, gain_db / 40.0);
        self.b0 = 1.0 + alpha * a;
        self.b1 = -2.0 * cos_omega;
        self.b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        self.a1 = -2.0 * cos_omega / a0;
        self.a2 = (1.0 - alpha / a) / a0;
        self.b0 /= a0;
        self.b1 /= a0;
        self.b2 /= a0;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }

    /// Technical implementation of the set_coefficients logic.
    pub fn set_coefficients(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;
        self.a1 = a1;
        self.a2 = a2;
    }
}

impl Default for BiquadFilter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the StateVariableFilter structure.
pub struct StateVariableFilter {
    frequency: f32,
    sample_rate: f32,
    resonance: f32,
    f_coeff: f32,
    lowpass: f32,
    bandpass: f32,
    highpass: f32,
}

impl StateVariableFilter {
    /// Initializes a new instance of the associated type.
    pub fn new(frequency: f32, sample_rate: f32, resonance: f32) -> Self {
        let f_coeff = 2.0 * sine_approx(core::f32::consts::PI * frequency / sample_rate);
        Self {
            frequency,
            sample_rate,
            resonance: resonance.max(0.5),
            f_coeff,
            lowpass: 0.0,
            bandpass: 0.0,
            highpass: 0.0,
        }
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.f_coeff = 2.0 * sine_approx(core::f32::consts::PI * frequency / self.sample_rate);
    }

    /// Technical implementation of the set_resonance logic.
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.max(0.5);
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.f_coeff = 2.0 * sine_approx(core::f32::consts::PI * self.frequency / sample_rate);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> (f32, f32, f32) {
        let q_inv = 1.0 / self.resonance;
        self.lowpass += self.f_coeff * self.bandpass;
        self.highpass = input - self.lowpass - q_inv * self.bandpass;
        self.bandpass += self.f_coeff * self.highpass;
        (self.lowpass, self.bandpass, self.highpass)
    }

    /// Technical implementation of the lowpass logic.
    pub fn lowpass(&self) -> f32 {
        self.lowpass
    }
    /// Technical implementation of the bandpass logic.
    pub fn bandpass(&self) -> f32 {
        self.bandpass
    }
    /// Technical implementation of the highpass logic.
    pub fn highpass(&self) -> f32 {
        self.highpass
    }
    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.lowpass = 0.0;
        self.bandpass = 0.0;
        self.highpass = 0.0;
    }
}

impl Default for StateVariableFilter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(1000.0, 44100.0, 1.0)
    }
}

/// Technical implementation of the LadderFilter structure.
pub struct LadderFilter {
    stage: [f32; 4],
    delay: [f32; 4],
    frequency: f32,
    sample_rate: f32,
    resonance: f32,
    cutoff_coeff: f32,
}

impl LadderFilter {
    /// Initializes a new instance of the associated type.
    pub fn new(frequency: f32, sample_rate: f32, resonance: f32) -> Self {
        let cutoff_coeff = Self::compute_coefficient(frequency, sample_rate);
        Self {
            stage: [0.0; 4],
            delay: [0.0; 4],
            frequency,
            sample_rate,
            resonance: resonance.clamp(0.0, 0.99),
            cutoff_coeff,
        }
    }

    /// Technical implementation of the compute_coefficient logic.
    fn compute_coefficient(frequency: f32, sample_rate: f32) -> f32 {
        let normalized = core::f32::consts::PI * frequency / sample_rate;
        let t = tan_approx(normalized);
        t / (1.0 + t)
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.cutoff_coeff = Self::compute_coefficient(frequency, self.sample_rate);
    }

    /// Technical implementation of the set_resonance logic.
    pub fn set_resonance(&mut self, resonance: f32) {
        self.resonance = resonance.clamp(0.0, 0.99);
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.cutoff_coeff = Self::compute_coefficient(self.frequency, sample_rate);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let g = self.cutoff_coeff;
        let feedback = self.stage[3] * self.resonance * 4.0;
        let input_with_feedback = input - feedback;
        self.stage[0] = self.delay[0] + g * (input_with_feedback - self.delay[0]);
        self.delay[0] = self.stage[0];
        self.stage[1] = self.delay[1] + g * (self.stage[0] - self.delay[1]);
        self.delay[1] = self.stage[1];
        self.stage[2] = self.delay[2] + g * (self.stage[1] - self.delay[2]);
        self.delay[2] = self.stage[2];
        self.stage[3] = self.delay[3] + g * (self.stage[2] - self.delay[3]);
        self.delay[3] = self.stage[3];
        self.stage[3]
    }

    /// Primary real-time signal processing execution block.
    pub fn process_into(&mut self, input: &[Sample], output: &mut [Sample]) {
        let len = input.len().min(output.len());
        for i in 0..len {
            output[i] = self.process(input[i]);
        }
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.stage = [0.0; 4];
        self.delay = [0.0; 4];
    }
}

impl Default for LadderFilter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(1000.0, 44100.0, 0.5)
    }
}
