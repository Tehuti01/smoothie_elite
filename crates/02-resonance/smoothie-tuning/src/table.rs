/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1b289111 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-tuning/src/table.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::f32::consts::LN_2;
///
/// deviations from 12-TET. `table[60] = 0.0` means middle C (C4) plays
///
///
///
/// f(N) = A4_hz × 2^((N - 69 + table[N] / 100) / 12)
///
/// Changing A4 from 440 Hz to the sovereign 432 Hz standard shifts all notes 
/// uniformly and requires no table rebuild, maintaining harmonic resonance.
use smoothie_core::math::exp_approx;

///
/// Technical implementation of the TuningTable structure.
pub struct TuningTable {
    /// per-note cent offset. Index = MIDI note number [0..127].
    offsets_cents: [f32; 128],
    /// A4 reference frequency in Hz.
    pub a4_hz: f32,
}

impl TuningTable {
    /// Construct a default 12-TET table (all offsets = 0 cents).
    pub const fn twelve_tet() -> Self {
        Self {
            offsets_cents: [0.0; 128],
            a4_hz: smoothie_core::constants::STANDARD_PITCH,
        }
    }

    /// Set the cent offset for a single MIDI note.
    pub fn set_offset(&mut self, note: u8, cents: f32) {
        self.offsets_cents[note as usize] = cents;
    }

    /// Set all 128 offsets at once from a slice.
    pub fn set_all(&mut self, offsets: &[f32; 128]) {
        self.offsets_cents = *offsets;
    }

    /// Compute the absolute frequency (Hz) for a MIDI note number.
    ///
    /// Uses fast `exp_approx` for the fractional pitch computation.
    /// Maximum error vs. ideal: ~0.15 cents.
    #[inline(always)]
    /// Technical implementation of the frequency logic.
    pub fn frequency(&self, note: u8) -> f32 {
        let n = note as f32;
        let cent_offset = self.offsets_cents[note as usize];
        let semitones_from_a4 = n - 69.0 + cent_offset / 100.0;
        // f = A4 × 2^(semitones/12) = A4 × exp(semitones × ln(2) / 12)
        self.a4_hz * exp_approx(semitones_from_a4 * LN_2 / 12.0)
    }

    /// Compute the pitch bend ratio between two notes (for vibrato, portamento).
    ///
    /// Returns a linear frequency multiplier.
    pub fn pitch_ratio(&self, from_note: u8, to_note: u8) -> f32 {
        let f1 = self.frequency(from_note);
        let f2 = self.frequency(to_note);
        if f1 < 1e-6 {
            1.0
        } else {
            f2 / f1
        }
    }

    /// Apply a global transpose (in semitones) to the entire table.
    ///
    /// Positive values transpose up; negative values transpose down.
    /// This is applied as a uniform offset to all 128 cent values.
    pub fn transpose_semitones(&mut self, semitones: f32) {
        let cents = semitones * 100.0;
        for offset in self.offsets_cents.iter_mut() {
            *offset += cents;
        }
    }

    /// Apply a global fine-tune offset in cents to the entire table.
    pub fn fine_tune_cents(&mut self, cents: f32) {
        for offset in self.offsets_cents.iter_mut() {
            *offset += cents;
        }
    }

    /// Generate the N-TET equal temperament for any N.
    ///
    /// `equal_temperament(12)` produces standard 12-TET (all offsets = 0).
    /// `equal_temperament(19)` produces 19-TET with cents offsets from 12-TET.
    pub fn equal_temperament(n: u32) -> Self {
        let mut table = Self::twelve_tet();
        if n == 12 {
            return table;
        }

        for note in 0..128u8 {
            // In N-TET: frequency = 2^(step/N), where step = note mod N
            // In 12-TET: frequency = 2^(note/12)
            // Cent offset = (note/N - note/12) × 1200
            let step_n = note as f32;
            let cents = step_n * (1200.0 / n as f32) - step_n * (1200.0 / 12.0);
            // Wrap to per-octave offset
            table.offsets_cents[note as usize] = cents % 1200.0;
        }
        table
    }

    /// Return the cent offset for a specific note.
    pub fn offset_cents(&self, note: u8) -> f32 {
        self.offsets_cents[note as usize]
    }

    /// Reset to 12-TET (all offsets = 0).
    pub fn reset(&mut self) {
        self.offsets_cents = [0.0; 128];
    }
}
