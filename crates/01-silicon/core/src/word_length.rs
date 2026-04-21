/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x93c0de34 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/word_length.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the round_f32 logic.
fn round_f32(x: f32) -> f32 {
    if x >= 0.0 {
        x + 0.5
    } else {
        x - 0.5
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

/// Audio bit depths supported by Smoothie Elite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the BitDepth enumeration.
pub enum BitDepth {
    Bits8,
    Bits16,
    Bits24,
    Bits32,
    Bits64,
}

/// Word length (bit depth) representation
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
/// Technical implementation of the WordLength structure.
pub struct WordLength(BitDepth);

impl WordLength {
    pub const INT8: Self = Self(BitDepth::Bits8);
    pub const INT16: Self = Self(BitDepth::Bits16);
    pub const INT24: Self = Self(BitDepth::Bits24);
    pub const FLOAT32: Self = Self(BitDepth::Bits32);
    pub const FLOAT64: Self = Self(BitDepth::Bits64);

    /// Technical implementation of the bits logic.
    pub fn bits(&self) -> usize {
        match self.0 {
            BitDepth::Bits8 => 8,
            BitDepth::Bits16 => 16,
            BitDepth::Bits24 => 24,
            BitDepth::Bits32 => 32,
            BitDepth::Bits64 => 64,
        }
    }

    /// Technical implementation of the bytes_per_sample logic.
    pub fn bytes_per_sample(&self) -> usize {
        (self.bits() + 7) / 8
    }

    /// Technical implementation of the is_float logic.
    pub fn is_float(&self) -> bool {
        matches!(self.0, BitDepth::Bits32 | BitDepth::Bits64)
    }

    /// Technical implementation of the max_value logic.
    pub fn max_value(&self) -> f64 {
        match self.0 {
            BitDepth::Bits8 => 127.0,
            BitDepth::Bits16 => 32767.0,
            BitDepth::Bits24 => 8388607.0,
            BitDepth::Bits32 => 1.0, // float -1.0 to 1.0
            BitDepth::Bits64 => 1.0, // float64 -1.0 to 1.0
        }
    }
}

impl Default for WordLength {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::FLOAT32
    }
}

/// Convert f32 sample to 16-bit integer
#[inline]
/// Technical implementation of the f32_to_i16 logic.
pub fn f32_to_i16(sample: f32) -> i16 {
    let s = sample.clamp(-1.0, 1.0);
    (s * 32767.0) as i16
}

/// Convert 16-bit integer to f32 sample
#[inline]
/// Technical implementation of the i16_to_f32 logic.
pub fn i16_to_f32(sample: i16) -> f32 {
    (sample as f32) / 32767.0
}

/// Convert f32 sample to 24-bit integer (stored in 32-bit container)
#[inline]
/// Technical implementation of the f32_to_i24 logic.
pub fn f32_to_i24(sample: f32) -> i32 {
    let s = sample.clamp(-1.0, 1.0);
    (s * 8388607.0) as i32
}

/// Convert 24-bit integer to f32 sample
#[inline]
/// Technical implementation of the i24_to_f32 logic.
pub fn i24_to_f32(sample: i32) -> f32 {
    (sample as f32) / 8388607.0
}

/// Convert f32 to 8-bit unsigned
#[inline]
/// Technical implementation of the f32_to_u8 logic.
pub fn f32_to_u8(sample: f32) -> u8 {
    let s = sample.clamp(-1.0, 1.0);
    ((s + 1.0) * 127.5) as u8
}

/// Convert 8-bit unsigned to f32
#[inline]
/// Technical implementation of the u8_to_f32 logic.
pub fn u8_to_f32(sample: u8) -> f32 {
    (sample as f32 / 127.5) - 1.0
}

/// Technical implementation of the convert_bit_depth logic.
pub fn convert_bit_depth(
    src: &[f32],
    dst: &mut [i32],
    _src_depth: WordLength,
    dst_depth: WordLength,
) {
    for (i, &sample) in src.iter().enumerate() {
        if i >= dst.len() {
            break;
        }
        dst[i] = match dst_depth.0 {
            BitDepth::Bits8 => (sample.clamp(-1.0, 1.0) * 127.0) as i32,
            BitDepth::Bits16 => (sample.clamp(-1.0, 1.0) * 32767.0) as i32,
            BitDepth::Bits24 => (sample.clamp(-1.0, 1.0) * 8388607.0) as i32,
            _ => 0,
        };
    }
}

/// Dithering for bit depth reduction
#[repr(align(64))]
/// Technical implementation of the Dither structure.
pub struct Dither {
    state: f32,
}

impl Dither {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self { state: 0.0 }
    }

    /// Technical implementation of the triangular logic.
    pub fn triangular(&mut self) -> f32 {
        // Simple triangular dither
        let r1 = fast_rand();
        let r2 = fast_rand();
        let dither = r1 - r2 - self.state;
        self.state = r1;
        dither * 0.5
    }

    /// Technical implementation of the rect logic.
    pub fn rect(&mut self) -> f32 {
        fast_rand() - 0.5
    }

    /// Technical implementation of the apply logic.
    pub fn apply(&mut self, sample: f32, bits: usize) -> f32 {
        let scale = 1 << (bits - 1);
        let dither = self.triangular();
        round_f32((sample + dither) * scale as f32) / scale as f32
    }
}

impl Default for Dither {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Fast pseudo-random for dithering
#[inline]
/// Technical implementation of the fast_rand logic.
fn fast_rand() -> f32 {
    use core::sync::atomic::AtomicU32;
    static SEED: AtomicU32 = AtomicU32::new(0x12345678);

    let x = SEED.load(core::sync::atomic::Ordering::Relaxed);
    let y = x.wrapping_mul(16807);
    SEED.store(y, core::sync::atomic::Ordering::Relaxed);
    (y as f32) / 2147483647.0
}

/// Quantization error feedback (noise shaping)
#[repr(align(64))]
/// Technical implementation of the NoiseShaper structure.
pub struct NoiseShaper {
    error: f32,
}

impl NoiseShaper {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self { error: 0.0 }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, sample: f32, bits: usize) -> f32 {
        let scale = 1 << (bits - 1);
        let quantized = round_f32((sample + self.error) * scale as f32) / scale as f32;
        self.error = sample - quantized;
        quantized
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.error = 0.0;
    }
}

impl Default for NoiseShaper {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_f32_to_i16 logic.
    fn test_f32_to_i16() {
        assert_eq!(f32_to_i16(1.0), 32767);
        assert_eq!(f32_to_i16(-1.0), -32767);
        assert_eq!(f32_to_i16(0.0), 0);
    }

    #[test]
    /// Technical implementation of the test_i16_to_f32 logic.
    fn test_i16_to_f32() {
        assert!((i16_to_f32(32767) - 1.0).abs() < 0.0001);
        assert!((i16_to_f32(0)).abs() < 0.0001);
    }

    #[test]
    /// Technical implementation of the test_word_length logic.
    fn test_word_length() {
        assert_eq!(WordLength::INT16.bits(), 16);
        assert_eq!(WordLength::FLOAT32.bytes_per_sample(), 4);
    }
}
