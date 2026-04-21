/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa901f4fc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/capacitor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;

/// Technical implementation of the CapacitorComponent structure.
pub struct CapacitorComponent {
    capacitance: f32,
    charge: f32,
    voltage: f32,
    sample_rate: f32,
}

impl CapacitorComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(capacitance: f32, sample_rate: f32) -> Self {
        Self {
            capacitance,
            charge: 0.0,
            voltage: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_capacitance logic.
    pub fn set_capacitance(&mut self, capacitance: f32) {
        self.capacitance = capacitance;
    }

    /// Technical implementation of the get_capacitance logic.
    pub fn get_capacitance(&self) -> f32 {
        self.capacitance
    }

    /// Technical implementation of the get_voltage logic.
    pub fn get_voltage(&self) -> f32 {
        self.voltage
    }

    /// Technical implementation of the charge_voltage logic.
    pub fn charge_voltage(&mut self, voltage: f32) {
        self.voltage = voltage;
        self.charge = self.capacitance * voltage;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, current_in: f32) -> f32 {
        let dt = 1.0 / self.sample_rate;
        self.charge += current_in * dt;
        self.voltage = self.charge / self.capacitance;
        self.voltage
    }
}

impl WdfNode for CapacitorComponent {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        1.0 / (2.0 * self.capacitance * self.sample_rate)
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.voltage
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.voltage = wave;
        self.charge = self.capacitance * wave;
    }
}
