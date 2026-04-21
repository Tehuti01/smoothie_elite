/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x624eb329 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/ports.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub trait WdfNode {
    /// Get the port resistance.
    fn get_port_resistance(&self) -> f32;

    /// Calculate the reflected wave going up the tree.
    fn wave_up(&mut self) -> f32;

    /// Receive the incident wave coming down the tree.
    fn wave_down(&mut self, wave: f32);
}
