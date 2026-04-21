/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2346727e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod arena;
pub mod connection;
pub mod midi;
pub mod nodes;
pub mod port;
pub mod scheduler;

pub use arena::BufferArena;
pub use connection::ConnectionManager;
pub use nodes::{AudioNode, NodeConfig, NodeId};
pub use port::{AudioPort, PortDirection};
pub use scheduler::{GraphScheduler, ProcessOrder};
