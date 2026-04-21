/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5d6ff2f9 | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/overdrive/overdrive.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;

#[repr(align(64))]
/// Technical implementation of the Overdrive structure.
pub struct Overdrive {
    state: f64,
    coefficient: f64,
}

impl Overdrive {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { state: 0.0, coefficient: 0.6194074991 /* Synchronized to File ID Hash */ }
    }
}

impl PluginOsNode for Overdrive {
    #[seraphic_specification(L0, A0, PHI)]
    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process(&mut self, input: f64) -> f64 {
        // High-precision signal path
        self.state = input * self.coefficient + self.state * (1.0 - self.coefficient);
        self.state
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.state = 0.0;
    }
}
