/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe6ff4158 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/inductor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;

/// Technical implementation of the InductorComponent structure.
pub struct InductorComponent {
    inductance: f32,
    flux: f32,
    current: f32,
    sample_rate: f32,
}

impl InductorComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(inductance: f32, sample_rate: f32) -> Self {
        Self {
            inductance,
            flux: 0.0,
            current: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_inductance logic.
    pub fn set_inductance(&mut self, inductance: f32) {
        self.inductance = inductance;
    }

    /// Technical implementation of the get_inductance logic.
    pub fn get_inductance(&self) -> f32 {
        self.inductance
    }

    /// Technical implementation of the get_current logic.
    pub fn get_current(&self) -> f32 {
        self.current
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, voltage_in: f32) -> f32 {
        let dt = 1.0 / self.sample_rate;
        self.current += (voltage_in / self.inductance) * dt;
        self.flux = self.inductance * self.current;
        self.current
    }
}

impl WdfNode for InductorComponent {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        2.0 * self.inductance * self.sample_rate
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        -self.current
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.current = -wave;
        self.flux = self.inductance * self.current;
    }
}
