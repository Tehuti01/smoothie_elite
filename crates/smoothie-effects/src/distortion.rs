//! Distortion and saturation effect.

use crate::EffectProcessor;

/// Distortion/Saturation processor.
///
/// Provides soft clipping, hard clipping, and asymmetric saturation.
pub struct DistortionProcessor {
    drive: f32,
    tone: f32,
    output_level: f32,
    kind: DistortionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistortionType {
    /// Soft knee saturation (smooth)
    SoftClip,
    /// Hard clipping (abrupt)
    HardClip,
    /// Asymmetric saturation (warm)
    Asymmetric,
    /// Tanh saturation (natural)
    Tanh,
}

impl DistortionProcessor {
    /// Create a new distortion processor.
    pub fn new(kind: DistortionType) -> Self {
        Self {
            drive: 1.0,
            tone: 0.5,
            output_level: 1.0,
            kind,
        }
    }

    /// Set drive amount (0.0–10.0, where 1.0 is unity).
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 10.0);
    }

    /// Set tone shaping (0.0–1.0).
    pub fn set_tone(&mut self, tone: f32) {
        self.tone = tone.clamp(0.0, 1.0);
    }

    /// Set output level for makeup gain.
    pub fn set_output(&mut self, level: f32) {
        self.output_level = level.clamp(0.0, 2.0);
    }

    /// Set distortion type.
    pub fn set_kind(&mut self, kind: DistortionType) {
        self.kind = kind;
    }

    // Soft knee saturation curve
    fn soft_clip(&self, x: f32) -> f32 {
        let driven = x * self.drive;
        if driven > 1.0 {
            2.0 / 3.0 // Asymptotic to 2/3
        } else if driven < -1.0 {
            -2.0 / 3.0
        } else {
            driven - (driven * driven * driven) / 3.0
        }
    }

    // Hard clipping
    fn hard_clip(&self, x: f32) -> f32 {
        (x * self.drive).clamp(-1.0, 1.0)
    }

    // Asymmetric saturation (steeper positive slope)
    fn asymmetric(&self, x: f32) -> f32 {
        let driven = x * self.drive;
        let pos_coeff = 0.5 + self.tone * 0.5; // 0.5–1.0
        let neg_coeff = 0.5 + (1.0 - self.tone) * 0.5; // 0.5–1.0

        if driven > 0.0 {
            (driven / (1.0 + pos_coeff * driven.abs())).min(1.0)
        } else {
            (driven / (1.0 + neg_coeff * driven.abs())).max(-1.0)
        }
    }

    // Hyperbolic tangent saturation
    fn tanh(&self, x: f32) -> f32 {
        let driven = x * self.drive;
        driven.tanh() // Natural saturation
    }
}

impl EffectProcessor for DistortionProcessor {
    fn process(&mut self, input: f32) -> f32 {
        let distorted = match self.kind {
            DistortionType::SoftClip => self.soft_clip(input),
            DistortionType::HardClip => self.hard_clip(input),
            DistortionType::Asymmetric => self.asymmetric(input),
            DistortionType::Tanh => self.tanh(input),
        };
        distorted * self.output_level
    }

    fn reset(&mut self) {
        // No state to reset
    }

    fn set_sample_rate(&mut self, _sr: f32) {
        // Distortion doesn't need sample rate
    }
}

impl Default for DistortionProcessor {
    fn default() -> Self {
        Self::new(DistortionType::SoftClip)
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
