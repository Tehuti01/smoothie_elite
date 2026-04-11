//! Fast approximations for common math functions used in real-time DSP.
//! All functions trade a small amount of accuracy for speed.

use std::f32::consts::{PI, FRAC_PI_2};

/// Fast sine approximation via Bhaskara I's formula.
/// Max error ≈ 0.00165 rad. Valid for x in [0, π].
#[inline]
pub fn fast_sin(x: f32) -> f32 {
    // Wrap x to [-π, π]
    let x = wrap_pi(x);
    let abs_x = x.abs();
    let numer = 16.0 * x * (PI - abs_x);
    let denom = 5.0 * PI * PI - 4.0 * abs_x * (PI - abs_x);
    numer / denom
}

/// Fast cosine via phase shift.
#[inline]
pub fn fast_cos(x: f32) -> f32 {
    fast_sin(x + FRAC_PI_2)
}

/// Fast tanh approximation (Padé-like). Accurate within ~0.5% for |x| < 4.
#[inline]
pub fn fast_tanh(x: f32) -> f32 {
    let x2 = x * x;
    let n = x * (27.0 + x2);
    let d = 27.0 + 9.0 * x2;
    (n / d).clamp(-1.0, 1.0)
}

/// Faster tanh using a 5th-order polynomial (|x| must be < ~3 for accuracy).
#[inline]
pub fn fast_tanh5(x: f32) -> f32 {
    let x = x.clamp(-3.0, 3.0);
    let x2 = x * x;
    x * (1.0 + x2 * (-0.3333 + x2 * 0.1333))
}

/// Fast exp2 approximation.
#[inline]
pub fn fast_exp2(x: f32) -> f32 {
    let n = x.floor() as i32;
    let f = x - n as f32;
    // Polynomial for 2^f in [0,1]: coefficients by minimax
    let p = 1.0 + f * (0.693_147 + f * (0.240_227 + f * (0.055_504 + f * 0.009_618)));
    f32::from_bits(((n + 127) as u32) << 23) * p
}

/// Fast log2 approximation.
#[inline]
pub fn fast_log2(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp  = ((bits >> 23) as i32) - 127;
    let mant = f32::from_bits((bits & 0x7F_FFFF) | 0x3F80_0000);
    exp as f32 + (mant - 1.0) * (1.0 / core::f32::consts::LN_2) * mant.ln()
}

/// Fast natural log using fast_log2 and change of base.
#[inline]
pub fn fast_ln(x: f32) -> f32 {
    fast_log2(x) * core::f32::consts::LN_2
}

/// Fast integer power.
#[inline]
pub fn fast_powi(mut base: f32, mut exp: u32) -> f32 {
    let mut result = 1.0f32;
    while exp > 0 {
        if exp & 1 == 1 { result *= base; }
        base *= base;
        exp >>= 1;
    }
    result
}

/// Wrap angle to [-π, π].
#[inline]
pub fn wrap_pi(x: f32) -> f32 {
    let x = x % (2.0 * PI);
    if x > PI { x - 2.0 * PI }
    else if x < -PI { x + 2.0 * PI }
    else { x }
}

/// Wrap angle to [0, 2π].
#[inline]
pub fn wrap_tau(x: f32) -> f32 {
    let tau = 2.0 * PI;
    ((x % tau) + tau) % tau
}

/// Soft-knee sigmoid via fast_tanh.
#[inline]
pub fn sigmoid(x: f32) -> f32 {
    0.5 + 0.5 * fast_tanh(x)
}

/// Smoothstep (Ken Perlin): 3t²-2t³, t clamped to [lo, hi].
#[inline]
pub fn smoothstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep: 6t⁵-15t⁴+10t³.
#[inline]
pub fn smootherstep(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
