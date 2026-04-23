/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa8f65d32 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/lib.rs                                                         │
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

pub mod compander;
pub mod compressor;
pub mod deesser;
pub mod delay;
pub mod distortion;
pub mod exciter;
pub mod gate;
pub mod imager;
pub mod limiter;
pub mod modulation;
pub mod multiband;
pub mod phaser;
pub mod pitch_shift;
pub mod reverb;
pub mod saturator;
pub mod shelving;
pub mod stereo;
pub mod time_stretch;
pub mod vintage;

pub use compander::{Compander, SplitCompander};
pub use compressor::Compressor;
pub use deesser::{DeEsser, DeesserMode};
pub use delay::DelayEffect;
pub use distortion::Distortion;
pub use exciter::{Exciter, PresenceBooster};
pub use gate::Gate;
pub use imager::{HaasEffect, MonoCompat, StereoImager};
pub use limiter::Limiter;
pub use modulation::Chorus;
pub use multiband::{Crossover, Multiband3, Multiband4, MultibandCombiner};
pub use phaser::Phaser;
pub use pitch_shift::{GranularPitchShift, PhaseVocoderPitchShift, PitchShift};
pub use reverb::ReverbEffect;
pub use saturator::{SaturationType, Saturator};
pub use shelving::{HighShelf, LowShelf, ShelfCurve, ShelvingEq};
pub use stereo::{
    AutoPan, ChannelSwap, MidSide, MonoStereo, StereoBalance, StereoCorrelation, StereoWidener,
    Tremolo,
};
pub use time_stretch::{ElastaneStretch, TimeStretch, WsolaStretcher};
pub use vintage::{MicPreamp, TapeEmulator, VintageEq};
