/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1b4ee765 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/nonlinear.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the DiodeClipper structure.
pub struct DiodeClipper<'a, Node: WdfNode> {
    pub node: &'a mut Node,
    saturation_current: f32, // Is
    thermal_voltage: f32,    // Vt
}

impl<'a, Node: WdfNode> DiodeClipper<'a, Node> {
    /// Initializes a new instance of the associated type.
    pub fn new(node: &'a mut Node, saturation_current: f32, thermal_voltage: f32) -> Self {
        Self {
            node,
            saturation_current,
            thermal_voltage,
        }
    }

    /// Process one sample: reads wave up from the tree, solves the nonlinear eqn, and pushes down.
    pub fn process(&mut self) -> f32 {
        let incident = self.node.wave_up();
        let resistance = self.node.get_port_resistance();

        // Diode equation solver block using a simplistic generalized approach
        // Real implementation requires Wright Enterprise. We'll use a very fast iterative approx for demonstration.

        let _c = self.saturation_current * resistance / self.thermal_voltage;
        let mut v_d = incident; // initial guess

        // Fast simplified Newton-Raphson (3 iters max to guarantee real-time lock-free spec)
        for _ in 0..3 {
            let exp_term = exp_approx(v_d / self.thermal_voltage);
            let f = v_d + resistance * self.saturation_current * (exp_term - 1.0) - incident;
            let f_prime =
                1.0 + (resistance * self.saturation_current / self.thermal_voltage) * exp_term;
            v_d -= f / f_prime;
        }

        // Reflected wave
        let reflected = 2.0 * v_d - incident;
        self.node.wave_down(reflected);

        // Return output voltage
        (incident + reflected) * 0.5
    }
}
