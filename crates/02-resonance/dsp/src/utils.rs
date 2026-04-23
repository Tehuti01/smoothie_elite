/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5574696c | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/utils.rs                               │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: High-performance DSP utilities and converters.              │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_core::constants::INV_SQRT_2;
use smoothie_core::primitives::Sample;

/// Panning laws for stereo positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanLaw {
    /// Linear panning (L + R = 1.0). Center = 0.5/0.5.
    Linear,
    /// Constant power panning (L² + R² = 1.0). Center = 0.707/0.707.
    ConstantPower,
    /// -3dB center attenuation.
    Minus3dB,
}

/// Stereo panner utility.
pub struct StereoPanner {
    pub law: PanLaw,
}

impl StereoPanner {
    pub fn new(law: PanLaw) -> Self {
        Self { law }
    }

    /// Pan a mono signal to stereo. position is [-1.0, 1.0].
    pub fn pan(&self, input: Sample, position: f32) -> (Sample, Sample) {
        let pos = (position + 1.0) * 0.5; // [0.0, 1.0]
        match self.law {
            PanLaw::Linear => {
                (input * (1.0 - pos), input * pos)
            }
            PanLaw::ConstantPower | PanLaw::Minus3dB => {
                let angle = pos * core::f32::consts::FRAC_PI_2;
                (input * angle.cos(), input * angle.sin())
            }
        }
    }
}

/// Inverts the phase of a signal.
pub struct PhaseInverter;
impl PhaseInverter {
    #[inline(always)]
    pub fn process(input: Sample) -> Sample {
        -input
    }
}

/// Sums stereo signals to mono.
pub struct MonoSum;
impl MonoSum {
    #[inline(always)]
    pub fn process(left: Sample, right: Sample) -> Sample {
        (left + right) * 0.5
    }
}

/// Converts a bipolar signal [-1, 1] to unipolar [0, 1].
pub struct BipolarToUnipolar;
impl BipolarToUnipolar {
    #[inline(always)]
    pub fn process(input: Sample) -> Sample {
        input * 0.5 + 0.5
    }
}

/// Converts a unipolar signal [0, 1] to bipolar [-1, 1].
pub struct UnipolarToBipolar;
impl UnipolarToBipolar {
    #[inline(always)]
    pub fn process(input: Sample) -> Sample {
        input * 2.0 - 1.0
    }
}

/// A simple, low-CPU noise gate.
pub struct SimpleGate {
    threshold: f32,
}

impl SimpleGate {
    pub fn new(threshold_db: f32) -> Self {
        Self {
            threshold: smoothie_core::math::db_to_amplitude(threshold_db),
        }
    }

    #[inline(always)]
    pub fn process(&self, input: Sample) -> Sample {
        if input.abs() > self.threshold {
            input
        } else {
            0.0
        }
    }
}

/// Soft-clipper with adjustable knee.
pub struct SoftClipper {
    pub knee: f32,
}

impl SoftClipper {
    pub fn new(knee: f32) -> Self {
        Self { knee: knee.clamp(0.01, 1.0) }
    }

    #[inline(always)]
    pub fn process(&self, input: Sample) -> Sample {
        if input.abs() < (1.0 - self.knee) {
            input
        } else {
            let sign = if input >= 0.0 { 1.0 } else { -1.0 };
            sign * (input.abs() - self.knee * (input.abs() - (1.0 - self.knee)).powi(2) / (4.0 * self.knee))
                .min(1.0)
        }
    }
}

/// Bit depth converter for dithering and quantization.
pub struct BitDepthConverter;
impl BitDepthConverter {
    /// Quantizes a float sample to a specific bit depth.
    pub fn quantize(input: Sample, bits: u32) -> Sample {
        let levels = (1u32 << (bits - 1)) as f32;
        (input * levels).round() / levels
    }
}
