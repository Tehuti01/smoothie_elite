/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xaec90778 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/shelves.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::bands::BandType;
///
/// Includes a Tilt EQ model (single-knob spectral balance).
use super::filters::BiquadFilter;

/// Technical implementation of the LowShelf structure.
pub struct LowShelf {
    filter: BiquadFilter,
    freq_hz: f64,
    gain_db: f64,
    slope: f64,
    sample_rate: f64,
}

impl LowShelf {
    /// Initializes a new instance of the associated type.
    pub fn new(freq_hz: f64, gain_db: f64, slope: f64, sample_rate: f64) -> Self {
        let coeffs = BandType::LowShelf {
            freq_hz,
            gain_db,
            slope,
        }
        .compute_coeffs(sample_rate);
        Self {
            filter: BiquadFilter::new(coeffs),
            freq_hz,
            gain_db,
            slope,
            sample_rate,
        }
    }

    /// Updates a framework parameter value.
    pub fn set_params(&mut self, freq_hz: f64, gain_db: f64, slope: f64) {
        self.freq_hz = freq_hz;
        self.gain_db = gain_db;
        self.slope = slope;
        let coeffs = BandType::LowShelf {
            freq_hz,
            gain_db,
            slope,
        }
        .compute_coeffs(self.sample_rate);
        self.filter.set_coeffs(coeffs);
    }

    /// Primary real-time signal processing execution block.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.filter.process_block_stereo(left, right);
    }
}

/// Technical implementation of the HighShelf structure.
pub struct HighShelf {
    filter: BiquadFilter,
    freq_hz: f64,
    gain_db: f64,
    slope: f64,
    sample_rate: f64,
}

impl HighShelf {
    /// Initializes a new instance of the associated type.
    pub fn new(freq_hz: f64, gain_db: f64, slope: f64, sample_rate: f64) -> Self {
        let coeffs = BandType::HighShelf {
            freq_hz,
            gain_db,
            slope,
        }
        .compute_coeffs(sample_rate);
        Self {
            filter: BiquadFilter::new(coeffs),
            freq_hz,
            gain_db,
            slope,
            sample_rate,
        }
    }

    /// Updates a framework parameter value.
    pub fn set_params(&mut self, freq_hz: f64, gain_db: f64, slope: f64) {
        self.freq_hz = freq_hz;
        self.gain_db = gain_db;
        self.slope = slope;
        let coeffs = BandType::HighShelf {
            freq_hz,
            gain_db,
            slope,
        }
        .compute_coeffs(self.sample_rate);
        self.filter.set_coeffs(coeffs);
    }

    /// Primary real-time signal processing execution block.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.filter.process_block_stereo(left, right);
    }
}

///
/// Negative `gain_db` darkens: boosts lows, attenuates highs.
/// Technical implementation of the TiltEq structure.
pub struct TiltEq {
    filter: BiquadFilter,
    sample_rate: f64,
}

impl TiltEq {
    /// Initializes a new instance of the associated type.
    pub fn new(pivot_hz: f64, gain_db: f64, sample_rate: f64) -> Self {
        let coeffs = BandType::Tilt {
            freq_hz: pivot_hz,
            gain_db,
        }
        .compute_coeffs(sample_rate);
        Self {
            filter: BiquadFilter::new(coeffs),
            sample_rate,
        }
    }

    /// Technical implementation of the set_tilt logic.
    pub fn set_tilt(&mut self, pivot_hz: f64, gain_db: f64) {
        let coeffs = BandType::Tilt {
            freq_hz: pivot_hz,
            gain_db,
        }
        .compute_coeffs(self.sample_rate);
        self.filter.set_coeffs(coeffs);
    }

    /// Primary real-time signal processing execution block.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.filter.process_block_stereo(left, right);
    }
}
