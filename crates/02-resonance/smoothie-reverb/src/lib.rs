/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x23ed85ec | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod diffuser;
pub mod early;
pub mod fdn;
pub mod predelay;
pub mod reverb;

pub use diffuser::AllPassDiffuser;
pub use early::EarlyReflections;
pub use fdn::{FdnOrder, FeedbackDelayNetwork};
pub use predelay::PreDelay;
pub use reverb::{Reverb, ReverbParams};
