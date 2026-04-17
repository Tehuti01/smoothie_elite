//! Waveshaping / saturation algorithms — all pure functions, zero allocation.

/// Soft-clip using a cubic waveshaper: y = x - x³/3  (Doidic et al.)
#[inline]
pub fn softclip(x: f32) -> f32 {
    let x = x.clamp(-1.5, 1.5);
    x - (x * x * x) / 3.0
}

/// Hardclip — pure brick-wall saturation.
#[inline]
pub fn hardclip(x: f32) -> f32 { x.clamp(-1.0, 1.0) }

/// Hyperbolic tangent shaper — warm tube-like saturation.
#[inline]
pub fn tanh_shaper(x: f32) -> f32 { x.tanh() }

/// Foldback distortion — the signal folds when it exceeds threshold.
#[inline]
pub fn foldback(x: f64, threshold: f64) -> f64 {
    if x.abs() > threshold {
        let over = x.abs() - threshold;
        if x > 0.0 { threshold - over } else { -threshold + over }
    } else {
        x
    }
}

/// A physical model of a Triode Vacuum Tube characteristic.
/// Provides asymmetrical soft-clipping and rich 2nd-order harmonics.
pub fn triode_model(x: f64, gain: f64, bias: f64) -> f64 {
    let drive = x * gain + bias;
    if drive > 0.0 {
        // Power-law distortion (Child's law approximation)
        drive.powf(1.5).min(1.0) * 2.0 - 1.0
    } else {
        -1.0
    }
}

/// Asymmetric tube saturation (second-harmonic generation).
#[inline]
pub fn tube_asymmetric(x: f32, drive: f32) -> f32 {
    let x = x * drive;
    if x >= 0.0 {
        tanh_shaper(x)
    } else {
        softclip(x) * 0.8
    }
}

/// Wavefolder — generates rich upper harmonics.
#[inline]
pub fn wavefold(x: f32) -> f32 {
    let x = x * std::f32::consts::PI;
    x.sin()
}
