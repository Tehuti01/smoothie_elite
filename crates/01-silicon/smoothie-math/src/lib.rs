/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x90dbde5e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

// Complex number arithmetic and phasor rotations.
pub mod complex;
// High-performance matrix and vector transformations.
pub mod matrix;
// Silicon-direct SIMD optimization layers (AVX/NEON).
pub mod simd;
// Spherical harmonics for spatial audio reconstruction.
pub mod spherical_harmonics;
// Statistical analysis and signal distribution metrics.
pub mod stats;
// Approximate trigonometric functions for performance-critical logic.
pub mod trig;

pub use smoothie_core::constants::{PHI, PHI_F64, PHI_INV, PHI_INV_F64, PI, TAU};
pub use smoothie_core::math::{FloatExt, PowiApprox};
