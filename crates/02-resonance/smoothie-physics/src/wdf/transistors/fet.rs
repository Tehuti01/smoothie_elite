/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xae41e6a9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/transistors/fet.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;
use smoothie_core::primitives::Sample;

/// Technical implementation of the NChannelFet structure.
pub struct NChannelFet<'a, Drain: WdfNode, Source: WdfNode, Gate: WdfNode> {
    pub drain: &'a mut Drain,
    pub source: &'a mut Source,
    pub gate: &'a mut Gate,

    k_n: f32,
    v_th: f32,
    lambda: f32,
    rdson: f32,
}

impl<'a, Drain: WdfNode, Source: WdfNode, Gate: WdfNode> NChannelFet<'a, Drain, Source, Gate> {
    /// Initializes a new instance of the associated type.
    pub fn new(drain: &'a mut Drain, source: &'a mut Source, gate: &'a mut Gate) -> Self {
        Self {
            drain,
            source,
            gate,
            k_n: 0.001,
            v_th: 2.0,
            lambda: 0.02,
            rdson: 1.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let v_d_inc = self.drain.wave_up();
        let v_s_inc = self.source.wave_up();
        let v_g_inc = self.gate.wave_up();

        let r_d = self.drain.get_port_resistance();
        let r_s = self.source.get_port_resistance();
        let r_g = self.gate.get_port_resistance();

        let v_ds = v_d_inc - v_s_inc;
        let v_gs = v_g_inc - v_s_inc;

        let i_d = if v_gs < self.v_th {
            0.0
        } else {
            let v_eff = v_gs - self.v_th;
            if v_ds < v_eff {
                self.k_n * (2.0 * v_eff * v_ds - v_ds * v_ds)
            } else {
                self.k_n * v_eff * v_eff * (1.0 + self.lambda * v_ds)
            }
        };

        let w_d_out = v_d_inc - 2.0 * r_d * i_d;
        let w_s_out = v_s_inc + 2.0 * r_s * i_d;
        let w_g_out = v_g_inc;

        self.drain.wave_down(w_d_out);
        self.source.wave_down(w_s_out);
        self.gate.wave_down(w_g_out);
    }
}

/// Technical implementation of the PChannelFet structure.
pub struct PChannelFet<'a, Drain: WdfNode, Source: WdfNode, Gate: WdfNode> {
    pub drain: &'a mut Drain,
    pub source: &'a mut Source,
    pub gate: &'a mut Gate,

    k_p: f32,
    v_th: f32,
    lambda: f32,
    rdson: f32,
}

impl<'a, Drain: WdfNode, Source: WdfNode, Gate: WdfNode> PChannelFet<'a, Drain, Source, Gate> {
    /// Initializes a new instance of the associated type.
    pub fn new(drain: &'a mut Drain, source: &'a mut Source, gate: &'a mut Gate) -> Self {
        Self {
            drain,
            source,
            gate,
            k_p: 0.0005,
            v_th: -2.0,
            lambda: 0.025,
            rdson: 2.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let v_d_inc = self.drain.wave_up();
        let v_s_inc = self.source.wave_up();
        let v_g_inc = self.gate.wave_up();

        let r_d = self.drain.get_port_resistance();
        let r_s = self.source.get_port_resistance();
        let r_g = self.gate.get_port_resistance();

        let v_sd = v_s_inc - v_d_inc;
        let v_sg = v_s_inc - v_g_inc;

        let i_d = if v_sg < -self.v_th {
            0.0
        } else {
            let v_eff = v_sg + self.v_th;
            if v_sd < v_eff {
                -self.k_p * (2.0 * v_eff * v_sd - v_sd * v_sd)
            } else {
                -self.k_p * v_eff * v_eff * (1.0 + self.lambda * v_sd)
            }
        };

        let w_d_out = v_d_inc - 2.0 * r_d * i_d;
        let w_s_out = v_s_inc + 2.0 * r_s * i_d;
        let w_g_out = v_g_inc;

        self.drain.wave_down(w_d_out);
        self.source.wave_down(w_s_out);
        self.gate.wave_down(w_g_out);
    }
}
