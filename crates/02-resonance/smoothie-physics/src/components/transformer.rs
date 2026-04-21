/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5e3e8dd0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/transformer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::FloatExt;

/// Technical implementation of the TransformerComponent structure.
pub struct TransformerComponent {
    turns_ratio: f32,
    primary_inductance: f32,
    secondary_inductance: f32,
    saturation_flux: f32,
    coercivity: f32,
    remnance: f32,
    primary_flux: f32,
    secondary_flux: f32,
    primary_state: f32,
    secondary_state: f32,
}

impl TransformerComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(turns_ratio: f32, primary_l: f32, secondary_l: f32) -> Self {
        Self {
            turns_ratio,
            primary_inductance: primary_l,
            secondary_inductance: secondary_l,
            saturation_flux: 1.5,
            coercivity: 0.02,
            remnance: 0.8,
            primary_flux: 0.0,
            secondary_flux: 0.0,
            primary_state: 0.0,
            secondary_state: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, v_p: f32, v_s: f32) -> (f32, f32) {
        let ratio = self.turns_ratio;
        let lp = self.primary_inductance;
        let ls = self.secondary_inductance;

        let flux_change = (v_p / lp) * 0.001;
        self.primary_flux += flux_change;

        let sat_factor = if self.primary_flux.abs() > self.saturation_flux {
            self.saturation_flux / self.primary_flux.abs()
        } else {
            1.0
        };
        self.primary_flux *= sat_factor;

        let v_s_induced = self.primary_flux * ratio * 200.0;

        let i_p = v_p / lp;
        let i_s = v_s / ls;

        let v_p_out = v_p - i_p * 0.0;
        let v_s_out = v_s_induced - i_s * 0.0;

        self.primary_state = v_p_out;
        self.secondary_state = v_s_out;

        (v_p_out, v_s_out)
    }

    /// Technical implementation of the get_flux logic.
    pub fn get_flux(&self) -> f32 {
        self.primary_flux
    }
}

impl WdfNode for TransformerComponent {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.primary_inductance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.primary_state
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.primary_state = wave;
    }
}
