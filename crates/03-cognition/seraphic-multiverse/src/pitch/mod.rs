/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8202e5e5 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-multiverse/src/pitch/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod auto_pitch_quantizer;
pub mod formant_analyzer;
pub mod formant_shifter;
pub mod phase_vocoder_core;
pub mod pitch_tracker_yin;
pub mod vocal_doubler;

pub use auto_pitch_quantizer::PitchQuantizer;
pub use formant_analyzer::FormantAnalyzer;
pub use formant_shifter::FormantShifter;
pub use phase_vocoder_core::PhaseVocoder;
pub use pitch_tracker_yin::PitchTracker;
pub use vocal_doubler::VocalDoubler;
