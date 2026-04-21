/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb7d660d2 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-tuning/src/temperaments.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::table::TuningTable;

/// A named historical temperament.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the Temperament enumeration.
pub enum Temperament {
    TwelveTet,
    Pythagorean,
    QuarterCommaMeantone,
    KirnbergerIii,
    WerckmeisterIii,
    Vallotti,
    Young,
    JustIntonation,
    Custom,
}

/// Fourth-root-of-five Comma Meantone — historically used for a pure major third.
const QUARTER_COMMA_MEANTONE_CENTS: [f32; 12] = [
    0.0,    // C
    -10.26, // C# (narrower than 12-TET)
    3.42,   // D
    -13.69, // Eb
    6.84,   // E  (pure major third above C)
    -3.42,  // F
    -13.69, // F#
    -6.84,  // G
    -17.11, // Ab
    -3.42,  // A
    -13.69, // Bb
    -6.84,  // B
];

/// Werckmeister III — the quintessential "Well-Tempered Clavier" temperament.
const WERCKMEISTER_III_CENTS: [f32; 12] = [
    0.0,     // C
    90.22,   // C#  → cents relative to C
    192.18,  // D
    294.14,  // Eb
    390.22,  // E
    498.04,  // F
    588.27,  // F#
    696.09,  // G
    792.18,  // Ab
    888.26,  // A
    996.09,  // Bb
    1092.18, // B
];

/// Vallotti temperament (Francesco Antonio Vallotti, 1779).
const VALLOTTI_CENTS: [f32; 12] = [
    0.0,     // C
    94.13,   // C#
    196.09,  // D
    298.04,  // Eb
    392.18,  // E
    501.96,  // F
    592.18,  // F#
    698.04,  // G
    796.09,  // Ab
    894.13,  // A
    999.02,  // Bb
    1094.13, // B
];

/// Cent offsets from 12-TET for the key of C major.
const JUST_INTONATION_CENTS: [f32; 12] = [
    0.0,    // C      1/1
    11.73,  // C#     16/15
    3.91,   // D      9/8
    15.64,  // Eb     6/5
    -13.69, // E      5/4
    -1.96,  // F      4/3
    -17.49, // F#     45/32
    1.96,   // G      3/2
    13.69,  // Ab     8/5
    -15.64, // A      5/3
    -17.60, // Bb     16/9
    11.73,  // B      15/8
];

/// A well-temperament mode — controls which key is tuned most purely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the WellTemperament enumeration.
pub enum WellTemperament {
    Kirnberger,
    Werckmeister,
    Vallotti,
    Young,
}

impl Temperament {
    /// Build a `TuningTable` for this temperament in a reference key.
    ///
    /// `root` is the root note (0 = C, 1 = C#, ..., 11 = B).
    /// The table is computed for all 128 MIDI notes, wrapping the 12-note
    /// pattern across octaves.
    pub fn build_table(&self, root: u8, a4_hz: f32) -> TuningTable {
        let mut table = TuningTable::twelve_tet();
        table.a4_hz = a4_hz;

        let offsets_12 = match self {
            Temperament::TwelveTet => return table,
            Temperament::Pythagorean => pythagorean_cents(),
            Temperament::QuarterCommaMeantone => QUARTER_COMMA_MEANTONE_CENTS,
            Temperament::KirnbergerIii => kirnberger_iii_cents(),
            Temperament::WerckmeisterIii => cents_from_absolute(&WERCKMEISTER_III_CENTS),
            Temperament::Vallotti => cents_from_absolute(&VALLOTTI_CENTS),
            Temperament::Young => young_cents(),
            Temperament::JustIntonation => JUST_INTONATION_CENTS,
            Temperament::Custom => return table,
        };

        // Apply rotated pattern across all 128 notes
        for note in 0u8..=127 {
            let pitch_class = (note as i32 - root as i32).rem_euclid(12) as usize;
            table.set_offset(note, offsets_12[pitch_class]);
        }
        table
    }
}

/// Pythagorean tuning: stacked pure fifths (3:2 ratio =  701.955 cents).
fn pythagorean_cents() -> [f32; 12] {
    let fifth = 701.955_f32;
    let mut cents = [0.0f32; 12];
    // 12 fifths above C, wrapped to one octave
    let steps: [i32; 12] = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
    for (pc, &s) in steps.iter().enumerate() {
        let absolute = (s as f32 * fifth) % 1200.0;
        cents[pc] = absolute - (pc as f32 * 100.0);
    }
    cents
}

/// Kirnberger III — 3 pure major thirds from C, E, G, B.
fn kirnberger_iii_cents() -> [f32; 12] {
    [
        0.0, -9.78, 3.91, 17.60, -13.69, 3.91, -19.55, 5.87, -9.78, -1.96, 9.78, -17.60,
    ]
}

/// Young temperament (Thomas Young, 1800).
fn young_cents() -> [f32; 12] {
    [
        0.0, 93.90, 195.81, 297.81, 391.69, 499.91, 591.91, 697.73, 795.63, 893.53, 997.73, 1091.62,
    ]
}

/// Convert an absolute-cents-from-C array to offsets-from-12-TET.
fn cents_from_absolute(abs: &[f32; 12]) -> [f32; 12] {
    let mut out = [0.0f32; 12];
    for (i, &a) in abs.iter().enumerate() {
        out[i] = a - (i as f32 * 100.0);
    }
    out
}
