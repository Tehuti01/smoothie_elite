/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd92b9409 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/extended.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

// use smoothie_core::math::FloatExt;
///
/// GELU variants, SwiGLU, FReLU, and more specialized activations.
use smoothie_core::math::{exp_approx, fast_log2, sqrt_approx, tanh_approx};

#[inline(always)]
/// Technical implementation of the gelu_erf logic.
pub fn gelu_erf(x: f32) -> f32 {
    0.5 * x * (1.0 + erf_approx(x))
}

#[inline(always)]
/// Technical implementation of the erf_approx logic.
fn erf_approx(x: f32) -> f32 {
    let a1 = 0.254_829_6;
    let a2 = -0.284_496_72;
    let a3 = 1.421_413_8;
    let a4 = -1.453_152_1;
    let a5 = 1.061_405_4;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x_abs = x.abs();
    let t = 1.0 / (1.0 + p * x_abs);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * exp_approx(-x_abs * x_abs);
    sign * y
}

#[inline(always)]
/// Technical implementation of the gelu_tanh logic.
pub fn gelu_tanh(x: f32) -> f32 {
    let k = 0.797_884_6;
    let v = 1.0 + exp_approx(k * x);
    0.5 * x * (v - 1.0) / v
}

#[inline(always)]
/// Technical implementation of the swiglu logic.
pub fn swiglu(x: f32) -> f32 {
    let half = x * 0.5;
    let gate = tanh_approx(half * 2.0 / (1.0 + (half * 2.0).abs()));
    half * (1.0 + gate)
}

#[inline(always)]
/// Technical implementation of the frelu logic.
pub fn frelu(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        x * x
    }
}

#[inline(always)]
/// Technical implementation of the aplu logic.
pub fn aplu(x: f32, n: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        x * (1.0 + n)
    }
}

#[inline(always)]
/// Technical implementation of the square_relu logic.
pub fn square_relu(x: f32) -> f32 {
    if x > 0.0 {
        x * x
    } else {
        0.0
    }
}

#[inline(always)]
/// Technical implementation of the cubic logic.
pub fn cubic(x: f32) -> f32 {
    x * x * x
}

#[inline(always)]
/// Technical implementation of the bent_identity logic.
pub fn bent_identity(x: f32) -> f32 {
    sqrt_approx(x * x + 1.0) * 0.5 + x * 0.5 - 0.5
}

#[inline(always)]
/// Technical implementation of the hard_silu logic.
pub fn hard_silu(x: f32) -> f32 {
    let s = x * 0.2 + 0.5;
    x * if s > 1.0 {
        1.0
    } else if s < 0.0 {
        0.0
    } else {
        s
    }
}

#[inline(always)]
/// Technical implementation of the log_sigmoid logic.
pub fn log_sigmoid(x: f32) -> f32 {
    -fast_log2(1.0 + exp_approx(-x))
}

#[inline(always)]
/// Technical implementation of the mish_composite logic.
pub fn mish_composite(x: f32) -> f32 {
    let sp = softplus_composite(x);
    x * (sp * sp / (1.0 + sp * sp))
}

#[inline(always)]
/// Technical implementation of the softplus_composite logic.
fn softplus_composite(x: f32) -> f32 {
    if x > 15.0 {
        x
    } else {
        x + exp_approx(-x) * 0.5
    }
}

#[inline(always)]
/// Technical implementation of the hard_mish logic.
pub fn hard_mish(x: f32) -> f32 {
    if x > -2.0 {
        x
    } else {
        x * x * 0.5 + x + 1.0
    }
}

#[inline(always)]
/// Technical implementation of the smooth_relu logic.
pub fn smooth_relu(x: f32) -> f32 {
    if x > 10.0 {
        x
    } else {
        fast_log2(1.0 + exp_approx(x))
    }
}

#[inline(always)]
/// Technical implementation of the phish logic.
pub fn phish(x: f32) -> f32 {
    x * (x * x * x + x * x + x + 1.0)
}
