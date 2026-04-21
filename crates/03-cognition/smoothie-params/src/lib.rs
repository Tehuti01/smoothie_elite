/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbe6eb6ce | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

/// High-performance atomic parameter wrappers for thread-safe access.
pub mod atomic;
/// Parameter bank orchestration for grouping and index-based access.
pub mod bank;
/// Metadata definitions for parameter range, units, and descriptors.
pub mod info;
/// Signal smoothing algorithms including linear and one-pole filtering.
pub mod smoothing;

pub use atomic::AtomicParameter;
pub use bank::ParameterBank;
pub use info::{ParameterInfo, ParameterRange, ParameterType, ParameterUnit};
pub use smoothing::{LinearSmoother, OnePoleSmoother};
