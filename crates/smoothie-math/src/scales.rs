//! Perceptual frequency scales (Mel, Bark, ERB, A-weighting).

use std::f32::consts::PI;

// ─── Mel scale ───────────────────────────────────────────────────────────────

/// Hz → Mel (O'Shaughnessy formula).
#[inline]
pub fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

/// Mel → Hz.
#[inline]
pub fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10.0_f32.powf(mel / 2595.0) - 1.0)
}

// ─── Bark scale ──────────────────────────────────────────────────────────────

/// Hz → Bark (Traunmüller 1990 formula).
#[inline]
pub fn hz_to_bark(hz: f32) -> f32 {
    let z = ((26.81 * hz) / (1960.0 + hz)) - 0.53;
    if z < 2.0 { z + 0.15 * (2.0 - z) }
    else if z > 20.1 { z + 0.22 * (z - 20.1) }
    else { z }
}

/// Bark → Hz (inverse approximation).
#[inline]
pub fn bark_to_hz(bark: f32) -> f32 {
    1960.0 / (26.81 / (bark + 0.53) - 1.0)
}

// ─── ERB scale ───────────────────────────────────────────────────────────────

/// Hz → ERB-rate (Moore & Glasberg 1983).
#[inline]
pub fn hz_to_erb(hz: f32) -> f32 {
    21.4 * (1.0 + 0.00437 * hz).log10()
}

/// ERB-rate → Hz.
#[inline]
pub fn erb_to_hz(erb: f32) -> f32 {
    (10.0_f32.powf(erb / 21.4) - 1.0) / 0.00437
}

// ─── A-weighting ─────────────────────────────────────────────────────────────

/// A-weighting correction in dB for a given frequency.
/// Per IEC 61672, based on the analytical formula.
pub fn a_weight_db(hz: f32) -> f32 {
    let f2 = hz * hz;
    let f4 = f2 * f2;
    let ra = (12194.0_f32.powi(2) * f4)
        / ((f2 + 20.6_f32.powi(2))
            * ((f2 + 107.7_f32.powi(2)) * (f2 + 737.9_f32.powi(2))).sqrt()
            * (f2 + 12194.0_f32.powi(2)));
    20.0 * ra.log10() + 2.0
}

// ─── Equal-loudness curves (Fletcher-Munson approximation) ───────────────────

/// Approximate equal-loudness (phon) level at `hz` for a tone at `spl_db`.
/// Very rough approximation — returns perceived loudness in phon.
pub fn spl_to_phon(hz: f32, spl_db: f32) -> f32 {
    let aw = a_weight_db(hz);
    (spl_db + aw).max(0.0)
}

// ─── MIDI note name ───────────────────────────────────────────────────────────

const NOTE_NAMES: &[&str] = &["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];

/// Return the note name and octave for a MIDI note number.
pub fn midi_note_name(midi: u8) -> (&'static str, i32) {
    let name = NOTE_NAMES[(midi % 12) as usize];
    let octave = (midi as i32 / 12) - 1;
    (name, octave)
}

/// Formatted note name, e.g. "A4", "C#3".
pub fn midi_note_string(midi: u8) -> std::string::String {
    let (name, oct) = midi_note_name(midi);
    format!("{}{}", name, oct)
}

// ─── Pythagorean Tuning ──────────────────────────────────────────────────────

/// Pure Pythagorean frequency ratios based on perfect fifths (3:2).
pub const PYTHAGOREAN_RATIOS: [f32; 12] = [
    1.0,           // Unison
    2187.0/2048.0, // minor second
    9.0/8.0,       // major second
    32.0/27.0,     // minor third
    81.0/64.0,     // major third
    4.0/3.0,       // perfect fourth
    729.0/512.0,   // augmented fourth
    3.0/2.0,       // perfect fifth
    128.0/81.0,    // minor sixth
    27.0/16.0,     // major sixth
    16.0/9.0,      // minor seventh
    243.0/128.0    // major seventh
];

/// Calculate frequency using pure Pythagorean tuning based on a reference A4=440Hz.
pub fn pythagorean_tuning(midi: u8) -> f32 {
    let a4_midi = 69;
    let diff = midi as i32 - a4_midi;
    let mut octaves = diff / 12;
    let mut note = diff % 12;
    if note < 0 {
        note += 12;
        octaves -= 1;
    }
    440.0 * PYTHAGOREAN_RATIOS[note as usize] * 2.0_f32.powi(octaves)
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
