/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x61350a4f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/linear.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::f32;

/// Identity pass-through
#[inline(always)]
/// Technical implementation of the linear logic.
pub fn linear(x: f32) -> f32 {
    x
}

/// Standard Rectified Linear Unit
#[inline(always)]
/// Technical implementation of the relu logic.
pub fn relu(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

/// Leaky ReLU with parametric alpha
#[inline(always)]
/// Technical implementation of the leaky_relu logic.
pub fn leaky_relu(x: f32, alpha: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        x * alpha
    }
}

/// Uses fast approximation for the exponential tail.
#[inline(always)]
/// Technical implementation of the elu logic.
pub fn elu(x: f32, alpha: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        alpha * (smoothie_core::math::exp_approx(x) - 1.0)
    }
}

/// Scaled Exponential Linear Unit
#[inline(always)]
/// Technical implementation of the selu logic.
pub fn selu(x: f32) -> f32 {
    const ALPHA: f32 = 1.6732632423543772;
    const SCALE: f32 = 1.0507009873554804;

    if x > 0.0 {
        SCALE * x
    } else {
        SCALE * ALPHA * (smoothie_core::math::exp_approx(x) - 1.0)
    }
}

/// Hard Shrink activation
#[inline(always)]
/// Technical implementation of the hard_shrink logic.
pub fn hard_shrink(x: f32, lambda: f32) -> f32 {
    if x > lambda || x < -lambda {
        x
    } else {
        0.0
    }
}

/// Soft Shrink activation
#[inline(always)]
/// Technical implementation of the soft_shrink logic.
pub fn soft_shrink(x: f32, lambda: f32) -> f32 {
    if x > lambda {
        x - lambda
    } else if x < -lambda {
        x + lambda
    } else {
        0.0
    }
}
