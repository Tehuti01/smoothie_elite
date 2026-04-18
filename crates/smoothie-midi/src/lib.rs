//! # smoothie-midi
//!
//! MIDI 1.0, MPE, and MIDI 2.0 support for Smoothie Elite.

pub mod event;
pub mod buffer;

pub use event::{MidiEvent, MidiMessage, MidiChannel};
pub use buffer::MidiBuffer;

pub mod prelude {
    pub use crate::{MidiEvent, MidiMessage, MidiChannel, MidiBuffer};
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
