/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5541d73a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/graphic_eq.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Fixed-band parametric EQ for visual/mixing applications.
extern crate alloc;

use super::bands::BandType;
use super::filters::{BiquadCoeffs, BiquadFilter};

/// ISO 1/3 octave center frequencies (31 bands).
pub const ISO_1_3_OCTAVE_FREQS: [f64; 31] = [
    20.0, 25.0, 31.5, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 315.0, 400.0,
    500.0, 630.0, 800.0, 1000.0, 1250.0, 1600.0, 2000.0, 2500.0, 3150.0, 4000.0, 5000.0, 6300.0,
    8000.0, 10000.0, 12500.0, 16000.0, 20000.0,
];

/// Number of graphic EQ bands.
pub const NUM_GRAPHIC_BANDS: usize = 31;

/// Graphic EQ band configuration.
#[derive(Clone, Copy, Debug)]
#[repr(align(64))]
/// Technical implementation of the GraphicEqBand structure.
pub struct GraphicEqBand {
    pub freq_hz: f64,
    pub gain_db: f64,
    pub enabled: bool,
}

impl Default for GraphicEqBand {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            freq_hz: 1000.0,
            gain_db: 0.0,
            enabled: true,
        }
    }
}

/// 31-band 1/3 octave graphic equalizer.
#[repr(align(64))]
/// Technical implementation of the GraphicEq structure.
pub struct GraphicEq {
    bands: [GraphicEqBand; NUM_GRAPHIC_BANDS],
    filters: [BiquadFilter; NUM_GRAPHIC_BANDS],
    sample_rate: f64,
    dirty: bool,
}

impl GraphicEq {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f64) -> Self {
        let mut filters = [const { BiquadFilter::const_default() }; NUM_GRAPHIC_BANDS];

        for (i, filter) in filters.iter_mut().enumerate() {
            filter.set_coeffs(
                BandType::Peaking {
                    freq_hz: ISO_1_3_OCTAVE_FREQS[i],
                    gain_db: 0.0,
                    q: 4.0,
                }
                .compute_coeffs(sample_rate),
            );
        }

        let mut bands = [GraphicEqBand::default(); NUM_GRAPHIC_BANDS];
        for (i, band) in bands.iter_mut().enumerate() {
            band.freq_hz = ISO_1_3_OCTAVE_FREQS[i];
        }

        Self {
            bands,
            filters,
            sample_rate,
            dirty: false,
        }
    }

    /// Set gain for a specific band [0.0, 1.0] normalized band index.
    pub fn set_band_gain(&mut self, index: usize, gain_db: f64) {
        if index < NUM_GRAPHIC_BANDS {
            self.bands[index].gain_db = gain_db.clamp(-12.0, 12.0);
            self.dirty = true;
        }
    }

    /// Get gain for a specific band.
    pub fn get_band_gain(&self, index: usize) -> f64 {
        if index < NUM_GRAPHIC_BANDS {
            self.bands[index].gain_db
        } else {
            0.0
        }
    }

    /// Recompute filter coefficients if dirty.
    fn recompute_if_dirty(&mut self) {
        if !self.dirty {
            return;
        }

        for (i, band) in self.bands.iter().enumerate() {
            if band.enabled && band.gain_db.abs() > 0.01 {
                let coeffs = BandType::Peaking {
                    freq_hz: band.freq_hz,
                    gain_db: band.gain_db,
                    q: 4.0,
                }
                .compute_coeffs(self.sample_rate);
                self.filters[i].set_coeffs(coeffs);
            } else {
                self.filters[i].set_coeffs(BiquadCoeffs::IDENTITY);
            }
        }
        self.dirty = false;
    }

    /// Process a stereo block.
    #[inline(always)]
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.recompute_if_dirty();
        debug_assert_eq!(left.len(), right.len());

        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let mut sample = *l;
            for filter in self.filters.iter_mut() {
                sample = filter.process_single(sample);
            }
            *l = sample;

            sample = *r;
            for filter in self.filters.iter_mut() {
                sample = filter.process_single(sample);
            }
            *r = sample;
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for filter in self.filters.iter_mut() {
            filter.reset();
        }
    }
}
