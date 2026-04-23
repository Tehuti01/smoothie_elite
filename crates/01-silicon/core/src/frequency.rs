/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1c87a4d8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/frequency.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the pow2f logic.
fn pow2f(x: f32) -> f32 {
    let xi = if x >= 0.0 { x as i32 } else { x as i32 - 1 };
    let xf = x - xi as f32;
    let frac = 1.0 + xf * (core::f32::consts::LN_2 + xf * (0.2402265 + xf * 0.0558015));
    let exp_bits = ((xi + 127) as u32) << 23;
    f32::from_bits(exp_bits) * frac
}

#[inline]
/// Technical implementation of the log2f logic.
fn log2f(x: f32) -> f32 {
    if x <= 0.0 {
        -126.0
    } else {
        let bits = x.to_bits();
        let exp = ((bits >> 23) & 0xFF) as f32 - 127.0;
        let mantissa = f32::from_bits((bits & 0x7FFFFF) | 0x3F800000) - 1.0;
        exp + mantissa * (core::f32::consts::LOG2_E - 0.442_695 * mantissa)
    }
}

#[inline]
/// Technical implementation of the trunc_f32 logic.
fn trunc_f32(x: f32) -> f32 {
    let bits = x.to_bits();
    let sign = bits & 0x80000000;
    let exp = (bits >> 23) & 0xFF;
    if exp < 127 {
        return 0.0;
    }
    let mantissa = bits & 0x007FFFFF;
    let new_exp = exp - 127;
    f32::from_bits(sign | (new_exp << 23) | mantissa)
}

/// Uses A4 = 440 Hz standard tuning.
#[inline]
/// Technical implementation of the note_to_frequency logic.
pub fn note_to_frequency(note: u8) -> f32 {
    note_to_frequency_tuned(note, 440.0)
}

/// Convert MIDI note to frequency with custom A4 reference.
#[inline]
/// Technical implementation of the note_to_frequency_tuned logic.
pub fn note_to_frequency_tuned(note: u8, a4_hz: f32) -> f32 {
    let exponent = (note as f32 - 69.0) / 12.0;
    a4_hz * pow2f(exponent)
}

/// Technical implementation of the frequency_to_note logic.
pub fn frequency_to_note(freq: f32) -> u8 {
    if freq <= 0.0 {
        return 0;
    }
    let note = 69.0 + 12.0 * log2f(freq / 440.0);
    note.clamp(0.0, 127.0) as u8
}

/// Technical implementation of the frequency_to_cents logic.
pub fn frequency_to_cents(freq: f32) -> f32 {
    if freq <= 0.0 {
        return 0.0;
    }
    let note_float = 69.0 + 12.0 * log2f(freq / 440.0);
    (note_float - trunc_f32(note_float) - 0.5) * 100.0 // -50 to +50 cents
}

/// Technical implementation of the cents_to_multiplier logic.
pub fn cents_to_multiplier(cents: f32) -> f32 {
    pow2f(cents / 1200.0)
}

/// Convert BPM to frequency in Hz.
#[inline]
/// Technical implementation of the bpm_to_frequency logic.
pub fn bpm_to_frequency(bpm: f32) -> f32 {
    bpm / 60.0
}

/// Convert BPM to samples per beat at given sample rate.
#[inline]
/// Technical implementation of the bpm_to_samples_per_beat logic.
pub fn bpm_to_samples_per_beat(bpm: f32, sample_rate: f32) -> f32 {
    sample_rate * 60.0 / bpm
}

/// Convert BPM to samples per bar (4/4 time).
#[inline]
/// Technical implementation of the bpm_to_samples_per_bar logic.
pub fn bpm_to_samples_per_bar(bpm: f32, sample_rate: f32) -> f32 {
    sample_rate * 240.0 / bpm
}

/// Technical implementation of the note_cents_to_frequency logic.
pub fn note_cents_to_frequency(note: u8, cents: f32, a4_hz: f32) -> f32 {
    let base = note_to_frequency_tuned(note, a4_hz);
    base * cents_to_multiplier(cents)
}

/// Convert semitones to frequency multiplier.
#[inline]
/// Technical implementation of the semitones_to_multiplier logic.
pub fn semitones_to_multiplier(semitones: f32) -> f32 {
    pow2f(semitones / 12.0)
}

/// Convert frequency ratio to semitones.
#[inline]
/// Technical implementation of the multiplier_to_semitones logic.
pub fn multiplier_to_semitones(ratio: f32) -> f32 {
    12.0 * log2f(ratio)
}

/// Standard MIDI note names for display
pub mod notes {
    pub const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    /// Technical implementation of the name logic.
    pub fn name(note: u8) -> &'static str {
        NAMES[(note as usize) % 12]
    }

    /// Technical implementation of the octave logic.
    pub fn octave(note: u8) -> i32 {
        (note as i32 / 12) - 1
    }

    /// Technical implementation of the name_octave logic.
    pub fn name_octave(note: u8) -> (&'static str, i32) {
        (name(note), octave(note))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_note_to_frequency logic.
    fn test_note_to_frequency() {
        let a4 = note_to_frequency(69);
        assert!((a4 - 440.0).abs() < 0.1);

        let c4 = note_to_frequency(60);
        assert!((c4 - 261.63).abs() < 0.1);
    }

    #[test]
    /// Technical implementation of the test_frequency_to_note logic.
    fn test_frequency_to_note() {
        assert_eq!(frequency_to_note(440.0), 69);
        assert_eq!(frequency_to_note(261.63), 60);
    }

    #[test]
    /// Technical implementation of the test_semitones_to_multiplier logic.
    fn test_semitones_to_multiplier() {
        let m = semitones_to_multiplier(12.0);
        assert!((m - 2.0).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_notes logic.
    fn test_notes() {
        assert_eq!(notes::name(60), "C");
        assert_eq!(notes::octave(60), 4);
    }
}
