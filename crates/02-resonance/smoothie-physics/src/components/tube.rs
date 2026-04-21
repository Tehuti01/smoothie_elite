/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5551bc22 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/tube.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;
use smoothie_core::math::FloatExt;

/// Technical implementation of the TubeComponent structure.
pub struct TubeComponent {
    tube_type: TubeType,
    mu: f32,
    ex: f32,
    kg1: f32,
    kp: f32,
    kvb: f32,
    grid_current: f32,
    plate_current: f32,
    screen_current: f32,
    state_g: f32,
    state_p: f32,
    state_s: f32,
    state_k: f32,
}

#[derive(Clone, Copy)]
/// Technical implementation of the TubeType enumeration.
pub enum TubeType {
    Triode12AX7,
    Triode12AT7,
    Triode12AU7,
    PentodeEL34,
    PentodeEL84,
    Tetrode6L6,
    Tetrode6V6,
    Beam6550,
}

impl TubeComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(tube_type: TubeType) -> Self {
        let (mu, ex, kg1, kp, kvb) = match tube_type {
            TubeType::Triode12AX7 => (100.0, 1.4, 1060.0, 600.0, 300.0),
            TubeType::Triode12AT7 => (70.0, 1.35, 1100.0, 550.0, 280.0),
            TubeType::Triode12AU7 => (100.0, 1.4, 1050.0, 580.0, 290.0),
            TubeType::PentodeEL34 => (11.5, 1.35, 280.0, 300.0, 250.0),
            TubeType::PentodeEL84 => (58.0, 1.5, 400.0, 400.0, 200.0),
            TubeType::Tetrode6L6 => (7.8, 1.5, 550.0, 280.0, 220.0),
            TubeType::Tetrode6V6 => (8.5, 1.45, 450.0, 250.0, 200.0),
            TubeType::Beam6550 => (6.2, 1.55, 680.0, 320.0, 250.0),
        };
        Self {
            tube_type,
            mu,
            ex,
            kg1,
            kp,
            kvb,
            grid_current: 0.0,
            plate_current: 0.0,
            screen_current: 0.0,
            state_g: 0.0,
            state_p: 0.0,
            state_s: 0.0,
            state_k: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(
        &mut self,
        v_g: f32,
        v_p: f32,
        v_s: f32,
        v_k: f32,
        r_g: f32,
        r_p: f32,
        r_s: f32,
        r_k: f32,
    ) {
        let mu = self.mu;
        let ex = self.ex;
        let kg1 = self.kg1;
        let kp = self.kp;
        let kvb = self.kvb;

        let v_gk = v_g - v_k;
        let v_pk = v_p - v_k;
        let v_sk = v_s - v_k;

        let mut i_p = 0.0;
        let mut i_s = 0.0;

        for _ in 0..4 {
            let kv_term = kvb + v_pk * v_pk;
            let sqrt_kv = fast_sqrt(kv_term);

            let exp_inner = kp * (1.0 / mu + v_gk / sqrt_kv.max(0.1));
            let e1 = (v_pk / kp) * fast_ln(1.0 + exp_approx(exp_inner.clamp(-10.0, 10.0)));

            if e1 > 0.0 {
                i_p = (e1 * fast_sqrt(e1)) / kg1;
            }

            if v_sk > 0.0 {
                i_s = 0.001 * v_sk * v_sk;
            }
        }

        self.grid_current = v_gk.max(0.0) * 0.001;
        self.plate_current = i_p;
        self.screen_current = i_s;

        self.state_g = v_g - self.grid_current * r_g;
        self.state_p = v_p - self.plate_current * r_p;
        self.state_s = v_s - self.screen_current * r_s;
        self.state_k = v_k + (self.grid_current + self.plate_current + self.screen_current) * r_k;
    }

    /// Technical implementation of the get_plate_current logic.
    pub fn get_plate_current(&self) -> f32 {
        self.plate_current
    }

    /// Technical implementation of the get_grid_current logic.
    pub fn get_grid_current(&self) -> f32 {
        self.grid_current
    }
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.1;
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
