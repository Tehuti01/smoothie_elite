//! Seraphic Distortion & Saturation
//! Hand-coded non-linearities for harmonically-rich audio.

use crate::filters::OnePoleFilter;

/// A classic tape saturator with multi-stage non-linearity.
pub struct TapeSaturator {
    pub drive: f32,
    pub lp: OnePoleFilter,
    pub hp: OnePoleFilter,
}

impl TapeSaturator {
    pub fn new(drive: f32) -> Self {
        Self {
            drive,
            lp: OnePoleFilter::new(0.1), // Alpha coefficient
            hp: OnePoleFilter::new(0.01),
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let low = self.lp.process(x);
        let high = x - low;
        
        // Multi-stage saturation
        let saturated_low = (low * self.drive).tanh();
        let out = saturated_low + high;
        
        // Final high-pass for DC offset removal
        self.hp.process(out)
    }
}

/// Harmonic soft-clipper using the cubic polynomial approach.
pub fn softclip(x: f32) -> f32 {
    if x > 1.0 { 1.0 }
    else if x < -1.0 { -1.0 }
    else { x - (x.powi(3) / 3.0) }
}

/// Hard-clipper with bit-perfect clamping.
pub fn hardclip(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

/// Hyperbolic tangent wave shaper.
pub fn tanh_shaper(x: f32, drive: f32) -> f32 {
    (x * drive).tanh()
}

/// Recursive foldback distortion.
pub fn foldback(x: f32, threshold: f32) -> f32 {
    if x.abs() > threshold {
        let over = x.abs() - threshold;
        let folded = threshold - over;
        folded * x.signum()
    } else {
        x
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
