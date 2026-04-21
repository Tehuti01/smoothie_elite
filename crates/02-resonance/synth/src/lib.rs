/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x31100eba | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod fm_synth;
pub mod grain_synth;
pub mod ironstack_synth;
pub mod monophonic;
pub mod oscillator_sync;
pub mod polyphonic;
pub mod super_osc;
pub mod unison;
pub mod voice;
pub mod wavefolding;
pub mod wavetable_synth;

pub use fm_synth::FMSynth;
pub use grain_synth::GrainSynth;
pub use ironstack_synth::IronStackPolySynth;
pub use monophonic::MonophonicSynth;
pub use oscillator_sync::{SyncConfig, SyncType, SyncedOscillator};
pub use polyphonic::PolyphonicSynth;
pub use super_osc::{SuperOsc, SuperOscConfig};
pub use unison::UnisonStack;
pub use voice::{Voice, VoiceState};
pub use wavefolding::{WavefoldConfig, WavefoldOsc};
pub use wavetable_synth::WavetableSynth;
