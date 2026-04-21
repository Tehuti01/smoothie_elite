/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x904377c8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-tuning/src/scala.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::table::TuningTable;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
///
/// arbitrary musical scale definitions. Scala files are human-readable text
///
///
/// ! meantone.scl
/// !
/// !
/// 25/24
/// 5/4
/// 25/16
/// ```
/// Lines beginning with `!` are comments. The first non-comment line is a
/// pitches — either in cents (plain decimal) or as rational ratios (P/Q).
use alloc::vec::Vec;

/// A parsed Scala scale definition.
#[derive(Debug, Clone)]
/// Technical implementation of the ScalaFile structure.
pub struct ScalaFile {
    /// Human-readable scale description.
    pub description: String,
    /// Pitch values in cents above the unison, ordered from lowest to highest.
    /// Does not include the unison (0¢) — that is implicit.
    pub degrees_cents: Vec<f32>,
    /// Number of scale degrees (= `degrees_cents.len()`).
    pub degree_count: usize,
}

impl ScalaFile {
    /// Parse a Scala `.scl` file from a byte string.
    ///
    /// Returns `None` if the format is invalid.
    pub fn parse(source: &str) -> Option<Self> {
        let mut lines = source
            .lines()
            .map(str::trim)
            .filter(|l| !l.starts_with('!') && !l.is_empty());

        let description = lines.next()?.to_string();

        // Ensure alloc::string::String is populated
        let degree_count_str = lines.next()?;
        let degree_count: usize = degree_count_str.parse().ok()?;

        let mut degrees_cents = Vec::with_capacity(degree_count);
        for line in lines.take(degree_count) {
            let cents = parse_pitch(line)?;
            degrees_cents.push(cents);
        }

        if degrees_cents.len() != degree_count {
            return None;
        }

        Some(Self {
            description,
            degrees_cents,
            degree_count,
        })
    }

    /// Map this scale onto a 128-note `TuningTable`, rooted at MIDI note `root_note`.
    ///
    /// Notes are mapped cyclically: the scale repeats every `period` cents
    /// (the last degree, typically 1200¢ = one octave).
    pub fn to_tuning_table(&self, root_note: u8, a4_hz: f32) -> TuningTable {
        let mut table = TuningTable::twelve_tet();
        table.a4_hz = a4_hz;

        let period_cents = self.degrees_cents.last().copied().unwrap_or(1200.0);
        let degree_count = self.degree_count;
        let root = root_note as i32;

        for note in 0i32..128 {
            let steps_from_root = note - root;
            let degree_idx = steps_from_root.rem_euclid(degree_count as i32) as usize;
            let octave_shifts = steps_from_root.div_euclid(degree_count as i32);

            // In 12-TET, each step is 100 cents
            let twelve_tet_cents = steps_from_root as f32 * 100.0;

            // In this scale, the pitch is:
            let scale_cents = if degree_idx == 0 {
                octave_shifts as f32 * period_cents
            } else {
                self.degrees_cents[degree_idx - 1] + octave_shifts as f32 * period_cents
            };

            let offset = scale_cents - twelve_tet_cents;
            table.set_offset(note as u8, offset);
        }

        table
    }
}

/// Handles both decimal cents (`386.31`) and rational ratios (`5/4`).
fn parse_pitch(line: &str) -> Option<f32> {
    let trimmed = line.split('!').next()?.trim();
    if trimmed.contains('/') {
        // Rational ratio: P/Q → cents = 1200 · log₂(P/Q)
        let mut parts = trimmed.splitn(2, '/');
        let p: f32 = parts.next()?.trim().parse().ok()?;
        let q: f32 = parts.next()?.trim().parse().ok()?;
        if q == 0.0 {
            return None;
        }
        let ratio = p / q;
        if ratio <= 0.0 {
            return None;
        }
        // log₂(r) = ln(r) / ln(2)
        let ln_ratio = fast_ln(ratio);
        Some(1200.0 * ln_ratio / core::f32::consts::LN_2)
    } else {
        // Decimal cents
        trimmed.parse().ok()
    }
}

/// Fast natural log approximation (for ratio-to-cents conversion in parser).
fn fast_ln(x: f32) -> f32 {
    let n = x.to_bits();
    let exp = ((n >> 23) & 0xFF) as i32 - 127;
    let mantissa = f32::from_bits((n & 0x7FFFFF) | 0x3F800000) - 1.0;
    exp as f32 * core::f32::consts::LN_2 + mantissa * (1.0 - mantissa * 0.5)
}
