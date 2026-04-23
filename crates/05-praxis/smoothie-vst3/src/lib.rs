/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1aa0006d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-vst3/src/lib.rs                                                         │
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
pub mod com;
pub mod component;
pub mod entry;
pub mod plugin_entry;
pub mod types;

pub use category::Vst3Category;
pub use com::{FUnknownImpl, FUnknownVTable, IID, TUID};
pub use component::{AudioProcessor, EditController, Vst3AudioProcessor, Vst3EditController};
pub use plugin_entry::{Vst3ComponentFlags, Vst3PluginEntry, Vst3ProcessorInfo};
pub use types::*;
