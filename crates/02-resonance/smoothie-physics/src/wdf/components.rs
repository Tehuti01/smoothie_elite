/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1089e781 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/components.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the Resistor structure.
pub struct Resistor {
    resistance: f32,
}

impl Resistor {
    /// Initializes a new instance of the associated type.
    pub fn new(resistance: f32) -> Self {
        Self { resistance }
    }

    /// Technical implementation of the set_resistance logic.
    pub fn set_resistance(&mut self, res: f32) {
        self.resistance = res;
    }
}

impl WdfNode for Resistor {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }
    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        0.0
    }
    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, _wave: f32) {}
}

/// Technical implementation of the Capacitor structure.
pub struct Capacitor {
    resistance: f32, // Rp = 1 / (2 * C * Fs)
    state: f32,
}

impl Capacitor {
    /// Initializes a new instance of the associated type.
    pub fn new(capacitance: f32, sample_rate: f32) -> Self {
        Self {
            resistance: 1.0 / (2.0 * capacitance * sample_rate),
            state: 0.0,
        }
    }

    /// Technical implementation of the set_capacitance logic.
    pub fn set_capacitance(&mut self, capacitance: f32, sample_rate: f32) {
        self.resistance = 1.0 / (2.0 * capacitance * sample_rate);
    }
}

impl WdfNode for Capacitor {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.state
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.state = wave;
    }
}

/// Technical implementation of the Inductor structure.
pub struct Inductor {
    resistance: f32, // Rp = 2 * L * Fs
    state: f32,
}

impl Inductor {
    /// Initializes a new instance of the associated type.
    pub fn new(inductance: f32, sample_rate: f32) -> Self {
        Self {
            resistance: 2.0 * inductance * sample_rate,
            state: 0.0,
        }
    }
}

impl WdfNode for Inductor {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        -self.state
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.state = wave;
    }
}
