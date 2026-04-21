/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5b1347ec | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-preset/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod bank;
pub mod blob;
pub mod diff;
pub mod ironstack_presets;
pub mod snapshot;

pub use bank::{PresetBank, PresetEntry};
pub use blob::{PresetBlob, PresetHeader, PRESET_FORMAT_VERSION, PRESET_MAGIC};
pub use diff::PresetDiff;
pub use ironstack_presets::init_ironstack_factory_bank;
pub use snapshot::{PresetSnapshot, MAX_PARAMS};
