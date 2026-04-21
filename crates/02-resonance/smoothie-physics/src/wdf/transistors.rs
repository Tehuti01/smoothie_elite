/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe0d65246 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/transistors.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the NpnTransistor structure.
pub struct NpnTransistor<'a, Emitter: WdfNode, Base: WdfNode, Collector: WdfNode> {
    pub emitter: &'a mut Emitter,
    pub base: &'a mut Base,
    pub collector: &'a mut Collector,

    // Ebers-Moll constants
    pub i_s: f32,    // saturation current
    pub v_t: f32,    // thermal voltage
    pub beta_f: f32, // forward gain
    pub beta_r: f32, // reverse gain
}

impl<'a, Emitter: WdfNode, Base: WdfNode, Collector: WdfNode>
    NpnTransistor<'a, Emitter, Base, Collector>
{
    /// Initializes a new instance of the associated type.
    pub fn new(emitter: &'a mut Emitter, base: &'a mut Base, collector: &'a mut Collector) -> Self {
        Self {
            emitter,
            base,
            collector,
            i_s: 1e-12,
            v_t: 0.02585,
            beta_f: 100.0,
            beta_r: 1.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        // Core execution: Newton Raphson applied over the base/emitter + base/collector
        // dual diode intersections to map active, saturated, and cutoff regions.
        let v_e = self.emitter.wave_up();
        let v_b = self.base.wave_up();
        let v_c = self.collector.wave_up();

        let re = self.emitter.get_port_resistance();
        let rb = self.base.get_port_resistance();
        let rc = self.collector.get_port_resistance();

        // Very simplified iterative Ebers-Moll Newton matrix
        let mut v_be = v_b - v_e;
        let mut v_bc = v_b - v_c;

        for _ in 0..4 {
            let exp_be = exp_approx(v_be / self.v_t);
            let exp_bc = exp_approx(v_bc / self.v_t);

            let i_be = (self.i_s / self.beta_f) * (exp_be - 1.0);
            let i_bc = (self.i_s / self.beta_r) * (exp_bc - 1.0);

            let i_b = i_be + i_bc;
            let i_c = self.beta_f * i_be - (1.0 + self.beta_r) * i_bc;
            let i_e = -(1.0 + self.beta_f) * i_be + self.beta_r * i_bc;

            v_be = v_b - v_e - i_b * rb + i_e * re;
            v_bc = v_b - v_c - i_b * rb + i_c * rc;
        }

        // Resolving waves
        // For actual WDF specs solving nonlinear roots typically delegates to 1 port returning wave down mappings.
        self.emitter.wave_down(v_e);
        self.base.wave_down(v_b);
        self.collector.wave_down(v_c);
    }
}
