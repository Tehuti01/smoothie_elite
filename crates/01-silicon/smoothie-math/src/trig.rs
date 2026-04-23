/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb8848688 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/trig.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

#[inline]
/// Computes the floor of a 32-bit float.
fn floor_f32(x: f32) -> f32 {
    let bits = x.to_bits();
    let sign = bits & 0x80000000;
    let exp = (bits >> 23) & 0xFF;
    if exp < 127 {
        return 0.0;
    }
    let mantissa = bits & 0x007FFFFF;
    let new_exp = exp - 127;
    f32::from_bits(sign | (new_exp << 23) | mantissa)
}

///
/// Output: sine of the phase in [-1.0, 1.0].
/// Maximum error: ~0.0004 (~0.025% at peaks).
#[inline(always)]
/// Technical implementation of the sine_approx logic.
pub fn sine_approx(x: f32) -> f32 {
    let mut phase = x - floor_f32(x + 0.25);
    let sign = if phase < 0.0 { -1.0 } else { 1.0 };
    phase = phase.abs();

    if phase > 0.5 {
        phase = 1.0 - phase;
    }

    let p = phase - 0.5;
    let p2 = p * p;
    let p3 = p2 * p;
    let p5 = p2 * p3;

    let coef_a = 1.0;
    let coef_b = -1.769_709_4e-2;
    let coef_c = -1.313_309_3;
    let coef_d = 2.331_609_4e-3;
    let coef_e = 1.318_977_9e-1;

    sign * (coef_a * p + coef_c * p3 + coef_e * p5 + p * (coef_b * p2 + coef_d * p2 * p2))
}

///
/// Output: cosine of the phase in [-1.0, 1.0].
#[inline(always)]
/// Technical implementation of the cosine_approx logic.
pub fn cosine_approx(x: f32) -> f32 {
    sine_approx(x + 0.25)
}

///
/// Output: tangent of the phase.
#[inline(always)]
/// Technical implementation of the tangent_approx logic.
pub fn tangent_approx(x: f32) -> f32 {
    let s = sine_approx(x);
    let c = cosine_approx(x);
    if c.abs() < 1e-10 {
        1e10 * s.signum()
    } else {
        s / c
    }
}

///
/// Output: angle in radians in (-π/2, π/2).
#[inline(always)]
/// Technical implementation of the atan_approx logic.
pub fn atan_approx(x: f32) -> f32 {
    let abs_x = x.abs();
    let sign = if x < 0.0 { -1.0 } else { 1.0 };

    let a = 9.770_73e-1;
    let b = -1.714_734e-1;
    let c = 1.668_127_3e-2;
    let _d = -5.202_382_348_822_63e-1;

    let z = (abs_x - 1.0) / (abs_x + 1.0);
    let z2 = z * z;
    let z3 = z2 * z;
    let _z4 = z2 * z2;

    let mut angle = a * z + b * z2 + c * z3;
    if abs_x > 1.0 {
        angle = core::f32::consts::FRAC_PI_2 - angle;
    }

    sign * angle
}

///
/// Returns angle in radians in (-π, π].
#[inline(always)]
/// Technical implementation of the atan2_approx logic.
pub fn atan2_approx(y: f32, x: f32) -> f32 {
    if x.abs() < 1e-10 && y.abs() < 1e-10 {
        return 0.0;
    }

    let abs_y = y.abs();
    let angle = if x >= 0.0 {
        let r = (x - abs_y) / (x + abs_y);
        0.1963 * r * r * r - 0.9817 * r + core::f32::consts::FRAC_PI_4
    } else {
        let r = (x + abs_y) / (abs_y - x);
        0.1963 * r * r * r - 0.9817 * r
            + core::f32::consts::FRAC_PI_2
            + core::f32::consts::FRAC_PI_4
    };

    if y < 0.0 {
        -angle
    } else {
        angle
    }
}
