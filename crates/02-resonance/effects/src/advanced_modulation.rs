/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1481fc20 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/advanced_modulation.rs                                                         │
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
/// Flanger with feedback, advanced phaser, and vibrato.

use smoothie_core::math::{hermite_interpolate, sine_approx};
use smoothie_core::primitives::Sample;

#[repr(align(64))]
/// Technical implementation of the Flanger structure.
pub struct Flanger {
    buffer: [Sample; 8192],
    write_pos: usize,
    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    base_delay: f32,
    feedback: f32,
    mix: f32,
}

impl Flanger {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: [0.0; 8192],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_rate: 0.25,
            lfo_depth: 4.0,
            base_delay: 2.0,
            feedback: 0.5,
            mix: 0.5,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate.clamp(0.01, 10.0);
    }
    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth_ms: f32, sample_rate: f32) {
        self.lfo_depth = depth_ms * sample_rate / 1000.0;
    }
    /// Technical implementation of the set_feedback logic.
    pub fn set_feedback(&mut self, fb: f32) {
        self.feedback = fb.clamp(-0.95, 0.95);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = sine_approx(self.lfo_phase * core::f32::consts::TAU);

        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let delay_samples = self.base_delay + lfo * self.lfo_depth;
        let mask = 8191;
        let read_pos = (self.write_pos as f32 - delay_samples + 8192.0) % 8192.0;

        let i0 = read_pos as usize;
        let i1 = (i0 + 1) & mask;
        let i2 = (i1 + 1) & mask;
        let im1 = if i0 > 0 { i0 - 1 } else { mask };
        let frac = read_pos - i0 as f32;

        let delayed = hermite_interpolate(
            self.buffer[im1],
            self.buffer[i0],
            self.buffer[i1],
            self.buffer[i2],
            frac,
        );

        let feedback_signal = delayed * self.feedback;
        self.buffer[self.write_pos] = input + feedback_signal;
        self.write_pos = (self.write_pos + 1) & mask;

        input * (1.0 - self.mix) + delayed * self.mix
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.buffer = [0.0; 8192];
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

#[repr(align(64))]
/// Technical implementation of the Vibrato structure.
pub struct Vibrato {
    buffer: [Sample; 16384],
    write_pos: usize,
    lfo_phase: f32,
    lfo_rate: f32,
    lfo_depth: f32,
    base_delay: f32,
    mod_rate: f32,
    mod_depth: f32,
}

impl Vibrato {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: [0.0; 16384],
            write_pos: 0,
            lfo_phase: 0.0,
            lfo_rate: 5.0,
            lfo_depth: 8.0,
            base_delay: 500.0,
            mod_rate: 5.0,
            mod_depth: 8.0,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate.clamp(0.1, 20.0);
    }
    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth_ms: f32, sample_rate: f32) {
        self.lfo_depth = depth_ms * sample_rate / 1000.0;
    }
    /// Technical implementation of the set_modulation logic.
    pub fn set_modulation(&mut self, rate: f32, depth_ms: f32, sample_rate: f32) {
        self.mod_rate = rate;
        self.mod_depth = depth_ms * sample_rate / 1000.0;
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = sine_approx(self.lfo_phase * core::f32::consts::TAU);

        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let delay_samples = self.base_delay + lfo * self.lfo_depth;
        let mask = 16383;
        let read_pos = (self.write_pos as f32 - delay_samples + 16384.0) % 16384.0;

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
        self.buffer = [0.0; 16384];
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}

#[repr(align(64))]
/// Technical implementation of the PhaserAdvanced structure.
pub struct PhaserAdvanced {
    stages: [AllPassStage; 8],
    lfo_phase: f32,
    lfo_rate: f32,
    min_freq: f32,
    max_freq: f32,
    feedback: f32,
    mix: f32,
    stages_used: usize,
}

struct AllPassStage {
    delay: f32,
    coef: f32,
    buffer: f32,
}

impl PhaserAdvanced {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        let base_freqs = [200.0, 400.0, 600.0, 800.0, 1000.0, 1200.0, 1400.0, 1600.0];
        let mut stages = [AllPassStage {
            delay: 0.0,
            coef: 0.0,
            buffer: 0.0,
        }; 8];

        for (i, stage) in stages.iter_mut().enumerate() {
            stage.delay = base_freqs[i];
            stage.coef = 0.0;
            stage.buffer = 0.0;
        }

        Self {
            stages,
            lfo_phase: 0.0,
            lfo_rate: 0.5,
            min_freq: 200.0,
            max_freq: 2000.0,
            feedback: 0.3,
            mix: 0.5,
            stages_used: 4,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate.clamp(0.01, 10.0);
    }
    /// Technical implementation of the set_range logic.
    pub fn set_range(&mut self, min_hz: f32, max_hz: f32) {
        self.min_freq = min_hz.clamp(20.0, 10000.0);
        self.max_freq = max_hz.clamp(self.min_freq, 20000.0);
    }
    /// Technical implementation of the set_stages logic.
    pub fn set_stages(&mut self, n: usize) {
        self.stages_used = n.clamp(1, 8);
    }
    /// Technical implementation of the set_feedback logic.
    pub fn set_feedback(&mut self, fb: f32) {
        self.feedback = fb.clamp(0.0, 0.9);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let lfo = sine_approx(self.lfo_phase * core::f32::consts::TAU);

        self.lfo_phase += self.lfo_rate / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let freq = self.min_freq + lfo * (self.max_freq - self.min_freq);

        let mut output = input;
        let mut prev = input * self.feedback;

        for i in 0..self.stages_used {
            let w = 2.0 * core::f32::consts::PI * freq * (1.0 + i as f32 * 0.5);
            let tan_val = (w / sample_rate).tan();
            let coef = (tan_val - 1.0) / (tan_val + 1.0);

            let stage_in = output + prev;
            let stage_out = coef * stage_in + self.stages[i].buffer;
            self.stages[i].buffer = stage_in - coef * stage_out;

            prev = stage_out;
            output = stage_out;
        }

        input * (1.0 - self.mix) + output * self.mix
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for stage in self.stages.iter_mut() {
            stage.buffer = 0.0;
        }
    }
}

impl Default for PhaserAdvanced {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

pub trait TanApprox {
    /// Technical implementation of the tan logic.
    fn tan(self) -> f32;
}
impl TanApprox for f32 {
    /// Technical implementation of the tan logic.
    fn tan(self) -> f32 {
        let sin = sine_approx(self);
        let cos = cos_approx(self);
        if cos.abs() < 1e-10 {
            1e10
        } else {
            sin / cos
        }
    }
}
