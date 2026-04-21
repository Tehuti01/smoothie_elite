/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd8c42327 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod bands;
pub mod eq;
pub mod filters;
pub mod graphic_eq;
pub mod linear_phase;
pub mod para_quad;
pub mod shelves;

pub use bands::BandType;
pub use eq::{EqBand, EqBandConfig, ParametricEq};
pub use filters::{BiquadCoeffs, BiquadFilter, BiquadState};
pub use graphic_eq::{GraphicEq, ISO_1_3_OCTAVE_FREQS, NUM_GRAPHIC_BANDS};
pub use para_quad::{ParaBandConfig, ParaEq, QuadEq, MAX_PARA_BANDS};
pub use shelves::{HighShelf, LowShelf, TiltEq};
