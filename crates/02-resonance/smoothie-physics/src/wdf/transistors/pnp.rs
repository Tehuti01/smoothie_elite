/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3a2a6d72 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/transistors/pnp.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the PnpTransistor structure.
pub struct PnpTransistor<'a, Emitter: WdfNode, Base: WdfNode, Collector: WdfNode> {
    pub emitter: &'a mut Emitter,
    pub base: &'a mut Base,
    pub collector: &'a mut Collector,

    i_s: f32,
    v_t: f32,
    beta_f: f32,
    beta_r: f32,
}

impl<'a, Emitter: WdfNode, Base: WdfNode, Collector: WdfNode>
    PnpTransistor<'a, Emitter, Base, Collector>
{
    /// Initializes a new instance of the associated type.
    pub fn new(emitter: &'a mut Emitter, base: &'a mut Base, collector: &'a mut Collector) -> Self {
        Self {
            emitter,
            base,
            collector,
            i_s: 1e-12,
            v_t: 0.02585,
            beta_f: 80.0,
            beta_r: 1.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let v_e = self.emitter.wave_up();
        let v_b = self.base.wave_up();
        let v_c = self.collector.wave_up();

        let re = self.emitter.get_port_resistance();
        let rb = self.base.get_port_resistance();
        let rc = self.collector.get_port_resistance();

        let mut v_eb = v_e - v_b;
        let mut v_cb = v_c - v_b;

        for _ in 0..4 {
            let exp_eb = exp_approx(v_eb / self.v_t);
            let exp_cb = exp_approx(v_cb / self.v_t);

            let i_e = (self.i_s / self.beta_f) * (exp_eb - 1.0);
            let i_c = (self.i_s / self.beta_r) * (exp_cb - 1.0);

            let i_b = i_e + i_c;
            let i_c_f = self.beta_f * i_e;
            let i_e_f = -(1.0 + self.beta_f) * i_e;

            v_eb = v_e - v_b + i_b * rb - i_e_f * re;
            v_cb = v_c - v_b + i_b * rb + i_c * rc;
        }

        let exp_eb = exp_approx(v_eb / self.v_t);
        let exp_cb = exp_approx(v_cb / self.v_t);
        let i_e = (self.i_s / self.beta_f) * (exp_eb - 1.0);
        let i_c = (self.i_s / self.beta_r) * (exp_cb - 1.0);

        self.emitter.wave_down(v_e);
        self.base.wave_down(v_b);
        self.collector.wave_down(v_c);
    }
}
