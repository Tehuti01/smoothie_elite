/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe98c2053 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/midi/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// to appropriate graph nodes. Each event carries a sample-accurate timestamp
/// so that the receiving node can schedule voice events with sub-block jitter.
use smoothie_core::ring_buffer::RingBuffer;

/// A 3-byte MIDI message.
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the MidiEvent structure.
pub struct MidiEvent {
    /// Sample offset within the current block (0 → block_size-1).
    pub sample_offset: u32,
    /// Raw MIDI status byte.
    pub status: u8,
    /// First data byte (note number for Note On/Off).
    pub data1: u8,
    /// Second data byte (velocity or controller value).
    pub data2: u8,
}

impl MidiEvent {
    /// Technical implementation of the note_on logic.
    pub fn note_on(offset: u32, channel: u8, note: u8, velocity: u8) -> Self {
        Self {
            sample_offset: offset,
            status: 0x90 | (channel & 0x0F),
            data1: note,
            data2: velocity,
        }
    }

    /// Technical implementation of the note_off logic.
    pub fn note_off(offset: u32, channel: u8, note: u8) -> Self {
        Self {
            sample_offset: offset,
            status: 0x80 | (channel & 0x0F),
            data1: note,
            data2: 0,
        }
    }

    /// Technical implementation of the control_change logic.
    pub fn control_change(offset: u32, channel: u8, controller: u8, value: u8) -> Self {
        Self {
            sample_offset: offset,
            status: 0xB0 | (channel & 0x0F),
            data1: controller,
            data2: value,
        }
    }

    /// Technical implementation of the pitch_bend logic.
    pub fn pitch_bend(offset: u32, channel: u8, lsb: u8, msb: u8) -> Self {
        Self {
            sample_offset: offset,
            status: 0xE0 | (channel & 0x0F),
            data1: lsb,
            data2: msb,
        }
    }

    /// Returns true if this is a Note On with non-zero velocity.
    #[inline(always)]
    /// Technical implementation of the is_note_on logic.
    pub fn is_note_on(&self) -> bool {
        (self.status & 0xF0) == 0x90 && self.data2 > 0
    }

    /// Returns true if this is a Note Off (or Note On with velocity = 0).
    #[inline(always)]
    /// Technical implementation of the is_note_off logic.
    pub fn is_note_off(&self) -> bool {
        (self.status & 0xF0) == 0x80 || ((self.status & 0xF0) == 0x90 && self.data2 == 0)
    }

    /// Returns MIDI channel index [0..15].
    #[inline(always)]
    /// Technical implementation of the channel logic.
    pub fn channel(&self) -> u8 {
        self.status & 0x0F
    }

    /// Compute absolute pitch frequency from MIDI note number using equal temperament.
    /// Tuning reference: A4 = 440 Hz.
    #[inline(always)]
    /// Technical implementation of the note_frequency logic.
    pub fn note_frequency(&self) -> f32 {
        // f = STANDARD_PITCH * 2^((note - 69) / 12)
        let semitones = self.data1 as i32 - 69;
        // Fast 2^x approximation for small integer exponents
        smoothie_core::constants::STANDARD_PITCH * fast_pow2_semitones(semitones)
    }
}

/// This avoids `libm::pow` by using pre-computed ratios for the 12-TET scale
/// combined with octave doubling.
fn fast_pow2_semitones(semitones: i32) -> f32 {
    const SEMITONE_RATIOS: [f32; 12] = [
        1.000000, 1.059463, 1.122462, 1.189207, 1.259921, 1.334840, 1.414214, 1.498307, 1.587401,
        1.681793, 1.781797, 1.887749,
    ];

    let octaves = semitones.div_euclid(12);
    let semi = semitones.rem_euclid(12) as usize;

    let ratio = SEMITONE_RATIOS[semi];

    // Multiply/divide by powers of two for octave shifts
    if octaves >= 0 {
        ratio * (1u32 << octaves.min(31) as u32) as f32
    } else {
        ratio / (1u32 << (-octaves).min(31) as u32) as f32
    }
}

/// Technical implementation of the MidiQueue structure.
pub struct MidiQueue {
    ring: RingBuffer<MidiEvent>,
}

impl MidiQueue {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            ring: RingBuffer::new(256),
        }
    }

    /// Push a new MIDI event from the host callback (non-realtime side).
    pub fn push(&mut self, event: MidiEvent) -> bool {
        self.ring.push(event)
    }

    /// Pop the next event from the audio thread side.
    pub fn pop(&mut self) -> Option<MidiEvent> {
        self.ring.pop()
    }
}
