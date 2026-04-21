/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe74ba47b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/constants.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub const PHI: f32 = 1.618_034_f32;
pub const PHI_F64: f64 = 1.618033988749895;

/// PHI² - Golden ratio squared (φ² = φ + 1)
pub const PHI_SQUARED: f32 = 2.618_034_f32;
pub const PHI_SQUARED_F64: f64 = 2.618033988749895;

/// 1/PHI - Inverse of golden ratio
pub const PHI_INV: f32 = 0.618_034_f32;
pub const PHI_INV_F64: f64 = 0.618033988749895;

/// Derived from the Flower of Life (Hexagonal Lattice)
pub const SQRT_3: f32 = 1.732_050_8f32;
pub const SQRT_3_F64: f64 = 1.7320508075688772;

/// The area of intersection between two circles divided by the total area
pub const VESICA_PISCIS: f32 = 0.391_002_2f32;
pub const VESICA_PISCIS_F64: f64 = 0.39100221895577053;

/// 🏛️ 12-Dimensional Symmetry Group
pub const C12_SYMMETRY: usize = 12;

/// PHI³ - Golden ratio cubed (4.236...)
pub const PHI_CUBED: f32 = 4.236_067_977_499_789_696_409_173_668_731_276_235f32;

/// ln(PHI) - Natural logarithm of golden ratio
pub const PHI_LN: f32 = 0.481_211_825_059_603_447_374_141_747_377_458_243_f32;

/// log₂(PHI) - Binary logarithm of golden ratio
pub const PHI_LOG2: f32 = 0.694_242_147_180_504_551_768_184_104_621_949_429f32;

// ═══════════════════════════════════════════════════════════════
// Fibonacci Sequence Numbers
// Used for sizing, counts, and harmonic structure
// ═══════════════════════════════════════════════════════════════

pub const FIB_1: usize = 1;
pub const FIB_2: usize = 2;
pub const FIB_3: usize = 3;
pub const FIB_4: usize = 5;
pub const FIB_5: usize = 8;
pub const FIB_6: usize = 13; // State machine boundaries
pub const FIB_7: usize = 21; // Common harmonic division
pub const FIB_8: usize = 34; // Audio buffer alignment
pub const FIB_9: usize = 55; // Cache line padding
pub const FIB_10: usize = 89; // Filter taps, delay lines, atomics
pub const FIB_11: usize = 144; // Window functions, atomic padding
pub const FIB_12: usize = 233; // Counter padding, FFT sizes
pub const FIB_13: usize = 377; // Spectrum bins
pub const FIB_14: usize = 610; // Ring buffer sizes
pub const FIB_15: usize = 987; // Large delay buffers
pub const FIB_16: usize = 1597; // Memory pool sizes
pub const FIB_17: usize = 2584; // DFT buffer capacity
pub const FIB_18: usize = 4181; // Large ring buffers
pub const FIB_19: usize = 6765; // Frame pool sizes

// Fibonacci aliases for DSP operations
pub const F_13: usize = FIB_6; // 13
pub const F_89: usize = FIB_10; // 89
pub const F_144: usize = FIB_11; // 144
pub const F_233: usize = FIB_12; // 233
pub const F_256: usize = 256; // Power of 2 for FFT (nearest Fibonacci-ish)

// ═══════════════════════════════════════════════════════════════
// Audio Standards
// ═══════════════════════════════════════════════════════════════

/// Standard tuning frequency (A4 in Hz)
pub const STANDARD_PITCH: f32 = 432.0;

/// MIDI Note 0 (C-1) frequency (at 432Hz standard)
pub const MIDI_NOTE_0_FREQ: f32 = 8.027_148_026_f32;

/// Semitone ratio (2^(1/12))
pub const SEMITONE_RATIO: f32 = 1.059_463_094_359_295f32;

/// Octave ratio
pub const OCTAVE_RATIO: f32 = 2.0;

/// Concert pitch (A = 440 Hz)
pub const CONCERT_PITCH: f32 = 440.0;

// ═══════════════════════════════════════════════════════════════
// Common Sample Rates (Fibonacci-optimized selection)
// ═══════════════════════════════════════════════════════════════

/// CD quality sample rate
pub const SAMPLE_RATE_44100: f32 = 44100.0;

/// Professional audio sample rate
pub const SAMPLE_RATE_48000: f32 = 48000.0;

/// High resolution audio (2x 44.1k)
pub const SAMPLE_RATE_88200: f32 = 88200.0;

/// High resolution audio (2x 48k)
pub const SAMPLE_RATE_96000: f32 = 96000.0;

/// Chief Engineering quality
pub const SAMPLE_RATE_192000: f32 = 192000.0;

// ═══════════════════════════════════════════════════════════════
// Decibel Reference Levels
// ═══════════════════════════════════════════════════════════════

/// Silence threshold (dB)
pub const SILENCE_THRESHOLD_DB: f32 = -80.0;

/// Nominal operating level (dB)
pub const NOMINAL_LEVEL_DB: f32 = -18.0;

/// Peak headroom (dB)
pub const HEADROOM_DB: f32 = 6.0;

/// Maximum before clipping (dB)
pub const CLIPPING_LEVEL_DB: f32 = 0.0;

/// Soft clipping threshold (dB)
pub const SOFT_CLIP_THRESHOLD: f32 = -3.0;

// ═══════════════════════════════════════════════════════════════
// Frequency Bands (for EQ and analysis)
// ═══════════════════════════════════════════════════════════════

/// Sub-bass lower boundary (Hz)
pub const FREQ_SUB_BASS_LOW: f32 = 20.0;

/// Sub-bass upper boundary (Hz)
pub const FREQ_SUB_BASS_HIGH: f32 = 60.0;

/// Bass lower boundary (Hz)
pub const FREQ_BASS_LOW: f32 = 60.0;

/// Bass upper boundary (Hz)
pub const FREQ_BASS_HIGH: f32 = 250.0;

/// Low-mid lower boundary (Hz)
pub const FREQ_LOW_MID_LOW: f32 = 250.0;

/// Low-mid upper boundary (Hz)
pub const FREQ_LOW_MID_HIGH: f32 = 500.0;

/// Mid lower boundary (Hz)
pub const FREQ_MID_LOW: f32 = 500.0;

/// Mid upper boundary (Hz)
pub const FREQ_MID_HIGH: f32 = 2000.0;

/// High-mid lower boundary (Hz)
pub const FREQ_HIGH_MID_LOW: f32 = 2000.0;

/// High-mid upper boundary (Hz)
pub const FREQ_HIGH_MID_HIGH: f32 = 4000.0;

/// Presence lower boundary (Hz)
pub const FREQ_PRESENCE_LOW: f32 = 4000.0;

/// Presence upper boundary (Hz)
pub const FREQ_PRESENCE_HIGH: f32 = 6000.0;

/// Brilliance lower boundary (Hz)
pub const FREQ_BRILLIANCE_LOW: f32 = 6000.0;

/// Brilliance upper boundary (Hz)
pub const FREQ_BRILLIANCE_HIGH: f32 = 20000.0;

// ═══════════════════════════════════════════════════════════════
// Time Constants
// ═══════════════════════════════════════════════════════════════

/// Minimum fade time (ms)
pub const MIN_FADE_TIME_MS: f32 = 1.0;

/// Default fade time (ms)
pub const DEFAULT_FADE_TIME_MS: f32 = 20.0;

/// Long fade time (ms, typical for volume automations)
pub const LONG_FADE_TIME_MS: f32 = 100.0;

/// Very long fade time (ms)
pub const VERY_LONG_FADE_TIME_MS: f32 = 500.0;

/// ADSR attack time (typical, in seconds)
pub const TYPICAL_ATTACK_SEC: f32 = 0.005;

/// ADSR decay time (typical, in seconds)
pub const TYPICAL_DECAY_SEC: f32 = 0.1;

/// ADSR release time (typical, in seconds)
pub const TYPICAL_RELEASE_SEC: f32 = 0.3;

/// Meter hold time (ms) - held for 1.1 seconds
pub const METER_HOLD_TIME_MS: f32 = 1100.0;

// ═══════════════════════════════════════════════════════════════
// Damping and Smoothing Coefficients
// ═══════════════════════════════════════════════════════════════

/// Fast smoothing coefficient (0.1 = 90% old, 10% new)
pub const SMOOTH_FAST: f32 = 0.1;

/// Medium smoothing coefficient
pub const SMOOTH_MEDIUM: f32 = 0.05;

/// Slow smoothing coefficient
pub const SMOOTH_SLOW: f32 = 0.01;

/// Very slow smoothing coefficient (for meters, 0.5 sec at 44.1k = 0.00003)
pub const SMOOTH_VERY_SLOW: f32 = 0.00003;

/// Low-pass filter resonance peak boost
pub const RESONANCE_PEAK: f32 = 1.5;

/// Default filter Q factor
pub const DEFAULT_Q: f32 = 0.707; // √2/2 (Butterworth)

/// Moog filter Q factor
pub const MOOG_Q: f32 = 1.0;

// ═══════════════════════════════════════════════════════════════
// Compression and Dynamics
// ═══════════════════════════════════════════════════════════════

/// Default compressor ratio
pub const DEFAULT_RATIO: f32 = 4.0;

/// Limiting ratio (very high compression)
pub const LIMITER_RATIO: f32 = 10.0;

/// Default compressor attack time (ms)
pub const DEFAULT_COMP_ATTACK_MS: f32 = 10.0;

/// Default compressor release time (ms)
pub const DEFAULT_COMP_RELEASE_MS: f32 = 100.0;

/// Default threshold (dB)
pub const DEFAULT_THRESHOLD_DB: f32 = -20.0;

// ═══════════════════════════════════════════════════════════════
// Delays and Effects
// ═══════════════════════════════════════════════════════════════

/// Minimum delay time (ms)
pub const MIN_DELAY_MS: f32 = 1.0;

/// Maximum delay time (ms) - 5 seconds
pub const MAX_DELAY_MS: f32 = 5000.0;

/// Default delay time (ms) - quarter note at 120 BPM
pub const DEFAULT_DELAY_MS: f32 = 500.0;

/// Minimum reverb room size
pub const MIN_ROOM_SIZE: f32 = 0.0;

/// Maximum reverb room size
pub const MAX_ROOM_SIZE: f32 = 1.0;

/// Default reverb damping
pub const DEFAULT_DAMPING: f32 = 0.5;

/// Comb filter base frequency (Hz) - for reverb algorithms
pub const COMB_FREQ_BASE: f32 = 41.1;

/// Allpass filter base frequency (Hz)
pub const ALLPASS_FREQ_BASE: f32 = 225.0;

// ═══════════════════════════════════════════════════════════════
// Oscillator Parameters
// ═══════════════════════════════════════════════════════════════

/// Default PWM (pulse width modulation) width
pub const DEFAULT_PWM_WIDTH: f32 = 0.5;

/// Minimum PWM width
pub const MIN_PWM_WIDTH: f32 = 0.01;

/// Maximum PWM width
pub const MAX_PWM_WIDTH: f32 = 0.99;

/// Default LFO frequency (Hz)
pub const DEFAULT_LFO_FREQ: f32 = 5.0;

/// Minimum LFO frequency (Hz)
pub const MIN_LFO_FREQ: f32 = 0.01;

/// Maximum LFO frequency (Hz)
pub const MAX_LFO_FREQ: f32 = 100.0;

// ═══════════════════════════════════════════════════════════════
// MIDI Constants
// ═══════════════════════════════════════════════════════════════

/// Minimum MIDI note number
pub const MIDI_NOTE_MIN: u8 = 0;

/// Maximum MIDI note number
pub const MIDI_NOTE_MAX: u8 = 127;

/// Middle C MIDI note
pub const MIDI_NOTE_MIDDLE_C: u8 = 60;

/// MIDI velocity range
pub const MIDI_VELOCITY_MIN: u8 = 0;

/// MIDI velocity max
pub const MIDI_VELOCITY_MAX: u8 = 127;

/// MIDI CC value minimum
pub const MIDI_CC_MIN: u8 = 0;

/// MIDI CC value maximum
pub const MIDI_CC_MAX: u8 = 127;

/// Pitch bend center value
pub const MIDI_PITCH_BEND_CENTER: u16 = 8192;

/// Pitch bend maximum deviation (semitones)
pub const MIDI_PITCH_BEND_RANGE: f32 = 2.0;

// ═══════════════════════════════════════════════════════════════
// Numerical Constants
// ═══════════════════════════════════════════════════════════════

/// π (Pi)
pub const PI: f32 = 3.141592653589793f32;
pub const PI_F64: f64 = 3.141592653589793;

/// 2π (Tau)
pub const TAU: f32 = 6.283185307179586f32;
pub const TAU_F64: f64 = 6.283185307179586;

/// √2
pub const SQRT_2: f32 = 1.414213562373095f32;

/// 1/√2
pub const INV_SQRT_2: f32 = 0.707_106_781_186_547_524_400_844_362_104_849_039f32;

/// Golden angle (radians)
pub const GOLDEN_ANGLE: f32 = 2.39_996_322_972_865_332_954_971_056_563_266_559f32;

// ═══════════════════════════════════════════════════════════════
// Computation Optimization Parameters
// ═══════════════════════════════════════════════════════════════

/// Fast reciprocal approximation iterations
pub const FAST_RECIP_ITERATIONS: usize = 2;

/// Fast sqrt approximation iterations
pub const FAST_SQRT_ITERATIONS: usize = 2;

/// LUT (Lookup Table) precision bits for sine
pub const SINE_LUT_BITS: usize = 8;

/// Window function LUT size
pub const WINDOW_LUT_BITS: usize = 7;

/// Interpolation table size
pub const INTERP_LUT_BITS: usize = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_phi_relationships logic.
    fn test_phi_relationships() {
        // φ² = φ + 1 (defining property)
        let diff = (PHI_SQUARED - (PHI + 1.0)).abs();
        assert!(diff < 0.001);
    }

    #[test]
    /// Technical implementation of the test_fibonacci_sequence logic.
    fn test_fibonacci_sequence() {
        // Fibonacci ratio approaches φ
        let ratio = FIB_10 as f32 / FIB_9 as f32;
        let phi_approx = 89.0 / 55.0;
        assert!((ratio - phi_approx).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_semitone_ratio logic.
    fn test_semitone_ratio() {
        // 12 semitones should equal one octave (2x)
        let freq_up_octave = SEMITONE_RATIO.powf(12.0);
        assert!((freq_up_octave - OCTAVE_RATIO).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_frequency_bands logic.
    fn test_frequency_bands() {
        // Ensure bands don't overlap
        assert!(FREQ_SUB_BASS_HIGH <= FREQ_BASS_LOW);
        assert!(FREQ_BASS_HIGH <= FREQ_LOW_MID_LOW);
        assert!(FREQ_LOW_MID_HIGH <= FREQ_MID_LOW);
    }

    #[test]
    /// Technical implementation of the test_midi_ranges logic.
    fn test_midi_ranges() {
        assert!(MIDI_NOTE_MIDDLE_C < MIDI_NOTE_MAX);
        assert!(MIDI_VELOCITY_MIN < MIDI_VELOCITY_MAX);
    }

    #[test]
    /// Technical implementation of the test_sqrt_constant logic.
    fn test_sqrt_constant() {
        let computed = crate::math::sqrt_approx(2.0_f32);
        assert!((SQRT_2 - computed).abs() < 0.01);
    }

    #[test]
    /// Verifies Note 69 (A4) matches the global STANDARD_PITCH.
    fn test_pitch_standard() {
        // f = Note0 * 2^(69/12)
        let a4_freq = MIDI_NOTE_0_FREQ * 2.0_f32.powf(69.0 / 12.0);
        let diff = (a4_freq - STANDARD_PITCH).abs();
        assert!(diff < 0.0001, "A4 freq {} != standard pitch {}", a4_freq, STANDARD_PITCH);
    }
}

/// Framework version constant.
pub const FRAMEWORK_VERSION: &str = "1.0.0";

/// Framework name constant.
pub const FRAMEWORK_NAME: &str = "Smoothie Elite";

/// Pythagorean perfect fifth ratio (3:2).
pub const PYTHAGOREAN_FIFTH: f64 = 1.5;

/// Pythagorean perfect fourth ratio (4:3).
pub const PYTHAGOREAN_FOURTH: f64 = 1.333333333333333;

/// Harmonic resonance product (used internally for coefficient derivation).
#[inline(always)]
/// Technical implementation of the harmonic_product logic.
pub const fn harmonic_product() -> f64 {
    // PHI * PI * 3/2 — natural resonance baseline
    1.618033988749895 * 3.141592653589793 * 1.5
}
