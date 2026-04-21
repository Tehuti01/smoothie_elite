/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x389fe6af | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/sources.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the IdealVoltageSource structure.
pub struct IdealVoltageSource {
    voltage: f32,
    inc_wave: f32,
}

impl IdealVoltageSource {
    /// Initializes a new instance of the associated type.
    pub fn new(voltage: f32) -> Self {
        Self {
            voltage,
            inc_wave: 0.0,
        }
    }

    /// Technical implementation of the set_voltage logic.
    pub fn set_voltage(&mut self, target: f32) {
        self.voltage = target;
    }
}

impl WdfNode for IdealVoltageSource {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        1e-9
    } // near zero for ideal, to avoid division errors in trees
    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        2.0 * self.voltage - self.inc_wave
    }
    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.inc_wave = wave;
    }
}

/// Technical implementation of the ResistiveVoltageSource structure.
pub struct ResistiveVoltageSource {
    voltage: f32,
    resistance: f32,
}

impl ResistiveVoltageSource {
    /// Initializes a new instance of the associated type.
    pub fn new(voltage: f32, resistance: f32) -> Self {
        Self {
            voltage,
            resistance,
        }
    }
}

impl WdfNode for ResistiveVoltageSource {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }
    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.voltage
    }
    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, _wave: f32) {} // Absorbs cleanly
}

/// Technical implementation of the IdealCurrentSource structure.
pub struct IdealCurrentSource {
    current: f32,
    inc_wave: f32,
}

impl IdealCurrentSource {
    /// Initializes a new instance of the associated type.
    pub fn new(current: f32) -> Self {
        Self {
            current,
            inc_wave: 0.0,
        }
    }
}

impl WdfNode for IdealCurrentSource {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        1e9
    } // infinite resistance placeholder
    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.inc_wave + 2.0 * self.get_port_resistance() * self.current
    }
    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.inc_wave = wave;
    }
}
