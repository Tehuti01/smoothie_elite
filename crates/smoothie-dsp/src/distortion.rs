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
pub fn foldback(x: f32, threshold: f32) -> f32 {
    if x.abs() <= threshold { return x; }
    let sign = x.signum();
    let t2 = threshold * 2.0;
    let x_wrap = ((x - sign * threshold) % t2 + t2) % t2;
    sign * (threshold - (x_wrap - threshold).abs())
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
