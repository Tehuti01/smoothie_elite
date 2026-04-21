/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3be9bc1e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod category;
pub mod descriptor;
pub mod events;
pub mod extensions;
pub mod host;
pub mod params;
pub mod plugin;
pub mod process;

pub use category::ClapPluginClassification;
pub use descriptor::{ClapDescriptor, features};
pub use events::{ClapEventHeader, ClapEventNote, ClapEventParamValue};
pub use plugin::SmoothieClapPlugin;
pub use process::{ClapAudioBuffer, ClapProcessContext};
