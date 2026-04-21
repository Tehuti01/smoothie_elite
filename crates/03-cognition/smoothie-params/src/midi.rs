/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc9d38343 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/midi.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicU16, AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiMessage enumeration.
pub enum MidiMessage {
    NoteOn { channel: u8, note: u8, velocity: u8 },
    NoteOff { channel: u8, note: u8, velocity: u8 },
    AfterTouch { channel: u8, note: u8, velocity: u8 },
    ControlChange { channel: u8, cc: u8, value: u8 },
    ProgramChange { channel: u8, program: u8 },
    PitchBend { channel: u8, value: u16 },
}

impl MidiMessage {
    /// Technical implementation of the from_bytes logic.
    pub fn from_bytes(data: [u8; 3]) -> Option<Self> {
        let status = data[0];
        let b1 = data[1];
        let b2 = data[2];
        let channel = (status & 0x0F) as u8;
        let status_type = status & 0xF0;

        match status_type {
            0x80 => Some(Self::NoteOff {
                channel,
                note: b1,
                velocity: b2,
            }),
            0x90 => Some(Self::NoteOn {
                channel,
                note: b1,
                velocity: b2,
            }),
            0xA0 => Some(Self::AfterTouch {
                channel,
                note: b1,
                velocity: b2,
            }),
            0xB0 => Some(Self::ControlChange {
                channel,
                cc: b1,
                value: b2,
            }),
            0xC0 => Some(Self::ProgramChange {
                channel,
                program: b1,
            }),
            0xE0 => Some(Self::PitchBend {
                channel,
                value: ((b2 as u16) << 7) | b1 as u16,
            }),
            _ => None,
        }
    }

    /// Technical implementation of the is_cc logic.
    pub fn is_cc(&self) -> bool {
        matches!(self, Self::ControlChange { .. })
    }

    /// Technical implementation of the channel logic.
    pub fn channel(&self) -> u8 {
        match self {
            Self::NoteOn { channel, .. }
            | Self::NoteOff { channel, .. }
            | Self::AfterTouch { channel, .. }
            | Self::ControlChange { channel, .. }
            | Self::ProgramChange { channel, .. }
            | Self::PitchBend { channel, .. } => *channel,
        }
    }

    /// Technical implementation of the cc_number logic.
    pub fn cc_number(&self) -> Option<u8> {
        match self {
            Self::ControlChange { cc, .. } => Some(*cc),
            _ => None,
        }
    }

    /// Technical implementation of the cc_value logic.
    pub fn cc_value(&self) -> Option<u8> {
        match self {
            Self::ControlChange { value, .. } => Some(*value),
            _ => None,
        }
    }
}

pub const MAX_MIDI_LEARN_SLOTS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiLearnState enumeration.
pub enum MidiLearnState {
    Inactive,
    WaitingForInput,
    Learning {
        param_index: usize,
    },
    Active {
        param_index: usize,
        channel: u8,
        cc: u8,
    },
}

/// Technical implementation of the MidiLearnSlot structure.
pub struct MidiLearnSlot {
    pub param_index: usize,
    pub channel: u8,
    pub cc: u8,
    pub min_value: f32,
    pub max_value: f32,
    pub enabled: bool,
}

impl MidiLearnSlot {
    /// Initializes a new instance of the associated type.
    pub const fn new(param_index: usize) -> Self {
        Self {
            param_index,
            channel: 0,
            cc: 0,
            min_value: 0.0,
            max_value: 1.0,
            enabled: true,
        }
    }

    /// Technical implementation of the map_value logic.
    pub fn map_value(&self, midi_value: u8) -> f32 {
        let normalized = midi_value as f32 / 127.0;
        self.min_value + normalized * (self.max_value - self.min_value)
    }
}

/// Technical implementation of the MidiLearnBank structure.
pub struct MidiLearnBank {
    slots: [Option<MidiLearnSlot>; MAX_MIDI_LEARN_SLOTS],
    count: usize,
    listening_param: Option<usize>,
}

impl MidiLearnBank {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            slots: [None; MAX_MIDI_LEARN_SLOTS],
            count: 0,
            listening_param: None,
        }
    }

    /// Technical implementation of the start_learning logic.
    pub fn start_learning(&mut self, param_index: usize) {
        self.listening_param = Some(param_index);
    }

    /// Technical implementation of the cancel_learning logic.
    pub fn cancel_learning(&mut self) {
        self.listening_param = None;
    }

    /// Technical implementation of the receive_midi logic.
    pub fn receive_midi(&mut self, message: MidiMessage) -> Option<f32> {
        if let Some(param_idx) = self.listening_param {
            if let Some(cc) = message.cc_number() {
                let slot = MidiLearnSlot::new(param_idx);
                let mut idx = self.count;
                if idx < MAX_MIDI_LEARN_SLOTS {
                    self.slots[idx] = Some(slot);
                    self.slots[idx].as_mut().unwrap().cc = cc;
                    self.slots[idx].as_mut().unwrap().channel = message.channel();
                    self.count += 1;
                    idx = self.count;
                }
                self.listening_param = None;
                return Some(slot.map_value(message.cc_value().unwrap_or(64)));
            }
        }

        for slot in self.slots.iter_mut() {
            if let Some(ref mut s) = slot {
                if s.enabled
                    && s.cc == message.cc_number().unwrap_or(255)
                    && s.channel == message.channel()
                {
                    return Some(s.map_value(message.cc_value().unwrap_or(64)));
                }
            }
        }
        None
    }

    /// Technical implementation of the get_slot logic.
    pub fn get_slot(&self, index: usize) -> Option<&MidiLearnSlot> {
        self.slots.get(index).and_then(|s| s.as_ref())
    }

    /// Technical implementation of the remove_slot logic.
    pub fn remove_slot(&mut self, index: usize) {
        if let Some(slot) = self.slots[index].take() {
            self.slots[index] = None;
            self.count -= 1;
        }
    }

    /// Technical implementation of the enable_slot logic.
    pub fn enable_slot(&mut self, index: usize, enabled: bool) {
        if let Some(ref mut slot) = self.slots[index] {
            slot.enabled = enabled;
        }
    }

    /// Technical implementation of the slot_count logic.
    pub fn slot_count(&self) -> usize {
        self.count
    }

    /// Technical implementation of the is_learning logic.
    pub fn is_learning(&self) -> bool {
        self.listening_param.is_some()
    }
}

impl Default for MidiLearnBank {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_midi_message_parse logic.
    fn test_midi_message_parse() {
        let data = [0xB0, 0x01, 0x40];
        let msg = MidiMessage::from_bytes(data);
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert!(msg.is_cc());
    }

    #[test]
    /// Technical implementation of the test_midi_learn logic.
    fn test_midi_learn() {
        let mut bank = MidiLearnBank::new();
        bank.start_learning(0);
        let data = [0xB0, 0x01, 0x40];
        let msg = MidiMessage::from_bytes(data).unwrap();
        let value = bank.receive_midi(msg);
        assert!(value.is_some());
    }
}
