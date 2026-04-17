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

/// A simplified magnetic tape hysteresis model.
/// Simulates the 'memory' effect of magnetic particles in tape.
pub struct TapeSaturator {
    last_out: f64,
    drive: f64,
}

impl TapeSaturator {
    pub fn new(drive: f64) -> Self {
        Self { last_out: 0.0, drive }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        // Hysteresis calculation: output depends on current input and previous state
        let saturation_limit = 0.9;
        let delta = (x - self.last_out) * self.drive;
        let out = (self.last_out + delta).clamp(-saturation_limit, saturation_limit);
        
        // Soft-clipping at the limit
        let soft_out = if out.abs() > 0.7 {
            let sign = out.signum();
            let over = out.abs() - 0.7;
            sign * (0.7 + (over / (1.0 + over * over)))
        } else {
            out
        };
        
        self.last_out = soft_out;
        soft_out
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
