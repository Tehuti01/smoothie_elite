/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8507288f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/loss.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Includes MSE, spectral losses, cross-entropy, and perceptual losses.
use smoothie_core::math::{exp_approx, fast_log2, floor_approx};

#[inline(always)]
/// Technical implementation of the fast_ln logic.
fn fast_ln(x: f32) -> f32 {
    if x <= 0.0 {
        return -1e10;
    }
    fast_log2(x) * 0.6931471805599453
}

#[inline(always)]
/// Technical implementation of the fast_round logic.
fn fast_round(x: f32) -> f32 {
    if x > 0.0 {
        floor_approx(x + 0.5)
    } else {
        -floor_approx(-x + 0.5)
    }
}

/// Mean Squared Error - standard regression loss.
#[inline]
/// Technical implementation of the mse logic.
pub fn mse(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let diff = pred[i] - target[i];
        sum += diff * diff;
    }
    sum / pred.len() as f32
}

/// Technical implementation of the mse_grad logic.
pub fn mse_grad(pred: &[f32], target: &[f32], grad: &mut [f32]) {
    let scale = 2.0 / pred.len() as f32;
    for i in 0..pred.len() {
        grad[i] = scale * (pred[i] - target[i]);
    }
}

/// Mean Absolute Error - robust to outliers.
#[inline]
/// Technical implementation of the mae logic.
pub fn mae(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        sum += (pred[i] - target[i]).abs();
    }
    sum / pred.len() as f32
}

/// Technical implementation of the huber logic.
pub fn huber(pred: &[f32], target: &[f32], delta: f32) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let diff = (pred[i] - target[i]).abs();
        if diff <= delta {
            sum += 0.5 * diff * diff;
        } else {
            sum += delta * diff - 0.5 * delta * delta;
        }
    }
    sum / pred.len() as f32
}

/// Technical implementation of the cross_entropy logic.
pub fn cross_entropy(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let p = pred[i].max(1e-7).min(1.0 - 1e-7);
        sum -= target[i] * fast_ln(p);
    }
    sum / pred.len() as f32
}

/// Technical implementation of the binary_cross_entropy logic.
pub fn binary_cross_entropy(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let p = pred[i].max(1e-7).min(1.0 - 1e-7);
        sum -= target[i] * fast_ln(p) + (1.0 - target[i]) * fast_ln(1.0 - p);
    }
    sum / pred.len() as f32
}

/// Technical implementation of the spectral_mse logic.
pub fn spectral_mse(pred: &[f32], target: &[f32], fft_size: usize) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..fft_size {
        if i < pred.len() && i < target.len() {
            let diff = pred[i] - target[i];
            sum += diff * diff;
        }
    }
    sum / fft_size as f32
}

/// Technical implementation of the log_spectral_dist logic.
pub fn log_spectral_dist(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let p = pred[i].max(1e-10);
        let t = target[i].max(1e-10);
        sum += t / p - fast_ln(t / p) - 1.0;
    }
    sum / pred.len() as f32
}

/// Technical implementation of the spectral_l1 logic.
pub fn spectral_l1(pred_fft: &[f32], target_fft: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred_fft.len().min(target_fft.len()) {
        sum += (pred_fft[i] - target_fft[i]).abs();
    }
    sum / pred_fft.len().max(1) as f32
}

/// Technical implementation of the MultiScaleSpectralLoss structure.
pub struct MultiScaleSpectralLoss {
    pub fft_sizes: [usize; 3],
    pub hop_sizes: [usize; 3],
}

impl MultiScaleSpectralLoss {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            fft_sizes: [512, 1024, 2048],
            hop_sizes: [128, 256, 512],
        }
    }

    /// Technical implementation of the compute logic.
    pub fn compute(&self, pred: &[f32], target: &[f32]) -> f32 {
        let mut total = 0.0f32;
        for i in 0..3 {
            total += spectral_mse(pred, target, self.fft_sizes[i]);
        }
        total / 3.0
    }
}

/// Technical implementation of the perceptual_loss logic.
pub fn perceptual_loss(pred_features: &[f32], target_features: &[f32]) -> f32 {
    mse(pred_features, target_features)
}

/// Technical implementation of the dice_loss logic.
pub fn dice_loss(pred: &[f32], target: &[f32]) -> f32 {
    let mut intersection = 0.0f32;
    let mut pred_sum = 0.0f32;
    let mut target_sum = 0.0f32;

    for i in 0..pred.len() {
        intersection += pred[i] * target[i];
        pred_sum += pred[i];
        target_sum += target[i];
    }

    1.0 - (2.0 * intersection + 1.0) / (pred_sum + target_sum + 1.0)
}

/// Technical implementation of the kl_divergence logic.
pub fn kl_divergence(pred: &[f32], target: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for i in 0..pred.len() {
        let p = pred[i].max(1e-7);
        let t = target[i].max(1e-7);
        sum += t * (fast_ln(t) - fast_ln(p));
    }
    sum / pred.len() as f32
}
