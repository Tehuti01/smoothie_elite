/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2cf44c10 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/diodes/schottky.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// to silicon diodes, and are used in fast switching and signal clipping applications.

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the SchottkyDiode structure.
pub struct SchottkyDiode<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    saturation_current: f32,
    thermal_voltage: f32,
    series_resistance: f32,
    n_factor: f32,
}

impl<'a, Node: WdfNode> SchottkyDiode<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node) -> Self {
        Self {
            node,
            saturation_current: 5e-6,
            thermal_voltage: 0.025,
            series_resistance: 0.5,
            n_factor: 1.1,
        }
    }

    /// Technical implementation of the with_params logic.
    pub fn with_params(node: &'a mut Node, is: f32, n: f32, rs: f32) -> Self {
        Self {
            node,
            saturation_current: is,
            thermal_voltage: 0.025,
            series_resistance: rs,
            n_factor: n,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        let r_port = self.node.get_port_resistance();
        let vt = self.thermal_voltage * self.n_factor;

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
