/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x389c2c59 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/compressor/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::detector::{DetectionMode, LevelDetector};
use super::gain_computer::GainComputer;
use smoothie_core::math::FloatExt;

/// Analogue topology style influencing detector and saturation behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the CompressorStyle enumeration.
pub enum CompressorStyle {
    /// Precise VCA-style gain reduction based on feed-forward topology.
    Vca,
    /// Fast FET-driven behavior with non-linear saturation characteristics.
    Fet,
    /// Frequency-dependent optical cell behavior with multi-stage release.
    Optical,
    /// Variable-mu tube behavior with input-dependent harmonic saturation.
    VariMu,
}

/// Configuration parameters for the dynamics processor.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the CompressorParams structure.
pub struct CompressorParams {
    /// Level at which compression begins (dB).
    pub threshold_db: f32,
    /// Compression intensity ratio (x:1).
    pub ratio: f32,
    /// Transition softness around the threshold (dB).
    pub knee_db: f32,
    /// Time taken for gain reduction to reach target level (ms).
    pub attack_ms: f32,
    /// Time taken for gain to return to unity (ms).
    pub release_ms: f32,
    /// Final gain compensation applied post-compression (dB).
    pub makeup_db: f32,
    /// Input drive saturation amount; primarily affects FET and VariMu styles.
    pub drive: f32,
    /// Dry/Wet blend ratio for parallel compression processing.
    pub mix: f32,
}

impl Default for CompressorParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            threshold_db: -12.0,
            ratio: 4.0,
            knee_db: 6.0,
            attack_ms: 10.0,
            release_ms: 100.0,
            makeup_db: 0.0,
            drive: 0.0,
            mix: 1.0,
        }
    }
}

/// (dual-mono) unless the user enables sidechain linking.
/// Technical implementation of the Compressor structure.
pub struct Compressor {
    style: CompressorStyle,
    params: CompressorParams,
    detector_l: LevelDetector,
    detector_r: LevelDetector,
    gain_l: f32,
    gain_r: f32,
    sample_rate: f32,
}

impl Compressor {
    /// Initializes a new instance of the associated type.
    pub fn new(style: CompressorStyle, params: CompressorParams, sample_rate: f32) -> Self {
        let mode = match style {
            CompressorStyle::Vca | CompressorStyle::Fet => DetectionMode::Peak,
            CompressorStyle::Optical => DetectionMode::Rms,
            CompressorStyle::VariMu => DetectionMode::Hybrid {
                peak_weight_pct: 30,
            },
        };

        Self {
            style,
            sample_rate,
            gain_l: 1.0,
            gain_r: 1.0,
            detector_l: LevelDetector::new(mode, params.attack_ms, params.release_ms, sample_rate),
            detector_r: LevelDetector::new(mode, params.attack_ms, params.release_ms, sample_rate),
            params,
        }
    }

    /// Update parameters at any time (thread-safe only from audio thread).
    pub fn set_params(&mut self, params: CompressorParams) {
        self.params = params;
    }

    /// Processes a single stereo sample pair through the compression engine.
    ///
    /// # Arguments
    /// * `in_l` - Left input sample.
    /// * `in_r` - Right input sample.
    ///
    /// Returns a tuple containing the processed (left, right) samples.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let computer = GainComputer::new(
            self.params.threshold_db,
            self.params.ratio,
            self.params.knee_db,
            self.params.makeup_db,
        );

        let level_l = self.detector_l.process(in_l);
        let level_r = self.detector_r.process(in_r);

        let level_db_l = linear_to_db(level_l);
        let level_db_r = linear_to_db(level_r);

        let target_gain_l = computer.compute(level_db_l);
        let target_gain_r = computer.compute(level_db_r);

        // Ballistic smoothing — separate from the detector, applied on the gain signal
        self.gain_l = self.gain_l + (target_gain_l - self.gain_l) * 0.001;
        self.gain_r = self.gain_r + (target_gain_r - self.gain_r) * 0.001;

        let drive_sat = |x: f32, d: f32| -> f32 {
            if d < 1e-5 {
                return x;
            }
            // Soft saturation via tanh-like rational approximation
            let driven = x * (1.0 + d * 4.0);
            driven / (1.0 + driven.abs())
        };

        let out_l_dry = in_l;
        let out_r_dry = in_r;
        let out_l_wet = drive_sat(in_l * self.gain_l, self.params.drive);
        let out_r_wet = drive_sat(in_r * self.gain_r, self.params.drive);

        let mix = self.params.mix;
        (
            out_l_dry * (1.0 - mix) + out_l_wet * mix,
            out_r_dry * (1.0 - mix) + out_r_wet * mix,
        )
    }

    /// Process an entire stereo block in-place.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        debug_assert_eq!(left.len(), right.len());
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (ol, or_) = self.process_stereo(*l, *r);
            *l = ol;
            *r = or_;
        }
    }

    /// Returns current gain reduction in dB for metering display.
    pub fn gain_reduction_db(&self) -> (f32, f32) {
        (linear_to_db(self.gain_l), linear_to_db(self.gain_r))
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.detector_l.reset();
        self.detector_r.reset();
        self.gain_l = 1.0;
        self.gain_r = 1.0;
    }
}

#[inline(always)]
/// Technical implementation of the linear_to_db logic.
fn linear_to_db(x: f32) -> f32 {
    if x <= 1e-9 {
        return -180.0;
    }
    // 20 * log10(x) ≈ 20 / ln(10) * ln(x) ≈ 8.6858896 * ln(x)
    // ln approximation for small values
    let approx_ln = {
        let n = x.to_bits();
        let exponent = ((n >> 23) & 0xFF) as i32 - 127;
        let mantissa_bits = (n & 0x7FFFFF) | 0x3F800000;
        let mantissa = f32::from_bits(mantissa_bits) - 1.0;
        exponent as f32 * core::f32::consts::LN_2 + mantissa * (1.0 - mantissa * 0.5)
    };
    8.685889638_f32 * approx_ln
}
