/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0d2a3e7b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/modulation.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::f32::consts::TAU;
use smoothie_core::math::{hermite_interpolate, sine_approx, tan_approx};
use smoothie_core::primitives::Sample;

#[derive(Debug, Clone)]
/// Technical implementation of the Chorus structure.
pub struct Chorus {
    buffer: [Sample; 4096],
    write_pos: usize,
    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    depth: f32,  // Samples
    base_delay: f32, // Samples
    feedback: f32,
    mix: f32,
}

impl Chorus {
    /// Initializes a new instance of the associated type.
    pub fn new(_sample_rate: f32) -> Self {
        Self::default()
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate.clamp(0.1, 10.0);
    }

    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth_ms: f32, sample_rate: f32) {
        self.lfo_depth = depth_ms * sample_rate / 1000.0;
        self.depth = self.lfo_depth;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let mask = 4095;
        let lfo = (sine_approx(self.lfo_phase * TAU) + 1.0) * 0.5;
        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let delay_samples = self.base_delay + lfo * self.lfo_depth;
        let read_pos = (self.write_pos as f32 - delay_samples + 4096.0) % 4096.0;

        let i0 = read_pos.floor() as usize;
        let i1 = (i0 + 1) & mask;
        let i2 = (i1 + 1) & mask;
        let i3 = (i0 + 4095) & mask;
        let frac = read_pos - i0 as f32;

        let delayed = hermite_interpolate(
            self.buffer[i3],
            self.buffer[i0],
            self.buffer[i1],
            self.buffer[i2],
            frac,
        );

        self.buffer[self.write_pos] = input + delayed * self.feedback;
        self.write_pos = (self.write_pos + 1) & mask;

        input * (1.0 - self.mix) + delayed * self.mix
    }
}

impl Default for Chorus {
    fn default() -> Self {
        Self {
            buffer: [0.0; 4096],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_rate: 0.5,
            lfo_depth: 88.0,
            depth: 88.0,
            base_delay: 882.0,
            feedback: 0.2,
            mix: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
/// Technical implementation of the Phaser structure.
pub struct Phaser {
    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    depth: f32,
    feedback: f32,
    stages: [f32; 6],
    prev_out: f32,
}

impl Phaser {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = (sine_approx(self.lfo_phase * TAU) + 1.0) * 0.5;
        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let freq = 200.0 + lfo * 1800.0;
        let tan_val = tan_approx((3.14159 * freq) / sample_rate);
        let coeff = (tan_val - 1.0) / (tan_val + 1.0);

        let input_with_fb = input + self.prev_out * self.feedback;
        let mut curr = input_with_fb;

        for i in 0..6 {
            let out = coeff * curr + self.stages[i];
            self.stages[i] = curr - coeff * out;
            curr = out;
        }

        self.prev_out = curr;
        (input + curr) * 0.5
    }
}

impl Default for Phaser {
    fn default() -> Self {
        Self {
            lfo_phase: 0.0,
            lfo_rate: 0.2,
            lfo_depth: 0.7,
            depth: 0.7,
            feedback: 0.5,
            stages: [0.0; 6],
            prev_out: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
/// Technical implementation of the Tremolo structure.
pub struct Tremolo {
    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    depth: f32,
}

impl Tremolo {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self::default()
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = (sine_approx(self.lfo_phase * TAU) + 1.0) * 0.5;
        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let mod_val = 1.0 - self.depth + lfo * self.depth;
        input * mod_val
    }
}

impl Default for Tremolo {
    fn default() -> Self {
        Self {
            lfo_phase: 0.0,
            lfo_rate: 5.0,
            lfo_depth: 0.5,
            depth: 0.5,
        }
    }
}
