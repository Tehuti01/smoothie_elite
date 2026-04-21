/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5dfc8590 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-tuning/src/mts.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::table::TuningTable;
use alloc::vec;
///
/// real-time — compatible with Surge XT, Dexed, OB-Xd, and any MTS-capable
///
///
/// - Byte 0: MIDI note (semitone)
/// - Byte 2: cents × 128/100 (LSB, 14-bit resolution)
/// # MTS Real-Time Single Note (SysEx F0 7F 7F 08 02)
/// Retunes a single note in real-time without interrupting playback.
use alloc::vec::Vec;

/// Technical implementation of the MtsMessage structure.
pub struct MtsMessage(pub Vec<u8>);

impl MtsMessage {
    /// Generate an MTS Non-Real-Time Bulk Tuning Dump SysEx for all 128 notes.
    ///
    /// The returned message can be sent via `clap.midi` to any compatible synth.
    pub fn bulk_dump(table: &TuningTable, device_id: u8) -> Self {
        let mut msg = Vec::with_capacity(408);

        // MTS Bulk Dump header
        msg.push(0xF0); // SysEx start
        msg.push(0x7E); // Non-real-time universal
        msg.push(device_id & 0x7F);
        msg.push(0x08); // Sub-ID: tuning
        msg.push(0x01); // Sub-ID2: bulk dump
                        // 3-character program name (zeroed)
        msg.extend_from_slice(b"   ");
        // 16-character name (zeroed)
        for _ in 0..16 {
            msg.push(0x20);
        }

        // 128 note tuning entries (3 bytes each)
        for note in 0u8..128 {
            let freq = table.frequency(note);
            let (nn, yy, zz) = freq_to_mts_bytes(freq);
            msg.push(nn);
            msg.push(yy);
            msg.push(zz);
        }

        // Checksum (XOR of all data bytes 1..end-1)
        let checksum = msg[1..].iter().fold(0u8, |acc, &b| acc ^ b) & 0x7F;
        msg.push(checksum);
        msg.push(0xF7); // SysEx end

        Self(msg)
    }

    /// Generate an MTS Real-Time Single Note Tuning Change.
    ///
    /// Tunes a single `note` to `target_hz` without affecting other notes.
    pub fn single_note(note: u8, target_hz: f32, device_id: u8) -> Self {
        let mut msg = Vec::with_capacity(12);
        msg.push(0xF0);
        msg.push(0x7F); // Real-time universal
        msg.push(device_id & 0x7F);
        msg.push(0x08); // Sub-ID: tuning
        msg.push(0x02); // Sub-ID2: single note
        msg.push(1); // Number of changes
        let (nn, yy, zz) = freq_to_mts_bytes(target_hz);
        msg.push(note & 0x7F);
        msg.push(nn);
        msg.push(yy);
        msg.push(zz);
        msg.push(0xF7);
        Self(msg)
    }

    /// Technical implementation of the bytes logic.
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

///
/// where `cents = (yy * 128 + zz) / 16384 * 100`.
fn freq_to_mts_bytes(freq_hz: f32) -> (u8, u8, u8) {
    if freq_hz <= 8.175 {
        return (0, 0, 0);
    }

    // Compute exact semitone position relative to MIDI note 0 (C-1 = 8.175 Hz at A4=440)
    let semitones_f = 69.0 + 12.0 * fast_log2(freq_hz / 440.0);
    let semitones_f = semitones_f.clamp(0.0, 127.9997);

    let nn_f = semitones_f;
    let nn = nn_f as u8;
    let frac = semitones_f - nn as f32;

    // Fractional semitone → 14-bit MTS encoding
    let cents_14bit = (frac * 100.0 / 100.0 * 16383.0) as u32;
    let yy = ((cents_14bit >> 7) & 0x7F) as u8;
    let zz = (cents_14bit & 0x7F) as u8;

    (nn & 0x7F, yy, zz)
}

/// Technical implementation of the fast_log2 logic.
fn fast_log2(x: f32) -> f32 {
    let n = x.to_bits();
    let exp = ((n >> 23) & 0xFF) as i32 - 127;
    let mantissa = f32::from_bits((n & 0x7FFFFF) | 0x3F800000) - 1.0;
    let log2e = core::f32::consts::LOG2_E;
    exp as f32 + mantissa * (1.0 - mantissa * 0.5) * log2e
}
