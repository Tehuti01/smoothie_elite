/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7dccb39f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/para_quad.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Configurable multi-band EQ with quad (4-channel) support.
extern crate alloc;

use super::bands::BandType;
use super::filters::{BiquadCoeffs, BiquadFilter};
use alloc::vec::Vec;

/// Maximum number of bands in para/quad EQ.
pub const MAX_PARA_BANDS: usize = 16;

/// A single parametric band configuration.
#[derive(Clone, Copy, Debug)]
#[repr(align(64))]
/// Technical implementation of the ParaBandConfig structure.
pub struct ParaBandConfig {
    pub band_type: BandType,
    pub enabled: bool,
    pub solo: bool,
    pub mute: bool,
}

impl Default for ParaBandConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            band_type: BandType::Bypass,
            enabled: true,
            solo: false,
            mute: false,
        }
    }
}

/// A single parametric band with filter state.
#[repr(align(64))]
/// Technical implementation of the ParaBand structure.
pub struct ParaBand {
    pub config: ParaBandConfig,
    filter: BiquadFilter,
    dirty: bool,
}

impl ParaBand {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            config: ParaBandConfig::default(),
            filter: BiquadFilter::identity(),
            dirty: true,
        }
    }

    /// Technical implementation of the recompute logic.
    fn recompute(&mut self, sample_rate: f64) {
        if self.dirty {
            let coeffs = if self.config.enabled && !self.config.mute {
                self.config.band_type.compute_coeffs(sample_rate)
            } else {
                BiquadCoeffs::IDENTITY
            };
            self.filter.set_coeffs(coeffs);
            self.dirty = false;
        }
    }
}

/// Parametric EQ (stereo).
#[repr(align(64))]
/// Technical implementation of the ParaEq structure.
pub struct ParaEq {
    bands: Vec<ParaBand>,
    sample_rate: f64,
    output_gain: f32,
}

impl ParaEq {
    /// Initializes a new instance of the associated type.
    pub fn new(num_bands: usize, sample_rate: f64) -> Self {
        let bands = (0..num_bands.min(MAX_PARA_BANDS))
            .map(|_| ParaBand::new())
            .collect();
        Self {
            bands,
            sample_rate,
            output_gain: 1.0,
        }
    }

    /// Technical implementation of the set_band logic.
    pub fn set_band(&mut self, index: usize, config: ParaBandConfig) {
        if let Some(band) = self.bands.get_mut(index) {
            band.config = config;
            band.dirty = true;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: &mut [f32], right: &mut [f32]) {
        debug_assert_eq!(left.len(), right.len());
        let sr = self.sample_rate;

        for band in self.bands.iter_mut() {
            if band.config.solo {
                band.recompute(sr);
                band.filter.process_block_stereo(left, right);
                return;
            }
        }

        for band in self.bands.iter_mut() {
            if !band.config.enabled || band.config.mute {
                continue;
            }
            band.recompute(sr);
            band.filter.process_block_stereo(left, right);
        }

        if (self.output_gain - 1.0).abs() > 1e-6 {
            for (l, r) in left.iter_mut().zip(right.iter_mut()) {
                *l *= self.output_gain;
                *r *= self.output_gain;
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

/// Quadraphonic EQ (4-channel).
#[repr(align(64))]
/// Technical implementation of the QuadEq structure.
pub struct QuadEq {
    bands: Vec<ParaBand>,
    sample_rate: f64,
    output_gain: f32,
}

impl QuadEq {
    /// Initializes a new instance of the associated type.
    pub fn new(num_bands: usize, sample_rate: f64) -> Self {
        let bands = (0..num_bands.min(MAX_PARA_BANDS))
            .map(|_| ParaBand::new())
            .collect();
        Self {
            bands,
            sample_rate,
            output_gain: 1.0,
        }
    }

    /// Technical implementation of the set_band logic.
    pub fn set_band(&mut self, index: usize, config: ParaBandConfig) {
        if let Some(band) = self.bands.get_mut(index) {
            band.config = config;
            band.dirty = true;
        }
    }

    /// Process 4-channel audio (FL, FR, RL, RR).
    #[inline(always)]
    pub fn process_quad(&mut self, fl: &mut [f32], fr: &mut [f32], rl: &mut [f32], rr: &mut [f32]) {
        let len = fl.len();
        debug_assert_eq!(len, fr.len());
        debug_assert_eq!(len, rl.len());
        debug_assert_eq!(len, rr.len());

        let sr = self.sample_rate;

        for band in self.bands.iter_mut() {
            if !band.config.enabled || band.config.mute {
                continue;
            }
            band.recompute(sr);

            for i in 0..len {
                fl[i] = band.filter.process_single(fl[i]);
                fr[i] = band.filter.process_single(fr[i]);
                rl[i] = band.filter.process_single(rl[i]);
                rr[i] = band.filter.process_single(rr[i]);
            }
        }

        if (self.output_gain - 1.0).abs() > 1e-6 {
            for i in 0..len {
                fl[i] *= self.output_gain;
                fr[i] *= self.output_gain;
                rl[i] *= self.output_gain;
                rr[i] *= self.output_gain;
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
