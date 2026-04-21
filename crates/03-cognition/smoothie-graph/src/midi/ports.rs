/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x12b50ff7 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/midi/ports.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use crate::port::PortDirection;
use alloc::string::String;

pub const MAX_CHANNELS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the PortType enumeration.
pub enum PortType {
    Audio,
    Midi,
    Control,
}

/// Technical implementation of the AudioChannel structure.
pub struct AudioChannel {
    pub index: usize,
    pub name: &'static str,
}

impl AudioChannel {
    /// Initializes a new instance of the associated type.
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            name: "Channels",
        }
    }

    /// Technical implementation of the with_name logic.
    pub const fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
}

/// Technical implementation of the MidiPort structure.
pub struct MidiPort {
    pub channel: u8,
    pub port_type: MidiPortType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiPortType enumeration.
pub enum MidiPortType {
    Note,
    Velocity,
    PitchBend,
    ModWheel,
    AfterTouch,
    Program,
    CC(u8),
}

impl MidiPort {
    /// Initializes a new instance of the associated type.
    pub const fn new(channel: u8) -> Self {
        Self {
            channel,
            port_type: MidiPortType::Note,
        }
    }

    /// Technical implementation of the note logic.
    pub const fn note(channel: u8) -> Self {
        Self {
            channel,
            port_type: MidiPortType::Note,
        }
    }

    /// Technical implementation of the cc logic.
    pub const fn cc(channel: u8, cc: u8) -> Self {
        Self {
            channel,
            port_type: MidiPortType::CC(cc),
        }
    }

    /// Technical implementation of the pitch_bend logic.
    pub const fn pitch_bend(channel: u8) -> Self {
        Self {
            channel,
            port_type: MidiPortType::PitchBend,
        }
    }
}

/// Technical implementation of the PortRegistry structure.
pub struct PortRegistry {
    audio_inputs: usize,
    audio_outputs: usize,
    midi_inputs: usize,
    midi_outputs: usize,
}

impl PortRegistry {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            audio_inputs: 0,
            audio_outputs: 0,
            midi_inputs: 0,
            midi_outputs: 0,
        }
    }

    /// Technical implementation of the register_audio_input logic.
    pub fn register_audio_input(&mut self) -> usize {
        let idx = self.audio_inputs;
        self.audio_inputs += 1;
        idx
    }

    /// Technical implementation of the register_audio_output logic.
    pub fn register_audio_output(&mut self) -> usize {
        let idx = self.audio_outputs;
        self.audio_outputs += 1;
        idx
    }

    /// Technical implementation of the register_midi_input logic.
    pub fn register_midi_input(&mut self) -> usize {
        let idx = self.midi_inputs;
        self.midi_inputs += 1;
        idx
    }

    /// Technical implementation of the register_midi_output logic.
    pub fn register_midi_output(&mut self) -> usize {
        let idx = self.midi_outputs;
        self.midi_outputs += 1;
        idx
    }

    /// Technical implementation of the audio_input_count logic.
    pub fn audio_input_count(&self) -> usize {
        self.audio_inputs
    }

    /// Technical implementation of the audio_output_count logic.
    pub fn audio_output_count(&self) -> usize {
        self.audio_outputs
    }

    /// Technical implementation of the midi_input_count logic.
    pub fn midi_input_count(&self) -> usize {
        self.midi_inputs
    }

    /// Technical implementation of the midi_output_count logic.
    pub fn midi_output_count(&self) -> usize {
        self.midi_outputs
    }
}

impl Default for PortRegistry {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_midi_port logic.
    fn test_midi_port() {
        let port = MidiPort::cc(0, 1);
        assert_eq!(port.channel, 0);
    }
}
