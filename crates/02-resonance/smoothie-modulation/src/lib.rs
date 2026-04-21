/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x347c7763 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

/// Aftertouch and Poly-Aftertouch modulation sources.
pub mod aftertouch;
/// Unit conversion and scaling utilities for modulation depths.
pub mod convert;
/// Modulation destination registry and handle management.
pub mod destinations;
/// DAHDSR envelope generators with configurable curves.
pub mod envelope;
/// Multi-shape, tempo-synced Low Frequency Oscillators (LFO).
pub mod lfo;
/// The central modulation matrix for source-to-destination routing.
pub mod matrix;

pub use aftertouch::{AftertouchMod, PolyAftertouch};
pub use convert::{bipolar_to_unipolar, convert, scale_depth, unipolar_to_bipolar};
pub use destinations::{ModDest, ModDestHandle, ModDestinations};
pub use envelope::{Envelope, EnvelopeParams, EnvelopeStage};
pub use lfo::{Lfo, LfoParams, LfoShape};
pub use matrix::{ModDestination, ModMatrix, ModRoute, ModSource};
