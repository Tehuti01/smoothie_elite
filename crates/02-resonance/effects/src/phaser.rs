/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x583149d2 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/phaser.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::TAU;
use smoothie_core::math::sine_approx;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

const NUM_STAGES: usize = 6;

/// Technical implementation of the Phaser structure.
pub struct Phaser {
    allpass_delays: [f32; NUM_STAGES],
    lfo_phase: f32,
    lfo_rate: f32,
    depth: f32,
    feedback: f32,
    mix: f32,
    center_frequency: f32,
    sweep_range: f32,
    last_output: f32,
    sample_rate: f32,
}

impl Phaser {
    /// Create a new phaser.
    ///
    /// - `rate`: LFO speed in Hz (0.1-10)
    /// - `depth`: Modulation depth (0.0-1.0)
    /// - `feedback`: Feedback amount (-0.95 to 0.95)
    /// - `sample_rate`: Audio sample rate
    pub fn new(rate: f32, depth: f32, feedback: f32, sample_rate: f32) -> Self {
        Self {
            allpass_delays: [0.0; NUM_STAGES],
            lfo_phase: 0.0,
            lfo_rate: rate,
            depth: depth.clamp(0.0, 1.0),
            feedback: feedback.clamp(-0.95, 0.95),
            mix: 0.5,
            center_frequency: 1000.0,
            sweep_range: 800.0,
            last_output: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate.max(0.01);
    }
    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.0, 1.0);
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
    pub fn process(&mut self, input: Sample) -> Sample {
        // LFO
        let lfo = sine_approx(self.lfo_phase * TAU);
        self.lfo_phase += self.lfo_rate / self.sample_rate;
        while self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Modulated coefficient
        let mod_freq = self.center_frequency + lfo * self.sweep_range * self.depth;
        let mod_freq = mod_freq.max(20.0).min(self.sample_rate * 0.49);
        let coefficient = (core::f32::consts::PI * mod_freq / self.sample_rate - 1.0)
            .max(-0.99)
            .min(0.99);

        // Input with feedback
        let input_fb = input + self.last_output * self.feedback;

        // Process through allpass chain
        let mut signal = input_fb;
        for i in 0..NUM_STAGES {
            let output = -coefficient * signal + self.allpass_delays[i];
            self.allpass_delays[i] = signal + coefficient * output;
            signal = output;
        }
        self.last_output = signal;

        // Mix
        input * (1.0 - self.mix) + signal * self.mix
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.allpass_delays = [0.0; NUM_STAGES];
        self.lfo_phase = 0.0;
        self.last_output = 0.0;
    }
}

impl Default for Phaser {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0.5, 0.7, 0.3, 44100.0)
    }
}
