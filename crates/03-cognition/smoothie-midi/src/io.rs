/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x002b75b0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/io.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::MidiMessage;

/// Maximum number of MIDI ports
pub const MAX_PORTS: usize = 16;
/// Maximum events per port per block
pub const MAX_EVENTS: usize = 128;

/// MIDI port direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the PortDirection enumeration.
pub enum PortDirection {
    Input,
    Output,
    Bidirectional,
}

/// MIDI port type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the PortType enumeration.
pub enum PortType {
    Usb,
    Virtual,
    Network,
    DIN, // 5-pin DIN
}

/// MIDI port information
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the PortInfo structure.
pub struct PortInfo {
    pub name: &'static str,
    pub direction: PortDirection,
    pub port_type: PortType,
    pub channel_mask: u16, // 16-bit mask for allowed channels
}

impl PortInfo {
    /// Initializes a new instance of the associated type.
    pub fn new(name: &'static str, direction: PortDirection) -> Self {
        Self {
            name,
            direction,
            port_type: PortType::Usb,
            channel_mask: 0xFFFF, // All channels by default
        }
    }

    /// Technical implementation of the with_type logic.
    pub fn with_type(mut self, port_type: PortType) -> Self {
        self.port_type = port_type;
        self
    }

    /// Technical implementation of the with_channel_mask logic.
    pub fn with_channel_mask(mut self, mask: u16) -> Self {
        self.channel_mask = mask;
        self
    }
}

/// MIDI event with timestamp
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the MidiEvent structure.
pub struct MidiEvent {
    pub message: MidiMessage,
    pub port: u8,
    pub timestamp_samples: u64,
}

impl MidiEvent {
    /// Initializes a new instance of the associated type.
    pub fn new(message: MidiMessage, port: u8, timestamp: u64) -> Self {
        Self {
            message,
            port,
            timestamp_samples: timestamp,
        }
    }
}

/// MIDI port handle (platform-specific implementation would use this trait)
pub trait MidiPort: Send + Sync {
    /// Technical implementation of the is_open logic.
    fn is_open(&self) -> bool;
    /// Technical implementation of the close logic.
    fn close(&mut self) -> Result<(), MidiError>;
}

/// MIDI input port
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the MidiInput structure.
pub struct MidiInput {
    port_info: PortInfo,
    channel_filter: u8,
    running_status: u8,
    buffer: [u8; 3],
    buffer_len: usize,
    open: bool,
}

impl MidiInput {
    /// Initializes a new instance of the associated type.
    pub fn new(info: PortInfo) -> Self {
        Self {
            port_info: info,
            channel_filter: 0xFF, // All channels
            running_status: 0,
            buffer: [0; 3],
            buffer_len: 0,
            open: false,
        }
    }

    /// Technical implementation of the open logic.
    pub fn open(&mut self) -> Result<(), MidiError> {
        if self.open {
            return Err(MidiError::AlreadyOpen);
        }
        // Platform-specific: would open real MIDI port here
        self.open = true;
        Ok(())
    }

    /// Technical implementation of the close logic.
    pub fn close(&mut self) -> Result<(), MidiError> {
        if !self.open {
            return Err(MidiError::NotOpen);
        }
        self.open = false;
        Ok(())
    }

    /// Technical implementation of the is_open logic.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Technical implementation of the set_channel_filter logic.
    pub fn set_channel_filter(&mut self, channel: u8) {
        self.channel_filter = channel;
    }

    /// Parse incoming raw MIDI bytes
    pub fn parse_bytes(&mut self, byte: u8) -> Option<MidiMessage> {
        // System real-time messages (can occur anytime)
        if byte >= 0xF8 {
            return MidiMessage::parse(&[byte]);
        }

        // Running status handling
        if byte < 0x80 && self.running_status != 0 {
            self.buffer[0] = self.running_status;
            self.buffer[1] = byte;
            self.buffer_len = 2;
            return None;
        }

        // New status byte
        if byte >= 0x80 {
            self.running_status = if byte < 0xF0 { byte } else { 0 };
            self.buffer[0] = byte;
            self.buffer_len = 1;
            return None;
        }

        // Data byte
        if self.buffer_len < 3 {
            self.buffer[self.buffer_len] = byte;
            self.buffer_len += 1;
        }

        // Try to parse complete message
        if self.buffer_len >= 2 {
            let msg = MidiMessage::parse(&self.buffer[..self.buffer_len]);
            if msg.is_some() {
                self.buffer_len = 0;
                return msg;
            }
        }

        None
    }

    /// Technical implementation of the info logic.
    pub fn info(&self) -> &PortInfo {
        &self.port_info
    }
}

/// MIDI output port
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the MidiOutput structure.
pub struct MidiOutput {
    port_info: PortInfo,
    running_status: u8,
    open: bool,
}

impl MidiOutput {
    /// Initializes a new instance of the associated type.
    pub fn new(info: PortInfo) -> Self {
        Self {
            port_info: info,
            running_status: 0,
            open: false,
        }
    }

    /// Technical implementation of the open logic.
    pub fn open(&mut self) -> Result<(), MidiError> {
        if self.open {
            return Err(MidiError::AlreadyOpen);
        }
        self.open = true;
        Ok(())
    }

    /// Technical implementation of the close logic.
    pub fn close(&mut self) -> Result<(), MidiError> {
        if !self.open {
            return Err(MidiError::NotOpen);
        }
        self.open = false;
        Ok(())
    }

    /// Technical implementation of the is_open logic.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Write a MIDI message (returns raw bytes)
    pub fn write_message(&mut self, msg: &MidiMessage) -> [u8; 3] {
        let mut bytes = [0u8; 3];
        let mut len = 0;

        match msg {
            MidiMessage::NoteOn {
                channel,
                note,
                velocity,
            } => {
                bytes[0] = 0x90 | (*channel & 0x0F);
                bytes[1] = *note;
                bytes[2] = *velocity;
                len = 3;
                self.running_status = bytes[0];
            }
            MidiMessage::NoteOff {
                channel,
                note,
                velocity,
            } => {
                bytes[0] = 0x80 | (*channel & 0x0F);
                bytes[1] = *note;
                bytes[2] = *velocity;
                len = 3;
                self.running_status = bytes[0];
            }
            MidiMessage::ControlChange {
                channel,
                controller,
                value,
            } => {
                bytes[0] = 0xB0 | (*channel & 0x0F);
                bytes[1] = *controller;
                bytes[2] = *value;
                len = 3;
                self.running_status = bytes[0];
            }
            MidiMessage::PitchBend { channel, value } => {
                bytes[0] = 0xE0 | (*channel & 0x0F);
                bytes[1] = (*value & 0x7F) as u8;
                bytes[2] = ((*value >> 7) & 0x7F) as u8;
                len = 3;
                self.running_status = bytes[0];
            }
            MidiMessage::ProgramChange { channel, program } => {
                bytes[0] = 0xC0 | (*channel & 0x0F);
                bytes[1] = *program;
                len = 2;
                self.running_status = bytes[0];
            }
            MidiMessage::Clock => {
                bytes[0] = 0xF8;
                len = 1;
            }
            MidiMessage::Start => {
                bytes[0] = 0xFA;
                len = 1;
            }
            MidiMessage::Stop => {
                bytes[0] = 0xFC;
                len = 1;
            }
            MidiMessage::Continue => {
                bytes[0] = 0xFB;
                len = 1;
            }
            _ => {}
        }

        if len < 3 {
            [bytes[0], bytes[1], 0]
        } else {
            bytes
        }
    }

    /// Technical implementation of the info logic.
    pub fn info(&self) -> &PortInfo {
        &self.port_info
    }
}

/// Technical implementation of the MidiIo structure.
pub struct MidiIo {
    inputs: [Option<MidiInput>; MAX_PORTS],
    outputs: [Option<MidiOutput>; MAX_PORTS],
    input_count: usize,
    output_count: usize,
}

impl MidiIo {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            inputs: [None; MAX_PORTS],
            outputs: [None; MAX_PORTS],
            input_count: 0,
            output_count: 0,
        }
    }

    /// Performs vector addition logic.
    pub fn add_input(&mut self, info: PortInfo) -> Result<u8, MidiError> {
        if self.input_count >= MAX_PORTS {
            return Err(MidiError::PortFull);
        }
        let id = self.input_count as u8;
        self.inputs[id as usize] = Some(MidiInput::new(info));
        self.input_count += 1;
        Ok(id)
    }

    /// Performs vector addition logic.
    pub fn add_output(&mut self, info: PortInfo) -> Result<u8, MidiError> {
        if self.output_count >= MAX_PORTS {
            return Err(MidiError::PortFull);
        }
        let id = self.output_count as u8;
        self.outputs[id as usize] = Some(MidiOutput::new(info));
        self.output_count += 1;
        Ok(id)
    }

    /// Technical implementation of the get_input logic.
    pub fn get_input(&self, id: u8) -> Option<&MidiInput> {
        if (id as usize) < MAX_PORTS {
            self.inputs[id as usize].as_ref()
        } else {
            None
        }
    }

    /// Technical implementation of the get_input_mut logic.
    pub fn get_input_mut(&mut self, id: u8) -> Option<&mut MidiInput> {
        if (id as usize) < MAX_PORTS {
            self.inputs[id as usize].as_mut()
        } else {
            None
        }
    }

    /// Technical implementation of the get_output logic.
    pub fn get_output(&self, id: u8) -> Option<&MidiOutput> {
        if (id as usize) < MAX_PORTS {
            self.outputs[id as usize].as_ref()
        } else {
            None
        }
    }

    /// Technical implementation of the get_output_mut logic.
    pub fn get_output_mut(&mut self, id: u8) -> Option<&mut MidiOutput> {
        if (id as usize) < MAX_PORTS {
            self.outputs[id as usize].as_mut()
        } else {
            None
        }
    }

    /// Technical implementation of the input_count logic.
    pub fn input_count(&self) -> usize {
        self.input_count
    }
    /// Technical implementation of the output_count logic.
    pub fn output_count(&self) -> usize {
        self.output_count
    }
}

impl Default for MidiIo {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// MIDI errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiError enumeration.
pub enum MidiError {
    NotOpen,
    AlreadyOpen,
    PortFull,
    InvalidData,
    DeviceNotFound,
}

impl core::fmt::Display for MidiError {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MidiError::NotOpen => write!(f, "Port not open"),
            MidiError::AlreadyOpen => write!(f, "Port already open"),
            MidiError::PortFull => write!(f, "Maximum ports reached"),
            MidiError::InvalidData => write!(f, "Invalid MIDI data"),
            MidiError::DeviceNotFound => write!(f, "Device not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_midi_input_parse logic.
    fn test_midi_input_parse() {
        let mut input = MidiInput::new(PortInfo::new("Test", PortDirection::Input));

        // Note On
        let msg = input.parse_bytes(0x90);
        assert!(msg.is_none());
        let msg = input.parse_bytes(60);
        assert!(msg.is_none());
        let msg = input.parse_bytes(100);
        assert!(msg.is_some());
    }

    #[test]
    /// Technical implementation of the test_midi_io_add_ports logic.
    fn test_midi_io_add_ports() {
        let mut io = MidiIo::new();
        let id = io
            .add_input(PortInfo::new("Input 1", PortDirection::Input))
            .unwrap();
        assert_eq!(id, 0);
    }
}
