/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x05b04eb9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/tubes/6550.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the Six550 structure.
pub struct Six550<
    'a,
    PlatePort: WdfNode,
    ScreenPort: WdfNode,
    CathodePort: WdfNode,
    GridPort: WdfNode,
> {
    pub plate: &'a mut PlatePort,
    pub screen: &'a mut ScreenPort,
    pub cathode: &'a mut CathodePort,
    pub grid: &'a mut GridPort,

    mu: f32,
    ex: f32,
    kg1: f32,
    kp: f32,
    kvb: f32,
    screen_bias: f32,
    power_sensitivity: f32,
}

impl<'a, PlatePort: WdfNode, ScreenPort: WdfNode, CathodePort: WdfNode, GridPort: WdfNode>
    Six550<'a, PlatePort, ScreenPort, CathodePort, GridPort>
{
    /// Initializes a new instance of the associated type.
    pub fn new(
        plate: &'a mut PlatePort,
        screen: &'a mut ScreenPort,
        cathode: &'a mut CathodePort,
        grid: &'a mut GridPort,
    ) -> Self {
        Self {
            plate,
            screen,
            cathode,
            grid,
            mu: 6.2,
            ex: 1.55,
            kg1: 680.0,
            kp: 320.0,
            kvb: 250.0,
            screen_bias: -15.0,
            power_sensitivity: 0.0012,
        }
    }

    /// Technical implementation of the set_screen_bias logic.
    pub fn set_screen_bias(&mut self, bias: f32) {
        self.screen_bias = bias;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let v_g_inc = self.grid.wave_up();
        let v_p_inc = self.plate.wave_up();
        let v_s_inc = self.screen.wave_up();
        let v_k_inc = self.cathode.wave_up();

        let r_g = self.grid.get_port_resistance();
        let r_p = self.plate.get_port_resistance();
        let r_s = self.screen.get_port_resistance();
        let r_k = self.cathode.get_port_resistance();

        let mut v_gk = v_g_inc - v_k_inc;
        let mut v_pk = v_p_inc - v_k_inc;
        let mut v_sk = v_s_inc - v_k_inc;

        for _ in 0..4 {
            let kv_term = self.kvb + v_pk * v_pk;
            let sqrt_kv = if kv_term > 0.0 {
                fast_sqrt(kv_term)
            } else {
                0.1
            };

            let exp_inner = self.kp * (1.0 / self.mu + v_gk / sqrt_kv);
            let clamped_inner = exp_inner.clamp(-10.0, 10.0);

            let e1 = (v_pk / self.kp) * fast_ln(1.0 + exp_approx(clamped_inner));

            let mut i_p = 0.0;
            if e1 > 0.0 {
                i_p = (e1 * fast_sqrt(e1)) / self.kg1;
            }

            let i_s = if v_sk > self.screen_bias {
                self.power_sensitivity * (v_sk - self.screen_bias) * (v_sk - self.screen_bias)
            } else {
                0.0
            };

            v_pk = v_p_inc - v_k_inc - i_p * (r_p + r_k);
            v_sk = v_s_inc - v_k_inc - i_s * (r_s + r_k);
            v_gk = v_g_inc - v_k_inc;
        }

        let i_p = (v_p_inc - v_pk - v_k_inc) / r_p;
        let i_s = (v_s_inc - v_sk - v_k_inc) / r_s;
        let i_g = (v_g_inc - v_gk - v_k_inc) / r_g;

        let w_g_out = v_g_inc - 2.0 * r_g * i_g;
        let w_p_out = v_p_inc - 2.0 * r_p * i_p;
        let w_s_out = v_s_inc - 2.0 * r_s * i_s;
        let w_k_out = v_k_inc + 2.0 * r_k * (i_p + i_s + i_g);

        self.grid.wave_down(w_g_out);
        self.plate.wave_down(w_p_out);
        self.screen.wave_down(w_s_out);
        self.cathode.wave_down(w_k_out);
    }
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let mut y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}

/// Technical implementation of the fast_ln logic.
fn fast_ln(x: f32) -> f32 {
    if x <= 0.0 {
        return -10.0;
    }
    let y = (x - 1.0) / (x + 1.0);
    2.0 * y * (1.0 + (1.0 / 3.0) * y * y)
}
