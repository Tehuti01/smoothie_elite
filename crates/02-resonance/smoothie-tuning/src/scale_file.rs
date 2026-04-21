/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x66ae1072 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-tuning/src/scale_file.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Parser for Scala .scl format and scale file management.

use alloc::string::String;
use alloc::vec::Vec;

#[repr(align(64))]
/// Technical implementation of the ScaleFile structure.
pub struct ScaleFile {
    pub name: [u8; 64],
    pub name_len: usize,
    pub degrees: [f32; 128],
    pub degree_count: usize,
}

impl ScaleFile {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            name: [0u8; 64],
            name_len: 0,
            degrees: [0.0; 128],
            degree_count: 0,
        }
    }

    /// Technical implementation of the parse logic.
    pub fn parse(source: &[u8]) -> Option<Self> {
        let mut scale = Self::new();

        let mut lines = source.split(|&b| b == b'\n' || b == b'\r');

        let mut first_line = true;
        let mut degree_count = 0;

        for line in lines {
            let trimmed = line
                .iter()
                .copied()
                .skip_while(|&&b| b == b' ' || b == b'\t')
                .take_while(|&&b| b != b' ' && b != b'\t')
                .collect::<Vec<_>>();

            if trimmed.is_empty() || trimmed[0] == b'!' {
                continue;
            }

            if first_line {
                scale.name_len = trimmed.len().min(64);
                for i in 0..scale.name_len {
                    scale.name[i] = trimmed[i];
                }
                first_line = false;
                continue;
            }

            if let Ok(count) = trimmed
                .iter()
                .map(|&b| b as char)
                .collect::<String>()
                .parse::<usize>()
            {
                degree_count = count;
                continue;
            }

            if degree_count > 0 && scale.degree_count < 128 {
                if let Some(cents) = parse_pitch_value(&trimmed) {
                    scale.degrees[scale.degree_count] = cents;
                    scale.degree_count += 1;

                    if scale.degree_count >= degree_count {
                        break;
                    }
                }
            }
        }

        if scale.degree_count > 0 {
            Some(scale)
        } else {
            None
        }
    }

    /// Technical implementation of the get_name logic.
    pub fn get_name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len]).unwrap_or("")
    }
}

impl Default for ScaleFile {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the parse_pitch_value logic.
fn parse_pitch_value(data: &[u8]) -> Option<f32> {
    if data.is_empty() {
        return None;
    }

    let has_slash = data.iter().position(|&b| b == b'/');

    if let Some(slash_pos) = has_slash {
        let numerator = parse_decimal(&data[..slash_pos])?;
        let denominator = parse_decimal(&data[slash_pos + 1..])?;

        if denominator == 0.0 {
            return None;
        }

        let ratio = numerator / denominator;
        if ratio <= 0.0 {
            return None;
        }

        Some(1200.0 * fast_log2(ratio))
    } else {
        parse_decimal(data)
    }
}

/// Technical implementation of the parse_decimal logic.
fn parse_decimal(data: &[u8]) -> Option<f32> {
    let mut result = 0.0f32;
    let mut decimal_started = false;
    let mut divisor = 1.0f32;
    let mut found_digit = false;

    for &byte in data {
        if byte >= b'0' && byte <= b'9' {
            if !decimal_started {
                result = result * 10.0 + (byte - b'0') as f32;
            } else {
                result += (byte - b'0') as f32 / divisor;
                divisor *= 10.0;
            }
            found_digit = true;
        } else if byte == b'.' && !decimal_started && found_digit {
            decimal_started = true;
        } else {
            break;
        }
    }

    if found_digit {
        Some(result)
    } else {
        None
    }
}

/// Technical implementation of the fast_log2 logic.
fn fast_log2(x: f32) -> f32 {
    let n = x.to_bits();
    let exp = ((n >> 23) & 0xFF) as i32 - 127;
    let mantissa = f32::from_bits((n & 0x7FFFFF) | 0x3F800000) - 1.0;
    exp as f32 + mantissa * (1.0 - mantissa * 0.5) * core::f32::consts::LOG2_E
}
