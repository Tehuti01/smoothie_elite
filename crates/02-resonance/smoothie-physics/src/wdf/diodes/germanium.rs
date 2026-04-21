/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf8c66ec1 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/diodes/germanium.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// to silicon diodes, and exhibit characteristic "soft" forward conduction.
/// Popular in vintage-style clipping circuits.

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the GermaniumDiode structure.
pub struct GermaniumDiode<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    saturation_current: f32,
    thermal_voltage: f32,
    series_resistance: f32,
    breakdown_voltage: f32,
    breakdown_factor: f32,
}

impl<'a, Node: WdfNode> GermaniumDiode<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node) -> Self {
        Self {
            node,
            saturation_current: 1e-4,
            thermal_voltage: 0.026,
            series_resistance: 2.0,
            breakdown_voltage: -50.0,
            breakdown_factor: 0.01,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        let vt = self.thermal_voltage;
        let i_s = self.saturation_current;
        let rs = self.series_resistance;
        let v_br = self.breakdown_voltage;
        let m = self.breakdown_factor;

        let mut v_d = incident;

        if incident < v_br {
            for _ in 0..3 {
                let i_rev = i_s * (exp_approx(v_br / vt) - 1.0) + (v_d - v_br) / (rs.max(1.0));
                let residual = v_d + rs * i_rev - incident;
                v_d -= residual / (1.0 + rs * m);
            }
        } else {
            for _ in 0..4 {
                let exp_term = exp_approx(v_d / vt);
                let i_f = i_s * (exp_term - 1.0);
                let residual = v_d + rs * i_f - incident;
                let jacobian = 1.0 + (rs * i_s / vt) * exp_term;
                v_d -= residual / jacobian;
                v_d = v_d.max(0.0);
            }
        }

        let v_out = if incident < v_br {
            v_br - m * (v_br - incident).max(0.0)
        } else {
            v_d.max(0.0)
        };

        let reflected = 2.0 * v_out - incident;
        self.node.wave_down(reflected);
        v_out
    }
}
