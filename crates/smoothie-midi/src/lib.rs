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
