/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6130630f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/waveshaper.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::{vec, vec::Vec};
///
/// waveshaping, bitcrushing, and ring modulation.

use smoothie_core::math::{cos_approx, sine_approx};
use smoothie_core::primitives::Sample;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the DistortionType enumeration.
pub enum DistortionType {
    SoftClip,
    HardClip,
    Asymmetric,
    Curve(f32),
    SinApprox,
    SquareLaw,
}

/// Technical implementation of the Waveshaper structure.
pub struct Waveshaper {
    curve: [f32; 256],
    drive: f32,
    mix: f32,
    dtype: DistortionType,
}

impl Waveshaper {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        let mut shaper = Self {
            curve: [0.0; 256],
            drive: 1.0,
            mix: 1.0,
            dtype: DistortionType::SoftClip,
        };
        shaper.generate_curve(DistortionType::SoftClip);
        shaper
    }

    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.max(0.0);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_type logic.
    pub fn set_type(&mut self, dtype: DistortionType) {
        self.dtype = dtype;
        self.generate_curve(dtype);
    }

    /// Technical implementation of the generate_curve logic.
    fn generate_curve(&mut self, dtype: DistortionType) {
        for i in 0..256 {
            let x = (i as f32 / 255.0) * 2.0 - 1.0;
            let y = match dtype {
                DistortionType::SoftClip => x.tanh(),
                DistortionType::HardClip => x.max(-1.0).min(1.0),
                DistortionType::Asymmetric => {
                    if x >= 0.0 {
                        x.tanh()
                    } else {
                        x * 0.5
                    }
                }
                DistortionType::Curve(k) => x / (1.0 + k * x * x),
                DistortionType::SinApprox => sine_approx(x * core::f32::consts::FRAC_PI_2),
                DistortionType::SquareLaw => {
                    if x >= 0.0 {
                        x * x
                    } else {
                        -(x * x)
                    }
                }
            };
            self.curve[i] = y;
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let driven = input * self.drive;
        let clamped = driven.clamp(-1.0, 1.0);
        let index = ((clamped + 1.0) * 127.5) as usize;
        let index = index.min(255);

        let lookup = self.curve[index];
        input * (1.0 - self.mix) + lookup * self.mix
    }
}

impl Default for Waveshaper {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the Bitcrusher structure.
pub struct Bitcrusher {
    bits: u8,
    downsample_factor: u32,
    sample_buffer: [f32; 64],
    buffer_pos: usize,
    bit_depth: f32,
    hold_sample: f32,
    hold_counter: u32,
}

impl Bitcrusher {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            bits: 8,
            downsample_factor: 1,
            sample_buffer: [0.0; 64],
            buffer_pos: 0,
            bit_depth: 256.0,
            hold_sample: 0.0,
            hold_counter: 0,
        }
    }

    /// Technical implementation of the set_bits logic.
    pub fn set_bits(&mut self, bits: u8) {
        self.bits = bits.clamp(1, 24);
        self.bit_depth = (1u32 << self.bits) as f32;
    }

    /// Technical implementation of the set_downsample logic.
    pub fn set_downsample(&mut self, factor: u32) {
        self.downsample_factor = factor.clamp(1, 64) as u32;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        self.sample_buffer[self.buffer_pos] = input;
        self.buffer_pos = (self.buffer_pos + 1) & 63;

        self.hold_counter += 1;
        if self.hold_counter >= self.downsample_factor {
            self.hold_counter = 0;

            let sum: f32 = self.sample_buffer.iter().sum();
            let avg = sum / 64.0;

            let quantized = (avg * self.bit_depth).round() / self.bit_depth;
            self.hold_sample = quantized;
        }

        self.hold_sample
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.sample_buffer = [0.0; 64];
        self.buffer_pos = 0;
        self.hold_sample = 0.0;
        self.hold_counter = 0;
    }
}

impl Default for Bitcrusher {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the RingModulator structure.
pub struct RingModulator {
    carrier_phase: f32,
    carrier_freq: f32,
    mod_index: f32,
    mix: f32,
}

impl RingModulator {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            carrier_phase: 0.0,
            carrier_freq: 440.0,
            mod_index: 1.0,
            mix: 1.0,
        }
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, freq: f32) {
        self.carrier_freq = freq.clamp(20.0, 20000.0);
    }
    /// Technical implementation of the set_mod_index logic.
    pub fn set_mod_index(&mut self, index: f32) {
        self.mod_index = index.clamp(0.0, 2.0);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let carrier = sine_approx(self.carrier_phase * core::f32::consts::TAU);

        self.carrier_phase += self.carrier_freq / sample_rate;
        if self.carrier_phase >= 1.0 {
            self.carrier_phase -= 1.0;
        }

        let modulated = input * carrier * (1.0 + self.mod_index);
        input * (1.0 - self.mod_index) * self.mix + modulated * self.mod_index
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.carrier_phase = 0.0;
    }
}

impl Default for RingModulator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the RotarySpeaker structure.
pub struct RotarySpeaker {
    lfo_phase: f32,
    drive: f32,
    horn_angle: f32,
    drum_angle: f32,
    horn_speed: f32,
    drum_speed: f32,
    acceleration: f32,
    mic_distance: f32,
}

impl RotarySpeaker {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            lfo_phase: 0.0,
            drive: 0.5,
            horn_angle: 0.0,
            drum_angle: 0.0,
            horn_speed: 2.0,
            drum_speed: 1.5,
            acceleration: 0.3,
            mic_distance: 0.7,
        }
    }

    /// Technical implementation of the set_speed logic.
    pub fn set_speed(&mut self, rpm: f32) {
        let rps = rpm / 60.0;
        self.horn_speed = rps * 2.0;
        self.drum_speed = rps * 1.5;
    }

    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_mic_distance logic.
    pub fn set_mic_distance(&mut self, dist: f32) {
        self.mic_distance = dist.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32, delta: f32) -> Sample {
        let speed = if delta > 0.5 {
            self.horn_speed
        } else {
            self.horn_speed * (1.0 - self.acceleration)
        };

        self.lfo_phase += speed / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let lfo = (sine_approx(self.lfo_phase * core::f32::consts::TAU) + 1.0) * 0.5;

        self.horn_angle += 0.01 * lfo;
        self.drum_angle += 0.01 * lfo * 0.75;

        let horn_mod = 0.7 + 0.3 * sine_approx(self.horn_angle * 10.0);
        let drum_mod = 0.5 + 0.5 * sine_approx(self.drum_angle * 8.0);

        let combined =
            input * horn_mod * self.mic_distance + input * drum_mod * (1.0 - self.mic_distance);

        combined * (1.0 + self.drive * 0.5)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.horn_angle = 0.0;
        self.drum_angle = 0.0;
        self.lfo_phase = 0.0;
    }
}

impl Default for RotarySpeaker {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
