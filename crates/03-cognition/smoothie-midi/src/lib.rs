/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x455d874b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod cc;
pub mod clock;
pub mod io;
pub mod learn;
pub mod mpe;
pub mod notes;
pub mod rpn;

pub use cc::CcMapping;
pub use clock::MidiClock;
pub use learn::MidiLearn;
pub use mpe::MpeManager;
pub use notes::NoteTracker;
pub use notes::{note_to_frequency, velocity_to_amplitude};

pub const NUM_CHANNELS: usize = 16;
pub const NUM_NOTES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiMessage enumeration.
pub enum MidiMessage {
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    ControlChange {
        channel: u8,
        controller: u8,
        value: u8,
    },
    PitchBend {
        channel: u8,
        value: u16,
    },
    Aftertouch {
        channel: u8,
        note: u8,
        value: u8,
    },
    ChannelPressure {
        channel: u8,
        value: u8,
    },
    ProgramChange {
        channel: u8,
        program: u8,
    },
    SysEx {
        length: usize,
    },
    Clock,
    Start,
    Continue,
    Stop,
    ActiveSensing,
    Reset,
}

impl MidiMessage {
    /// Technical implementation of the parse logic.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }
        let status = bytes[0];

        if status >= 0xF8 {
            return match status {
                0xF8 => Some(Self::Clock),
                0xFA => Some(Self::Start),
                0xFB => Some(Self::Continue),
                0xFC => Some(Self::Stop),
                0xFE => Some(Self::ActiveSensing),
                0xFF => Some(Self::Reset),
                _ => None,
            };
        }

        if bytes.len() < 2 {
            return None;
        }

        let channel = status & 0x0F;
        let command = status & 0xF0;

        match command {
            0x80 => {
                if bytes.len() >= 3 {
                    Some(Self::NoteOff {
                        channel,
                        note: bytes[1],
                        velocity: bytes[2],
                    })
                } else {
                    None
                }
            }
            0x90 => {
                if bytes.len() >= 3 {
                    if bytes[2] == 0 {
                        Some(Self::NoteOff {
                            channel,
                            note: bytes[1],
                            velocity: 0,
                        })
                    } else {
                        Some(Self::NoteOn {
                            channel,
                            note: bytes[1],
                            velocity: bytes[2],
                        })
                    }
                } else {
                    None
                }
            }
            0xA0 => {
                if bytes.len() >= 3 {
                    Some(Self::Aftertouch {
                        channel,
                        note: bytes[1],
                        value: bytes[2],
                    })
                } else {
                    None
                }
            }
            0xB0 => {
                if bytes.len() >= 3 {
                    Some(Self::ControlChange {
                        channel,
                        controller: bytes[1],
                        value: bytes[2],
                    })
                } else {
                    None
                }
            }
            0xC0 => Some(Self::ProgramChange {
                channel,
                program: bytes[1],
            }),
            0xD0 => Some(Self::ChannelPressure {
                channel,
                value: bytes[1],
            }),
            0xE0 => {
                if bytes.len() >= 3 {
                    let value = (bytes[1] as u16) | ((bytes[2] as u16) << 7);
                    Some(Self::PitchBend { channel, value })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Technical implementation of the channel logic.
    pub fn channel(&self) -> Option<u8> {
        match self {
            Self::NoteOn { channel, .. } => Some(*channel),
            Self::NoteOff { channel, .. } => Some(*channel),
            Self::ControlChange { channel, .. } => Some(*channel),
            Self::PitchBend { channel, .. } => Some(*channel),
            Self::Aftertouch { channel, .. } => Some(*channel),
            Self::ChannelPressure { channel, .. } => Some(*channel),
            Self::ProgramChange { channel, .. } => Some(*channel),
            _ => None,
        }
    }
}
