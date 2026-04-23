/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc74aad6c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/wavefolding.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// Audio-rate wave folding for aggressive harmonic distortion.
extern crate alloc;

use smoothie_core::constants::{STANDARD_PITCH, TAU};
use smoothie_core::math::{floor_approx, sine_approx};
use smoothie_core::primitives::Sample;

/// Wavefolding configuration.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the WavefoldConfig structure.
pub struct WavefoldConfig {
    pub folds: f32,       // Number of folding stages (1-8)
    pub asymmetry: f32,   // -1.0 to 1.0, adjusts fold bias
    pub drive: f32,       // Input gain before folding
    pub output_gain: f32, // Post-fold gain
}

impl Default for WavefoldConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            folds: 2.0,
            asymmetry: 0.0,
            drive: 1.0,
            output_gain: 0.5,
        }
    }
}

/// Technical implementation of the WavefoldOsc structure.
pub struct WavefoldOsc {
    pub phase: f32,
    pub phase_inc: f32,
    config: WavefoldConfig,
    last_out: f32,
}

impl WavefoldOsc {
    /// Initializes a new instance of the associated type.
    pub fn new(config: WavefoldConfig, sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            phase_inc: STANDARD_PITCH / sample_rate,
            config,
            last_out: 0.0,
        }
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32, sample_rate: f32) {
        self.phase_inc = freq / sample_rate;
    }

    #[inline(always)]
    /// Technical implementation of the fold_single logic.
    #[allow(dead_code)]
    fn fold_single(&self, x: f32) -> f32 {
        let folded = (x * self.config.folds + self.config.asymmetry).abs();
        folded - floor_approx(folded) - 0.5
    }

    #[inline(always)]
    /// Technical implementation of the fold_bounce logic.
    fn fold_bounce(&self, x: f32) -> f32 {
        let scaled = x * self.config.folds + self.config.asymmetry;
        2.0 * (scaled.abs() % 1.0) - 1.0
    }

    /// Technical implementation of the next logic.
    pub fn process(&mut self) -> Sample {
        // Generate base sine
        let input = sine_approx(self.phase * TAU);
        self.phase += self.phase_inc;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Apply input drive
        let driven = input * self.config.drive;

        // Apply wave folding
        let folded = self.fold_bounce(driven);

        // Second pass for more aggressive folding
        let folded2 = if self.config.folds > 4.0 {
            self.fold_bounce(folded)
        } else {
            folded
        };

        self.last_out = folded2 * self.config.output_gain;
        self.last_out
    }

    /// Primary real-time signal processing execution block.
    pub fn process_input(&mut self, input: Sample) -> Sample {
        let driven = input * self.config.drive;
        let folded = self.fold_bounce(driven);
        self.last_out = folded * self.config.output_gain;
        self.last_out
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}
