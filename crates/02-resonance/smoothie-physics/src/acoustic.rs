/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0b57e82d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod acoustic_bowed;
pub mod acoustic_brass;
pub mod acoustic_drums;
pub mod acoustic_guitar;
pub mod acoustic_organ;
pub mod acoustic_piano;
pub mod acoustic_strings;
pub mod acoustic_woodwind;

pub use acoustic_bowed::AcousticBowed;
pub use acoustic_brass::AcousticBrass;
pub use acoustic_drums::AcousticDrums;
pub use acoustic_guitar::AcousticGuitar;
pub use acoustic_organ::AcousticOrgan;
pub use acoustic_piano::AcousticPiano;
pub use acoustic_strings::AcousticStrings;
pub use acoustic_woodwind::AcousticWoodwind;
