/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0b6a2f04 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-multiverse/src/synthesis/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod multiverse_env;
pub mod multiverse_lfo;
pub mod phase_distorter;
pub mod spectral_filter;
pub mod unison_engine;
pub mod wavetable_osc;

pub use multiverse_env::{EnvState, DistributedEnvironmentEnv};
pub use multiverse_lfo::{LfoMode, DistributedEnvironmentLfo};
pub use phase_distorter::{DistortionMode, PhaseDistorter};
pub use spectral_filter::SpectralFilter;
pub use unison_engine::UnisonEngine;
pub use wavetable_osc::WavetableOsc;
