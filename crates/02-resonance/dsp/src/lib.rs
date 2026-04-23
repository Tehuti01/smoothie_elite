/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x43b66f90 | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/lib.rs                                 │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#![forbid(unsafe_code)]
extern crate alloc;
extern crate smoothie_core;

pub mod analysis;
pub mod autopan;
pub mod bitcrusher;
pub mod chorus;
pub mod compressor;
pub mod delay;
pub mod distortion;
pub mod envelope_mod;
pub mod exciter;
pub mod expander;
pub mod expansion;
pub mod fft;
pub mod filter;
pub mod filters;
pub mod flanger;
pub mod gate;
pub mod limiter;
pub mod oscillators;
pub mod overdrive;
pub mod phaser;
pub mod reverb;
pub mod ringmod;
pub mod tremolo;
pub mod vibrato;
pub mod wavetables;
pub mod widener;

pub mod prelude {
    pub use crate::filters::*;
    pub use crate::oscillators::*;
    pub use smoothie_core::ring_buffer::DelayLine;
}
