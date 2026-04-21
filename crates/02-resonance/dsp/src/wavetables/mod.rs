/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x510b8ccb | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/dsp/src/wavetables/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod generation;
pub use generation::{generate_from_samples, normalize, VintageShape, WavetableGenerator};

use smoothie_core::constants::{F_233, TAU};
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;

/// Technical implementation of the Wavetable structure.
pub struct Wavetable {
    data: [Sample; F_233],
    size: usize,
}

impl Wavetable {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            data: [0.0; F_233],
            size: F_233,
        }
    }

    /// Technical implementation of the sine logic.
    pub fn sine() -> Self {
        let mut wt = Self::new();
        for i in 0..F_233 {
            let phase = (i as f32) / (F_233 as f32) * TAU;
            wt.data[i] = sine_approx(phase);
        }
        wt.size = F_233;
        wt
    }

    /// Technical implementation of the triangle logic.
    pub fn triangle() -> Self {
        let mut wt = Self::new();
        for i in 0..F_233 {
            let normalized = (i as f32) / (F_233 as f32);
            wt.data[i] = if normalized < 0.25 {
                normalized * 8.0 - 1.0
            } else if normalized < 0.75 {
                3.0 - normalized * 8.0
            } else {
                normalized * 8.0 - 7.0
            };
        }
        wt.size = F_233;
        wt
    }

    /// Technical implementation of the sawtooth logic.
    pub fn sawtooth() -> Self {
        let mut wt = Self::new();
        for i in 0..F_233 {
            let normalized = (i as f32) / (F_233 as f32);
            wt.data[i] = 2.0 * normalized - 1.0;
        }
        wt.size = F_233;
        wt
    }

    /// Technical implementation of the square logic.
    pub fn square() -> Self {
        let mut wt = Self::new();
        for i in 0..F_233 {
            let normalized = (i as f32) / (F_233 as f32);
            wt.data[i] = if normalized < 0.5 { 1.0 } else { -1.0 };
        }
        wt.size = F_233;
        wt
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, phase: f32) -> Sample {
        let pos = phase * (self.size as f32);
        let i0 = pos as usize % self.size;
        let i1 = (i0 + 1) % self.size;
        let frac = pos - (pos as i32) as f32;
        self.data[i0] * (1.0 - frac) + self.data[i1] * frac
    }

    /// Technical implementation of the size logic.
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for Wavetable {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
