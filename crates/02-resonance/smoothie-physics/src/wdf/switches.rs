/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x64a041ab | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/switches.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the Switch structure.
pub struct Switch {
    closed: bool,
    inc_wave: f32,
}

impl Switch {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            closed: false,
            inc_wave: 0.0,
        }
    }
    /// Technical implementation of the toggle logic.
    pub fn toggle(&mut self) {
        self.closed = !self.closed;
    }
}

impl WdfNode for Switch {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        if self.closed {
            1e-9
        } else {
            1e9
        } // short circuit vs open circuit
    }
    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        if self.closed {
            -self.inc_wave
        } else {
            self.inc_wave
        }
    }
    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.inc_wave = wave;
    }
}

/// Technical implementation of the Vactrol structure.
pub struct Vactrol {
    resistance: f32,
    dark_resistance: f32,
    light_resistance: f32,
    slew: f32,
}

impl Vactrol {
    /// Initializes a new instance of the associated type.
    pub fn new(dark: f32, light: f32) -> Self {
        Self {
            resistance: dark,
            dark_resistance: dark,
            light_resistance: light,
            slew: 0.001,
        }
    }
    /// Technical implementation of the shine_led logic.
    pub fn shine_led(&mut self, intensity: f32) {
        let target =
            self.dark_resistance + intensity * (self.light_resistance - self.dark_resistance);
        self.resistance += (target - self.resistance) * self.slew;
    }
}

impl WdfNode for Vactrol {
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
