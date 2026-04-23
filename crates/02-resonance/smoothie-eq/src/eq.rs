/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7744b5e5 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/eq.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::bands::BandType;
use super::filters::{BiquadCoeffs, BiquadFilter};
///
/// Coefficient recomputation is triggered by a dirty flag from the parameter
/// guaranteeing zero allocation and zero locking on the hot path.
use alloc::vec::Vec;

/// Maximum number of EQ bands in a single `ParametricEq`.
pub const MAX_BANDS: usize = 32;

/// Configuration for a single EQ band.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the EqBandConfig structure.
pub struct EqBandConfig {
    pub band_type: BandType,
    pub enabled: bool,
}

impl Default for EqBandConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            band_type: BandType::Bypass,
            enabled: true,
        }
    }
}

/// Technical implementation of the EqBand structure.
pub struct EqBand {
    pub config: EqBandConfig,
    filter: BiquadFilter,
    dirty: bool,
}

impl EqBand {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            config: EqBandConfig::default(),
            filter: BiquadFilter::identity(),
            dirty: false,
        }
    }

    /// Update the band configuration. Marks the band dirty so coefficients
    /// are recomputed before the next `process_block()`.
    pub fn set_config(&mut self, config: EqBandConfig) {
        self.config = config;
        self.dirty = true;
    }

    /// Technical implementation of the recompute_if_dirty logic.
    fn recompute_if_dirty(&mut self, sample_rate: f64) {
        if self.dirty {
            let coeffs = if self.config.enabled {
                self.config.band_type.compute_coeffs(sample_rate)
            } else {
                BiquadCoeffs::IDENTITY
            };
            self.filter.set_coeffs(coeffs);
            self.dirty = false;
        }
    }
}

/// Technical implementation of the ParametricEq structure.
pub struct ParametricEq {
    bands: Vec<EqBand>,
    sample_rate: f64,
    pub output_gain: f32,
}

impl ParametricEq {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f64) -> Self {
        let bands = (0..MAX_BANDS).map(|_| EqBand::new()).collect();
        Self {
            bands,
            sample_rate,
            output_gain: 1.0,
        }
    }

    /// Update a band configuration by index.
    ///
    /// # Panics
    /// Panics in debug mode if `index >= MAX_BANDS`.
    pub fn set_band(&mut self, index: usize, config: EqBandConfig) {
        debug_assert!(index < MAX_BANDS);
        self.bands[index].set_config(config);
    }

    /// Retrieve a reference to a band configuration.
    pub fn band(&self, index: usize) -> &EqBandConfig {
        &self.bands[index].config
    }

    /// Update the operating sample rate and invalidate all coefficients.
    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
        for band in self.bands.iter_mut() {
            band.dirty = true;
        }
    }

    /// Process a complete stereo block through all enabled EQ bands.
    ///
    /// Recomputation of dirty coefficients occurs inline before block processing,
    /// meaning the very first block after a parameter change will use the updated
    /// coefficients. There is no one-block delay.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        debug_assert_eq!(left.len(), right.len());
        let sr = self.sample_rate;

        for band in self.bands.iter_mut() {
            if !band.config.enabled {
                continue;
            }
            band.recompute_if_dirty(sr);
            band.filter.process_block_stereo(left, right);
        }

        if (self.output_gain - 1.0).abs() > 1e-6 {
            let g = self.output_gain;
            for (l, r) in left.iter_mut().zip(right.iter_mut()) {
                *l *= g;
                *r *= g;
            }
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for band in self.bands.iter_mut() {
            band.filter.reset();
        }
    }
}
