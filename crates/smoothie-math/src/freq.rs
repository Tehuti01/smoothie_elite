//! Frequency / pitch / MIDI conversions.

pub const A4_HZ: f32   = 440.0;
pub const A4_MIDI: f32 = 69.0;

/// MIDI note number → frequency (Hz). Equal temperament, A4=440 Hz.
#[inline]
pub fn midi_to_hz(note: f32) -> f32 {
    A4_HZ * 2.0_f32.powf((note - A4_MIDI) / 12.0)
}

/// Frequency (Hz) → MIDI note number (fractional).
#[inline]
pub fn hz_to_midi(hz: f32) -> f32 {
    A4_MIDI + 12.0 * (hz / A4_HZ).log2()
}

/// Semitone offset → frequency ratio.
#[inline]
pub fn semitones_to_ratio(semitones: f32) -> f32 {
    2.0_f32.powf(semitones / 12.0)
}

/// Frequency ratio → semitones.
#[inline]
pub fn ratio_to_semitones(ratio: f32) -> f32 {
    12.0 * ratio.log2()
}

/// Cents offset → frequency ratio.
#[inline]
pub fn cents_to_ratio(cents: f32) -> f32 {
    2.0_f32.powf(cents / 1200.0)
}

/// Frequency → octave-normalized position in [0, 1] within a given range.
#[inline]
pub fn freq_to_octave_pos(hz: f32, lo: f32, hi: f32) -> f32 {
    ((hz / lo).log2() / (hi / lo).log2()).clamp(0.0, 1.0)
}

/// Period in samples for a given frequency and sample rate.
#[inline]
pub fn freq_to_samples(hz: f32, sample_rate: f32) -> f32 {
    sample_rate / hz
}

/// Nyquist frequency for a given sample rate.
#[inline]
pub fn nyquist(sample_rate: f32) -> f32 { sample_rate * 0.5 }

/// Normalized angular frequency (0..π) from Hz and sample rate.
#[inline]
pub fn hz_to_omega(hz: f32, sample_rate: f32) -> f32 {
    2.0 * std::f32::consts::PI * hz / sample_rate
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
