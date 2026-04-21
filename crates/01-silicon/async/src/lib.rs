/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9956abd9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/async/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod channel;
pub mod executor;
pub mod future;
pub mod task;
pub mod timer;
pub mod waker;

pub use channel::{channel, mpsc};
pub use core::future::Future;
pub use executor::{block_on, spawn};
pub use task::Task;
pub use timer::sleep;
pub use waker::TaskWaker;

/// Runtime version
pub const VERSION: &str = "1.0.0";
