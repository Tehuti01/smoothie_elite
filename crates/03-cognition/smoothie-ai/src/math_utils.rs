/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xed35ddfc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/math_utils.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// Math utilities for AI module (no_std compatible approximations)

use smoothie_core::math::{exp_approx, fast_log2, fast_pow, sqrt_approx};

#[inline(always)]
/// Technical implementation of the ln_approx logic.
pub fn ln_approx(x: f32) -> f32 {
    fast_log2(x) * 0.6931471805599453
}

#[inline(always)]
/// Technical implementation of the sqrt_approx logic.
pub fn sqrt_approx(x: f32) -> f32 {
    sqrt_approx(x)
}

#[inline(always)]
/// Technical implementation of the powf_approx logic.
pub fn powf_approx(base: f32, exp: f32) -> f32 {
    fast_pow(base, exp)
}

#[inline(always)]
/// Technical implementation of the round_approx logic.
pub fn round_approx(x: f32) -> f32 {
    if x > 0.0 {
        (x + 0.5).floor()
    } else {
        (x - 0.5).ceil()
    }
}

#[inline(always)]
/// Technical implementation of the exp logic.
pub fn exp(x: f32) -> f32 {
    exp_approx(x)
}
