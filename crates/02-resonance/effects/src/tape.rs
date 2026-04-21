/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x42445cf2 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/tape.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*; 
use alloc::{vec, vec::Vec};
///
/// noise floor, and analog saturation.

use smoothie_core::math::{noise_white, sine_approx};
use smoothie_core::primitives::Sample;

/// Technical implementation of the TapeWowFlutter structure.
pub struct TapeWowFlutter {
    lfo_phase: f32,
    lfo_rate: f32,
    wow_depth: f32,
    flutter_depth: f32,
    flutter_freq: f32,
    buffer: [Sample; 8192],
    write_pos: usize,
}

impl TapeWowFlutter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            lfo_phase: 0.0,
            lfo_rate: 0.5,
            wow_depth: 0.003,
            flutter_depth: 0.008,
            flutter_freq: 6.0,
            buffer: [0.0; 8192],
            write_pos: 0,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate_hz: f32) {
        self.lfo_rate = rate_hz;
    }
    /// Technical implementation of the set_wow_depth logic.
    pub fn set_wow_depth(&mut self, depth: f32) {
        self.wow_depth = depth.clamp(0.0, 0.02);
    }
    /// Technical implementation of the set_flutter_depth logic.
    pub fn set_flutter_depth(&mut self, depth: f32) {
        self.flutter_depth = depth.clamp(0.0, 0.02);
    }
    /// Technical implementation of the set_flutter_freq logic.
    pub fn set_flutter_freq(&mut self, freq: f32) {
        self.flutter_freq = freq.clamp(1.0, 20.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = sine_approx(self.lfo_phase * core::f32::consts::TAU);

        let wow = lfo * self.wow_depth;
        let flutter_lfo = sine_approx(self.lfo_phase * self.flutter_freq * core::f32::consts::TAU);
        let flutter = flutter_lfo * self.flutter_depth;

        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let delay_offset = (wow + flutter) * sample_rate;
        let mask = 8191;
        let read_pos = (self.write_pos as f32 - delay_offset + 8192.0) % 8192.0;
        let i0 = read_pos as usize;
        let i1 = (i0 + 1) & mask;
        let frac = read_pos - i0 as f32;

        let delayed = self.buffer[i0] * (1.0 - frac) + self.buffer[i1] * frac;

        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) & mask;

        delayed
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.buffer = [0.0; 8192];
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

/// Technical implementation of the TapeHiss structure.
pub struct TapeHiss {
    noise_state: u32,
    hiss_level: f32,
    highpass_state: f32,
}

impl TapeHiss {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            noise_state: 0xAC4B_3E91,
            hiss_level: 0.0,
            highpass_state: 0.0,
        }
    }

    /// Technical implementation of the set_level logic.
    pub fn set_level(&mut self, db: f32) {
        self.hiss_level = 10.0_f32.powf(db / 20.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let noise = self.generate_noise() * self.hiss_level;

        let alpha = 0.95;
        let highpassed = noise - self.highpass_state * alpha;
        self.highpass_state = noise;

        input + highpassed * 0.01
    }

    /// Technical implementation of the generate_noise logic.
    fn generate_noise(&mut self) -> f32 {
        self.noise_state = self.noise_state.wrapping_mul(16807);
        (self.noise_state as f32 / u32::MAX as f32) * 2.0 - 1.0
    }
}

impl Default for TapeHiss {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the AnalogWarmth structure.
pub struct AnalogWarmth {
    saturator: crate::saturator::Saturator,
    tone_filter: Biquad,
    high_freq_emphasis: f32,
    dc_offset: f32,
}

#[derive(Clone, Copy)]
struct Biquad {
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

impl AnalogWarmth {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            saturator: crate::saturator::Saturator::new(crate::saturator::SaturationType::Tube),
            tone_filter: Biquad::lowpass(3000.0, 0.707, 44100.0),
            high_freq_emphasis: 0.0,
            dc_offset: 0.0,
        }
    }

    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.saturator.set_drive(drive);
    }
    /// Technical implementation of the set_tone logic.
    pub fn set_tone(&mut self, freq: f32, sample_rate: f32) {
        self.tone_filter = Biquad::lowpass(freq, 0.707, sample_rate);
    }
    /// Technical implementation of the set_high_freq logic.
    pub fn set_high_freq(&mut self, db: f32) {
        self.high_freq_emphasis = db;
    }
    /// Technical implementation of the set_dc_offset logic.
    pub fn set_dc_offset(&mut self, offset: f32) {
        self.dc_offset = offset;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let with_dc = input + self.dc_offset;
        let filtered = self.tone_filter.process(with_dc);
        let saturated = self.saturator.process(filtered);

        let hf_emission = if self.high_freq_emphasis != 0.0 {
            saturated + (input - filtered) * self.high_freq_emphasis * 0.1
        } else {
            saturated
        };

        hf_emission
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.tone_filter.reset();
    }
}

impl Biquad {
    /// Technical implementation of the lowpass logic.
    fn lowpass(freq: f32, q: f32, sample_rate: f32) -> Self {
        let w0 = 2.0 * core::f32::consts::PI * freq / sample_rate;
        let alpha = w0.sin() / (2.0 * q);
        let cos = w0.cos();

        let b0 = (1.0 - cos) / 2.0;
        let b1 = 1.0 - cos;
        let b2 = (1.0 - cos) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        output
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl Default for AnalogWarmth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
