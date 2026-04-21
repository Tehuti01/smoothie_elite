/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x36984a67 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/tubes.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the Triode12AX7 structure.
pub struct Triode12AX7<'a, GridPort: WdfNode, PlatePort: WdfNode, CathodePort: WdfNode> {
    pub grid: &'a mut GridPort,
    pub plate: &'a mut PlatePort,
    pub cathode: &'a mut CathodePort,

    // Physical Characteristics (Typical 12AX7 parameters)
    mu: f32,  // Amplification factor
    ex: f32,  // Exponent factor
    kg1: f32, // Perveance
    kp: f32,  // Plate characteristic
    kvb: f32, // Knee voltage
}

impl<'a, GridPort: WdfNode, PlatePort: WdfNode, CathodePort: WdfNode>
    Triode12AX7<'a, GridPort, PlatePort, CathodePort>
{
    /// Initializes a new instance of the associated type.
    pub fn new(
        grid: &'a mut GridPort,
        plate: &'a mut PlatePort,
        cathode: &'a mut CathodePort,
    ) -> Self {
        Self {
            grid,
            plate,
            cathode,
            mu: 100.0,
            ex: 1.4,
            kg1: 1060.0,
            kp: 600.0,
            kvb: 300.0,
        }
    }

    /// Primary solver for the non-linear triode junction.
    pub fn process(&mut self) {
        let v_g_inc = self.grid.wave_up();
        let v_p_inc = self.plate.wave_up();
        let v_k_inc = self.cathode.wave_up();

        let r_g = self.grid.get_port_resistance();
        let r_p = self.plate.get_port_resistance();
        let r_k = self.cathode.get_port_resistance();

        // ----------------------------------------------------
        // Nonlinear root-finding for Triode characteristics
        // (Simplified placeholder iterative solver for zero-latency)
        // ----------------------------------------------------

        let mut v_gk = v_g_inc - v_k_inc; // Grid-to-cathode voltage guess
        let mut v_pk = v_p_inc - v_k_inc; // Plate-to-cathode voltage guess

        // Newton-Raphson to solve Koren's Triode equation:
        // I_p = (E1^ex) / kg1 * (1 + sgn(E1))
        // where E1 = (V_pk / kp) * log(1 + exp(kp * (1/mu + V_gk / sqrt(kvb + V_pk^2))))

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
                // e1^1.5 approx
                i_p = (e1 * fast_sqrt(e1)) / self.kg1;
            }

            // Grid current (heuristic diode approximation)
            let i_g = if v_gk > 0.0 {
                (v_gk * v_gk) * 0.001
            } else {
                0.0
            };

            // Update voltages based on resistances
            v_pk = v_p_inc - v_k_inc - i_p * (r_p + r_k);
            v_gk = v_g_inc - v_k_inc - i_g * (r_g + r_k);
        }

        // Calculate reflected waves
        let i_p = (v_p_inc - v_pk - v_k_inc) / r_p;
        let i_g = (v_g_inc - v_gk - v_k_inc) / r_g;

        let w_g_out = v_g_inc - 2.0 * r_g * i_g;
        let w_p_out = v_p_inc - 2.0 * r_p * i_p;
        let w_k_out = v_k_inc + 2.0 * r_k * (i_p + i_g);

        self.grid.wave_down(w_g_out);
        self.plate.wave_down(w_p_out);
        self.cathode.wave_down(w_k_out);
    }
}

/// Technical implementation of the fast_inv_sqrt logic.
fn fast_inv_sqrt(x: f32) -> f32 {
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let mut y = f32::from_bits(i);
    y = y * (1.5 - xhalf * y * y);
    y
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    x * fast_inv_sqrt(x)
}

/// Technical implementation of the fast_ln logic.
fn fast_ln(x: f32) -> f32 {
    // simplified polynomial approx (just placeholder curve)
    if x <= 0.0 {
        return -10.0;
    }
    let y = (x - 1.0) / (x + 1.0);
    2.0 * y * (1.0 + (1.0 / 3.0) * y * y)
}
