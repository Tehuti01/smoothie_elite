/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x81ea4b95 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/diodes.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the ZenerDiode structure.
pub struct ZenerDiode<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    breakdown_voltage: f32,
    knee_softness: f32,
}

impl<'a, Node: WdfNode> ZenerDiode<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node, breakdown_voltage: f32) -> Self {
        Self {
            node,
            breakdown_voltage,
            knee_softness: 2.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        // Avalanche modeling utilizing inverse tangent soft limiting towards Zener bounds.
        // Extremely useful for modeled analog power supply sags.

        let bound = self.breakdown_voltage;
        let mut v_out = incident;

        if incident > bound {
            v_out = bound + ((incident - bound) / self.knee_softness);
        } else if incident < -0.7 {
            // Standard silicon forward drop
            v_out = -0.7 + ((incident + 0.7) / self.knee_softness);
        }

        let reflected = 2.0 * v_out - incident;
        self.node.wave_down(reflected);
        v_out
    }
}
