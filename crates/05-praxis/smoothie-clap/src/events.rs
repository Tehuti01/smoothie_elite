/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3153084f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/events.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ClapEventHeader structure.
pub struct ClapEventHeader {
    /// Size of the entire event structure in bytes (including the header).
    pub size: u32,
    /// Sample-accurate timestamp within the current block.
    pub time: u32,
    /// Namespace identifier for the event type.
    pub space_id: u16,
    /// Event type discriminant.
    pub event_type: u16,
    /// Bitfield of transport flags.
    pub flags: u32,
}

/// Represents a Note On, Note Off, or Note Chord event.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ClapEventNote structure.
pub struct ClapEventNote {
    pub header: ClapEventHeader,
    /// Host-assigned voice/note identifier for polyphonic expression.
    pub note_id: i32,
    /// MIDI channel [0..15].
    pub port_index: i16,
    pub channel: i16,
    /// MIDI note number [0..127]. -1 means "all notes".
    pub key: i16,
    /// Normalised velocity [0.0, 1.0]. 0.0 = Note Off.
    pub velocity: f64,
}

/// Automation event: host is controlling a parameter value.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the ClapEventParamValue structure.
pub struct ClapEventParamValue {
    pub header: ClapEventHeader,
    /// The parameter ID being modulated.
    pub param_id: u32,
    /// Optional cookie set during `param_info` for fast lookup.
    pub cookie: u64,
    /// Polyphonic voice matching fields (-1 = global).
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    /// The new parameter value in plain (not normalised) units.
    pub value: f64,
}

/// Polyphonic modulation event (MPE / CLAP-native poly expression).
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ClapEventParamMod structure.
pub struct ClapEventParamMod {
    pub header: ClapEventHeader,
    pub param_id: u32,
    pub cookie: u64,
    pub note_id: i32,
    pub port_index: i16,
    pub channel: i16,
    pub key: i16,
    /// Additive modulation amount (signed, in plain units).
    pub amount: f64,
}

/// MIDI 1.0 raw event (for compatibility with legacy MIDI gear).
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ClapEventMidi structure.
pub struct ClapEventMidi {
    pub header: ClapEventHeader,
    pub port_index: u16,
    pub data: [u8; 3],
}

/// MIDI SysEx blob event.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ClapEventMidiSysex structure.
pub struct ClapEventMidiSysex {
    pub header: ClapEventHeader,
    pub port_index: u16,
    /// Pointer to SysEx data buffer (host-owned, valid only for the duration of `process()`).
    pub buffer: *const u8,
    pub size: u32,
}

// Safety: The `buffer` pointer is only valid during a `process()` call and is
// never stored across block boundaries. Usage is restricted to within `process()`.
unsafe impl Send for ClapEventMidiSysex {}
