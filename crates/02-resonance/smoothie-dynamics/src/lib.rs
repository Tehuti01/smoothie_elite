/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3351f741 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod compressor;
pub mod detector;
pub mod gain_computer;
pub mod gate;
pub mod limiter;
pub mod transient;

pub use compressor::{Compressor, CompressorParams, CompressorStyle};
pub use detector::{DetectionMode, LevelDetector};
pub use gain_computer::GainComputer;
pub use gate::{Gate, GateParams};
pub use limiter::{Limiter, LimiterMode};
pub use transient::{TransientParams, TransientShaper};
