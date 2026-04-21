/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xfe8de345 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/diodes/silicon.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Used in clipping, rectification, and general signal processing.

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the SiliconDiode structure.
pub struct SiliconDiode<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    saturation_current: f32,
    thermal_voltage: f32,
    series_resistance: f32,
    ideality_factor: f32,
}

impl<'a, Node: WdfNode> SiliconDiode<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node) -> Self {
        Self {
            node,
            saturation_current: 1e-12,
            thermal_voltage: 0.02585,
            series_resistance: 0.25,
            ideality_factor: 1.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        let vt = self.thermal_voltage * self.ideality_factor;
        let i_s = self.saturation_current;
        let rs = self.series_resistance;

        let mut v_d = incident.max(0.0);

        for _ in 0..4 {
            let exp_term = exp_approx(v_d / vt);
            let i_f = i_s * (exp_term - 1.0);
            let residual = v_d + rs * i_f - incident;
            let jacobian = 1.0 + (rs * i_s / vt) * exp_term;
            v_d -= residual / jacobian;
            v_d = v_d.max(0.0);
        }

        let exp_term = exp_approx(v_d / vt);
        let i_f = i_s * (exp_term - 1.0);
        let v_out = v_d - i_f * rs;

        let reflected = 2.0 * v_out - incident;
        self.node.wave_down(reflected);
        v_out
    }
}
