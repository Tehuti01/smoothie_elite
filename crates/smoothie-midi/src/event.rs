//! MIDI event types.

pub type MidiChannel = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MidiMessage {
    NoteOn  { channel: MidiChannel, note: u8, velocity: u8 },
    NoteOff { channel: MidiChannel, note: u8, velocity: u8 },
    ControlChange { channel: MidiChannel, controller: u8, value: u8 },
    PitchBend     { channel: MidiChannel, value: i16 },
    ProgramChange { channel: MidiChannel, program: u8 },
    Aftertouch    { channel: MidiChannel, note: u8, pressure: u8 },
    ChannelPressure { channel: MidiChannel, pressure: u8 },
    SysEx(u8),
}

impl MidiMessage {
    /// Parse from 3 raw MIDI bytes.
    pub fn from_bytes(bytes: [u8; 3]) -> Option<Self> {
        let status  = bytes[0];
        let channel = status & 0x0F;
        match status >> 4 {
            0x9 => Some(Self::NoteOn  { channel, note: bytes[1], velocity: bytes[2] }),
            0x8 => Some(Self::NoteOff { channel, note: bytes[1], velocity: bytes[2] }),
            0xB => Some(Self::ControlChange { channel, controller: bytes[1], value: bytes[2] }),
            0xE => {
                let value = ((bytes[2] as i16) << 7 | bytes[1] as i16) - 8192;
                Some(Self::PitchBend { channel, value })
            }
            0xC => Some(Self::ProgramChange { channel, program: bytes[1] }),
            0xA => Some(Self::Aftertouch { channel, note: bytes[1], pressure: bytes[2] }),
            0xD => Some(Self::ChannelPressure { channel, pressure: bytes[1] }),
            _   => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MidiEvent {
    /// Sample offset within the current processing block.
    pub sample_offset: u32,
    pub message: MidiMessage,
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
