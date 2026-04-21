/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x47d126a3 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/diodes/led.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// exhibit exponential current-voltage relationship with an added
/// optical output proportional to forward current.

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the LedDiode structure.
pub struct LedDiode<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    saturation_current: f32,
    thermal_voltage: f32,
    series_resistance: f32,
    forward_voltage: f32,
    optical_output: f32,
}

impl<'a, Node: WdfNode> LedDiode<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node) -> Self {
        Self {
            node,
            saturation_current: 8e-20,
            thermal_voltage: 0.026,
            series_resistance: 10.0,
            forward_voltage: 2.0,
            optical_output: 0.0,
        }
    }

    /// Technical implementation of the with_color logic.
    pub fn with_color(node: &'a mut Node, color: LedColor) -> Self {
        let v_f = match color {
            LedColor::Red => 1.8,
            LedColor::Green => 2.2,
            LedColor::Blue => 3.2,
            LedColor::White => 3.0,
            LedColor::Infrared => 1.2,
        };
        Self {
            node,
            saturation_current: 8e-20,
            thermal_voltage: 0.026,
            series_resistance: 10.0,
            forward_voltage: v_f,
            optical_output: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        let vt = self.thermal_voltage;
        let i_s = self.saturation_current;
        let rs = self.series_resistance;
        let v_f = self.forward_voltage;

        let mut v_d = incident.max(0.0);

        for _ in 0..4 {
            let exp_term = exp_approx((v_d - v_f) / vt);
            let i_f = i_s * exp_term;
            let residual = v_d + rs * i_f - incident;
            let jacobian = 1.0 + (rs * i_s / vt) * exp_term;
            v_d -= if jacobian.abs() > 1e-6 {
                residual / jacobian
            } else {
                residual
            };
            v_d = v_d.max(v_f);
        }

        let exp_term = exp_approx((v_d - v_f) / vt);
        let i_f = i_s * exp_term;
        self.optical_output = i_f * 0.1;
        let v_out = v_d - i_f * rs;

        let reflected = 2.0 * v_out - incident;
        self.node.wave_down(reflected);
        v_out
    }

    /// Technical implementation of the get_optical_output logic.
    pub fn get_optical_output(&self) -> f32 {
        self.optical_output
    }
}

/// Technical implementation of the LedColor enumeration.
pub enum LedColor {
    Red,
    Green,
    Blue,
    White,
    Infrared,
}
