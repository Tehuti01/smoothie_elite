/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x90502039 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/trigonometric.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::sine_approx;
// use smoothie_core::math::FloatExt;

/// Max deviation < 0.003
#[inline(always)]
/// Technical implementation of the tanh logic.
pub fn tanh(x: f32) -> f32 {
    // Padé Approximant expansion
    let x2 = x * x;
    let a = x * (135135.0 + x2 * (17325.0 + x2 * (378.0 + x2)));
    let b = 135135.0 + x2 * (62370.0 + x2 * (3150.0 + x2 * 28.0));

    // Safety boundary clamping for extreme floats
    if b.abs() < 1e-6 {
        return x.signum();
    }

    let ratio = a / b;
    ratio.clamp(-1.0, 1.0)
}

/// Maps real phase ranges into sine_approx phase (0 to 1).
#[inline(always)]
/// Technical implementation of the sine logic.
pub fn sine(x: f32) -> f32 {
    let phase = (x / core::f32::consts::TAU) % 1.0;
    sine_approx(if phase < 0.0 { phase + 1.0 } else { phase })
}

/// Cosine activation (SIREN alternative).
#[inline(always)]
/// Technical implementation of the cosine logic.
pub fn cosine(x: f32) -> f32 {
    let phase = ((x / core::f32::consts::TAU) + 0.25) % 1.0;
    sine_approx(if phase < 0.0 { phase + 1.0 } else { phase })
}
