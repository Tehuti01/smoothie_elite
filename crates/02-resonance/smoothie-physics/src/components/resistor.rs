/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb07a683c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/resistor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;

/// Technical implementation of the ResistorComponent structure.
pub struct ResistorComponent {
    resistance: f32,
    r_squared: f32,
}

impl ResistorComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(resistance: f32) -> Self {
        Self {
            resistance,
            r_squared: resistance * resistance,
        }
    }

    /// Technical implementation of the set_resistance logic.
    pub fn set_resistance(&mut self, resistance: f32) {
        self.resistance = resistance;
        self.r_squared = resistance * resistance;
    }

    /// Technical implementation of the get_resistance logic.
    pub fn get_resistance(&self) -> f32 {
        self.resistance
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&self, voltage: f32) -> f32 {
        voltage / self.resistance
    }
}

impl WdfNode for ResistorComponent {
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
