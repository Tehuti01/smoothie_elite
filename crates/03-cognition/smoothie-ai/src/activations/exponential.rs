/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb0b2b0b5 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/exponential.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::exp_approx;
// use smoothie_core::math::FloatExt;

/// Sigmoid is bounded `[0, 1]` based on logistic bounds.
#[inline(always)]
/// Technical implementation of the sigmoid logic.
pub fn sigmoid(x: f32) -> f32 {
    0.5 * (x / (1.0 + x.abs())) + 0.5
}

/// Hard Sigmoid - Even faster piecewise bounding without polynomials.
#[inline(always)]
/// Technical implementation of the hard_sigmoid logic.
pub fn hard_sigmoid(x: f32) -> f32 {
    let scaled = x * 0.2 + 0.5;
    if scaled > 1.0 {
        1.0
    } else if scaled < 0.0 {
        0.0
    } else {
        scaled
    }
}

/// SiLU / Swish Activation. `x * sigmoid(x)`
#[inline(always)]
/// Technical implementation of the swish logic.
pub fn swish(x: f32) -> f32 {
    x * sigmoid(x)
}

/// Uses approximation curves since `no_std` `ln()` is expensive.
#[inline(always)]
/// Technical implementation of the softplus logic.
pub fn softplus(x: f32) -> f32 {
    // Quick heuristic approximation to skip logarithm when x is large
    if x > 15.0 {
        x
    } else {
        // We use a custom fast log approximation or fallback to piecewise scaling for audio.
        // x < 0: exp(x). x > 0: x + exp(-x) approximately.
        if x < 0.0 {
            exp_approx(x)
        } else {
            x + exp_approx(-x) * 0.5 // Simplified fast heuristic
        }
    }
}

/// Mish Activation `x * tanh(softplus(x))`
#[inline(always)]
/// Technical implementation of the mish logic.
pub fn mish(x: f32) -> f32 {
    x * super::trigonometric::tanh(softplus(x))
}

/// GELU (Gaussian Error Linear Unit). Heavy use in transformer topologies.
#[inline(always)]
/// Technical implementation of the gelu logic.
pub fn gelu(x: f32) -> f32 {
    // Fast GELU approximation: x * sigmoid(1.702 * x)
    x * sigmoid(1.702 * x)
}

/// Softsign activation
#[inline(always)]
/// Technical implementation of the softsign logic.
pub fn softsign(x: f32) -> f32 {
    x / (1.0 + x.abs())
}
