/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xab827eaa | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/transistor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;

/// Technical implementation of the TransistorComponent structure.
pub struct TransistorComponent {
    #[allow(dead_code)]
    transistor_type: TransistorType,
    saturation_current: f32,
    thermal_voltage: f32,
    forward_gain: f32,
    reverse_gain: f32,
    emitter_state: f32,
    base_state: f32,
    collector_state: f32,
}

#[derive(Clone, Copy)]
/// Technical implementation of the TransistorType enumeration.
pub enum TransistorType {
    NPN,
    PNP,
}

impl TransistorComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(transistor_type: TransistorType) -> Self {
        Self {
            transistor_type,
            saturation_current: 1e-12,
            thermal_voltage: 0.02585,
            forward_gain: 100.0,
            reverse_gain: 1.0,
            emitter_state: 0.0,
            base_state: 0.0,
            collector_state: 0.0,
        }
    }

    /// Updates a framework parameter value.
    pub fn set_parameters(&mut self, is: f32, bf: f32, br: f32) {
        self.saturation_current = is;
        self.forward_gain = bf;
        self.reverse_gain = br;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, v_e: f32, v_b: f32, v_c: f32) -> (f32, f32, f32) {
        let vt = self.thermal_voltage;
        let is = self.saturation_current;
        let bf = self.forward_gain;
        let br = self.reverse_gain;

        let v_eb = v_e - v_b;
        let v_cb = v_c - v_b;

        let exp_eb = exp_approx(v_eb / vt);
        let exp_cb = exp_approx(v_cb / vt);

        let i_e = (is / bf) * (exp_eb - 1.0);
        let i_c = (is / br) * (exp_cb - 1.0);
        let i_b = i_e + i_c;

        self.emitter_state = -i_e;
        self.base_state = i_b;
        self.collector_state = i_c;

        (self.emitter_state, self.base_state, self.collector_state)
    }
}

impl WdfNode for TransistorComponent {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        1000.0
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.base_state
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.base_state = wave;
    }
}
