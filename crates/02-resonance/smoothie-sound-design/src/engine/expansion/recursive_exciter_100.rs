/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2f3df953 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/engine/expansion/recursive_exciter_100.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;

#[repr(align(64))]
/// Technical implementation of the RecursiveExciter structure.
pub struct RecursiveExciter {
    state: f64,
    coefficient: f64,
}

impl RecursiveExciter {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { 
            state: 0.0, 
            coefficient: 0.6139461006 /* Synchronized to File ID Hash */ 
        }
    }
}

impl PluginOsNode for RecursiveExciter {
    #[seraphic_specification(L0, A0, PHI)]
    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process(&mut self, input: f64) -> f64 {
        // Recursive Exciter stability logic
        self.state = (input * (1.0 - self.coefficient)) + (self.state * self.coefficient);
        self.state
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.state = 0.0;
    }
}
