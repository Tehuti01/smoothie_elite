/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1e45d238 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/sample.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[inline]
/// Technical implementation of the powf logic.
fn powf(base: f32, exp: f32) -> f32 {
    pow2f(exp * log2f(base))
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
/// Technical implementation of the pow2f logic.
fn pow2f(x: f32) -> f32 {
    let xi = if x >= 0.0 { x as i32 } else { x as i32 - 1 };
    let xf = x - xi as f32;
    let frac = 1.0 + xf * (core::f32::consts::LN_2 + xf * (0.2402265 + xf * 0.0558015));
    let exp_bits = ((xi + 127) as u32) << 23;
    f32::from_bits(exp_bits) * frac
}

#[inline]
/// Technical implementation of the log10f logic.
fn log10f(x: f32) -> f32 {
    log2f(x) / core::f32::consts::LOG2_E
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

/// Sample value (single audio sample, -1.0 to 1.0)
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the Sample structure.
pub struct Sample(f32);

impl Sample {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);
    pub const NEG_ONE: Self = Self(-1.0);

    /// Initializes a new instance of the associated type.
    pub fn new(val: f32) -> Self {
        Self(val)
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the saturating_add logic.
    pub fn saturating_add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }

    /// Technical implementation of the saturating_sub logic.
    pub fn saturating_sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }

    /// Technical implementation of the abs logic.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Technical implementation of the is_silent logic.
    pub fn is_silent(&self) -> bool {
        self.0.abs() < 1e-10
    }

    /// Technical implementation of the to_db logic.
    pub fn to_db(self) -> Decibel {
        if self.0.abs() < 1e-10 {
            Decibel::MINUS_INFINITY
        } else {
            Decibel(20.0 * log10f(self.0.abs()))
        }
    }

    /// Technical implementation of the from_db logic.
    pub fn from_db(db: Decibel) -> Self {
        if db.0 <= Decibel::MINUS_INFINITY.0 {
            Self::ZERO
        } else {
            Self(powf(10.0_f32, db.0 / 20.0))
        }
    }
}

impl Add for Sample {
    type Output = Self;
    /// Performs vector addition logic.
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Sample {
    type Output = Self;
    /// Performs vector subtraction logic.
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul for Sample {
    type Output = Self;
    /// Performs matrix/vector multiplication.
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Div for Sample {
    type Output = Self;
    /// Technical implementation of the div logic.
    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl Neg for Sample {
    type Output = Self;
    /// Technical implementation of the neg logic.
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl PartialEq for Sample {
    /// Technical implementation of the eq logic.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Sample {
    /// Technical implementation of the partial_cmp logic.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

/// Frequency in Hz
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the Frequency structure.
pub struct Frequency(f32);

impl Frequency {
    pub const ZERO: Self = Self(0.0);
    pub const MIN: Self = Self(0.0);
    pub const MAX: Self = Self(192000.0);
    pub const NYQUIST: Self = Self(22050.0); // At 44100 Hz sample rate

    /// Initializes a new instance of the associated type.
    pub fn new(hz: f32) -> Self {
        Self(hz.max(0.0))
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the from_note logic.
    pub fn from_note(note: u8, a4: f32) -> Self {
        let exp = (note as f32 - 69.0) / 12.0;
        Self(a4 * powf(2.0_f32, exp))
    }

    /// Technical implementation of the to_note logic.
    pub fn to_note(self, a4: f32) -> u8 {
        if self.0 <= 0.0 {
            return 0;
        }
        let note = 69.0 + 12.0 * log2f(self.0 / a4);
        note.clamp(0.0, 127.0) as u8
    }

    /// Technical implementation of the to_radians logic.
    pub fn to_radians(self) -> f32 {
        2.0 * core::f32::consts::PI * self.0
    }

    /// Technical implementation of the from_radians logic.
    pub fn from_radians(rad: f32) -> Self {
        Self(rad / (2.0 * core::f32::consts::PI))
    }
}

impl Add for Frequency {
    type Output = Self;
    /// Performs vector addition logic.
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Frequency {
    type Output = Self;
    /// Performs vector subtraction logic.
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul<f32> for Frequency {
    type Output = Self;
    /// Performs matrix/vector multiplication.
    fn mul(self, other: f32) -> Self {
        Self(self.0 * other)
    }
}

impl Div<f32> for Frequency {
    type Output = Self;
    /// Technical implementation of the div logic.
    fn div(self, other: f32) -> Self {
        Self(self.0 / other)
    }
}

/// Decibel value
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the Decibel structure.
pub struct Decibel(f32);

impl Decibel {
    pub const MINUS_INFINITY: Self = Self(-96.0);
    pub const ZERO: Self = Self(0.0);
    pub const MIN: Self = Self(-96.0);
    pub const MAX: Self = Self(6.0); // +6 dB (headroom)

    /// Initializes a new instance of the associated type.
    pub fn new(db: f32) -> Self {
        Self(db.clamp(-96.0, 6.0))
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the to_linear logic.
    pub fn to_linear(self) -> f32 {
        if self.0 <= -96.0 {
            0.0
        } else {
            powf(10.0_f32, self.0 / 20.0)
        }
    }

    /// Technical implementation of the from_linear logic.
    pub fn from_linear(linear: f32) -> Self {
        if linear <= 0.0 {
            Self::MINUS_INFINITY
        } else {
            Self(20.0 * log10f(linear))
        }
    }
}

impl Add for Decibel {
    type Output = Self;
    /// Performs vector addition logic.
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for Decibel {
    type Output = Self;
    /// Performs vector subtraction logic.
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

/// Phase value (0.0 to 1.0, or 0 to 2π radians)
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the Phase structure.
pub struct Phase(f32);

impl Phase {
    pub const ZERO: Self = Self(0.0);
    pub const HALF: Self = Self(0.5);
    pub const ONE: Self = Self(1.0);

    /// Initializes a new instance of the associated type.
    pub fn new(phase: f32) -> Self {
        Self(phase - trunc_f32(phase))
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the radians logic.
    pub fn radians(self) -> f32 {
        self.0 * 2.0 * core::f32::consts::PI
    }
    /// Technical implementation of the from_radians logic.
    pub fn from_radians(rad: f32) -> Self {
        let normalized = rad / (2.0 * core::f32::consts::PI);
        Self(normalized - trunc_f32(normalized))
    }

    /// Performs vector addition logic.
    pub fn add(&self, delta: f32) -> Self {
        Self((self.0 + delta) - trunc_f32(self.0 + delta))
    }
}

/// BPM value
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the Bpm structure.
pub struct Bpm(f32);

impl Bpm {
    pub const MIN: Self = Self(20.0);
    pub const MAX: Self = Self(300.0);
    pub const DEFAULT: Self = Self(120.0);

    /// Initializes a new instance of the associated type.
    pub fn new(bpm: f32) -> Self {
        Self(bpm.clamp(20.0, 300.0))
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the to_ms logic.
    pub fn to_ms(self) -> f32 {
        60000.0 / self.0
    }
    /// Technical implementation of the from_ms logic.
    pub fn from_ms(ms: f32) -> Self {
        Self(60000.0 / ms)
    }
}

impl Add for Bpm {
    type Output = Self;
    /// Performs vector addition logic.
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0).clamp(20.0, 300.0))
    }
}

/// Sample rate
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the SampleRate structure.
pub struct SampleRate(f32);

impl SampleRate {
    pub const MIN: Self = Self(8000.0);
    pub const MAX: Self = Self(192000.0);
    pub const STANDARD_44100: Self = Self(44100.0);
    pub const STANDARD_48000: Self = Self(48000.0);

    /// Initializes a new instance of the associated type.
    pub fn new(sr: f32) -> Self {
        Self(sr.clamp(8000.0, 192000.0))
    }
    /// Technical implementation of the get logic.
    pub fn get(self) -> f32 {
        self.0
    }

    /// Technical implementation of the nyquist logic.
    pub fn nyquist(self) -> Frequency {
        Frequency(self.0 / 2.0)
    }

    /// Technical implementation of the to_period logic.
    pub fn to_period(self) -> f32 {
        1.0 / self.0
    }
}

impl PartialEq for SampleRate {
    /// Technical implementation of the eq logic.
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.01
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_sample_from_db logic.
    fn test_sample_from_db() {
        let s = Sample::from_db(Decibel::ZERO);
        assert!((s.get() - 1.0).abs() < 0.01);

        let s = Sample::from_db(Decibel::MINUS_INFINITY);
        assert!(s.get() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_frequency_from_note logic.
    fn test_frequency_from_note() {
        let a4 = Frequency::from_note(69, 440.0);
        assert!((a4.get() - 440.0).abs() < 0.1);
    }

    #[test]
    /// Technical implementation of the test_decibel_to_linear logic.
    fn test_decibel_to_linear() {
        let db = Decibel::ZERO;
        assert!((db.to_linear() - 1.0).abs() < 0.001);

        let db = Decibel(-6.0);
        assert!((db.to_linear() - 0.5).abs() < 0.001);
    }
}
