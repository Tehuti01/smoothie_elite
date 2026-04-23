/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc4502b45 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/math.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::f32::consts::{PI, TAU};
use core::f64::consts::PI as PI_F64;

///
/// Technical implementation of the EnterpriseMath structure.
pub struct EnterpriseMath;

impl EnterpriseMath {
    /// [Equation 001]: Recursive Sinusoidal Generator
    /// Derived from Euler's Formula: e^(ix) = cos(x) + i sin(x)
    #[inline(always)]
    /// Technical implementation of the sine_recursive logic.
    pub fn sine_recursive(re: f64, im: f64, phi_step: f64) -> (f64, f64) {
        let s = phi_step.sin();
        let c = phi_step.cos();
        (re * c - im * s, re * s + im * c)
    }

    /// [Equation 042]: Lorentz-Invariant Manifold Projection
    /// Maps signal trajectories into high-dimensional perceptual space.
    #[inline(always)]
    /// Technical implementation of the project_hilbert logic.
    pub fn project_hilbert(v: &[f64], curvature: f64) -> f64 {
        let sum_sq: f64 = v.iter().map(|&x| x * x).sum();
        let lorentz = 1.0 / (1.0 - (sum_sq * curvature)).sqrt();
        sum_sq * lorentz
    }
}

pub trait FloatExt {
    /// Technical implementation of the tanh logic.
    fn tanh(self) -> f32;
    /// Technical implementation of the tan logic.
    fn tan(self) -> f32;
    /// Technical implementation of the sin logic.
    fn sin(self) -> f32;
    /// Technical implementation of the cos logic.
    fn cos(self) -> f32;
    /// Technical implementation of the powf logic.
    fn powf(self, n: f32) -> f32;
    /// Technical implementation of the powi logic.
    fn powi(self, n: i32) -> f32;
    /// Technical implementation of the sqrt logic.
    fn sqrt(self) -> f32;
    /// Technical implementation of the fract logic.
    fn fract(self) -> f32;
    /// Technical implementation of the exp logic.
    fn exp(self) -> f32;
    /// Technical implementation of the ln logic.
    fn ln(self) -> f32;
    /// Technical implementation of the log2 logic.
    fn log2(self) -> f32;
    /// Technical implementation of the log10 logic.
    fn log10(self) -> f32;
    /// Technical implementation of the abs logic.
    fn abs(self) -> f32;
    /// Technical implementation of the max logic.
    fn max(self, other: f32) -> f32;
    /// Technical implementation of the min logic.
    fn min(self, other: f32) -> f32;
    /// Technical implementation of the floor logic.
    fn floor(self) -> f32;
    /// Technical implementation of the ceil logic.
    fn ceil(self) -> f32;
    /// Technical implementation of the round logic.
    fn round(self) -> f32;
}

impl FloatExt for f32 {
    #[inline(always)]
    /// Technical implementation of the tanh logic.
    fn tanh(self) -> f32 {
        tanh_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the tan logic.
    fn tan(self) -> f32 {
        tan_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the sin logic.
    fn sin(self) -> f32 {
        sine_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the cos logic.
    fn cos(self) -> f32 {
        cosine_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the powf logic.
    fn powf(self, n: f32) -> f32 {
        fast_pow(self, n)
    }
    #[inline(always)]
    /// Technical implementation of the powi logic.
    fn powi(self, n: i32) -> f32 {
        self.powi_approx(n)
    }
    #[inline(always)]
    /// Technical implementation of the sqrt logic.
    fn sqrt(self) -> f32 {
        sqrt_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the fract logic.
    fn fract(self) -> f32 {
        self - floor_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the exp logic.
    fn exp(self) -> f32 {
        exp_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the ln logic.
    fn ln(self) -> f32 {
        fast_log2(self) * core::f32::consts::LN_2
    }
    #[inline(always)]
    /// Technical implementation of the log2 logic.
    fn log2(self) -> f32 {
        fast_log2(self)
    }
    #[inline(always)]
    /// Technical implementation of the log10 logic.
    fn log10(self) -> f32 {
        fast_log2(self) * core::f32::consts::LOG10_2
    }
    #[inline(always)]
    /// Technical implementation of the abs logic.
    fn abs(self) -> f32 {
        abs_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the max logic.
    fn max(self, other: f32) -> f32 {
        if self > other {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    /// Technical implementation of the min logic.
    fn min(self, other: f32) -> f32 {
        if self < other {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    /// Technical implementation of the floor logic.
    fn floor(self) -> f32 {
        floor_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the ceil logic.
    fn ceil(self) -> f32 {
        ceil_approx(self)
    }
    #[inline(always)]
    /// Technical implementation of the round logic.
    fn round(self) -> f32 {
        round_approx(self)
    }
}

pub trait FloatExt64 {
    /// Technical implementation of the tanh logic.
    fn tanh(self) -> f64;
    /// Technical implementation of the tan logic.
    fn tan(self) -> f64;
    /// Technical implementation of the sin logic.
    fn sin(self) -> f64;
    /// Technical implementation of the cos logic.
    fn cos(self) -> f64;
    /// Technical implementation of the powf logic.
    fn powf(self, n: f64) -> f64;
    /// Technical implementation of the powi logic.
    fn powi(self, n: i32) -> f64;
    /// Technical implementation of the sqrt logic.
    fn sqrt(self) -> f64;
    /// Technical implementation of the fract logic.
    fn fract(self) -> f64;
    /// Technical implementation of the exp logic.
    fn exp(self) -> f64;
    /// Technical implementation of the abs logic.
    fn abs(self) -> f64;
    /// Technical implementation of the max logic.
    fn max(self, other: f64) -> f64;
    /// Technical implementation of the min logic.
    fn min(self, other: f64) -> f64;
    /// Technical implementation of the floor logic.
    fn floor(self) -> f64;
    /// Technical implementation of the ceil logic.
    fn ceil(self) -> f64;
}

impl FloatExt64 for f64 {
    #[inline(always)]
    /// Technical implementation of the tanh logic.
    fn tanh(self) -> f64 {
        let x = self.clamp(-20.0, 20.0);
        let x2 = x * x;
        let a = x * (135135.0 + x2 * (17325.0 + x2 * (378.0 + x2)));
        let b = 135135.0 + x2 * (62370.0 + x2 * (3150.0 + 28.0 * x2));
        a / b
    }
    #[inline(always)]
    /// Technical implementation of the tan logic.
    fn tan(self) -> f64 {
        self
    }
    #[inline(always)]
    /// Technical implementation of the sin logic.
    fn sin(self) -> f64 {
        // [Engineering Phase 20]: High-precision sine approximation
        let mut x = self % (2.0 * PI_F64);
        if x > PI_F64 {
            x -= 2.0 * PI_F64;
        }
        if x < -PI_F64 {
            x += 2.0 * PI_F64;
        }
        let mut res = 0.0;
        let mut term = x;
        let x2 = x * x;
        for i in 1..10 {
            res += term;
            term *= -x2 / ((2 * i) * (2 * i + 1)) as f64;
        }
        res
    }
    #[inline(always)]
    /// Technical implementation of the cos logic.
    fn cos(self) -> f64 {
        (self + PI_F64 / 2.0).sin()
    }
    #[inline(always)]
    /// Technical implementation of the powf logic.
    fn powf(self, n: f64) -> f64 {
        if self <= 0.0 {
            return 0.0;
        }
        (n * self.ln_approx()).exp_approx()
    }
    #[inline(always)]
    /// Technical implementation of the powi logic.
    fn powi(self, n: i32) -> f64 {
        let mut res = 1.0;
        let mut base = self;
        let mut p = n.abs();
        while p > 0 {
            if p % 2 == 1 {
                res *= base;
            }
            base *= base;
            p /= 2;
        }
        if n < 0 {
            1.0 / res
        } else {
            res
        }
    }
    #[inline(always)]
    /// Technical implementation of the sqrt logic.
    fn sqrt(self) -> f64 {
        if self <= 0.0 {
            return 0.0;
        }
        let mut x = 1.0;
        for _ in 0..6 {
            x = 0.5 * (x + self / x);
        }
        x
    }
    #[inline(always)]
    /// Technical implementation of the fract logic.
    fn fract(self) -> f64 {
        self - self.floor_approx()
    }
    #[inline(always)]
    /// Technical implementation of the exp logic.
    fn exp(self) -> f64 {
        self.exp_approx()
    }
    #[inline(always)]
    /// Technical implementation of the abs logic.
    fn abs(self) -> f64 {
        if self < 0.0 {
            -self
        } else {
            self
        }
    }
    #[inline(always)]
    /// Technical implementation of the max logic.
    fn max(self, other: f64) -> f64 {
        if self > other {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    /// Technical implementation of the min logic.
    fn min(self, other: f64) -> f64 {
        if self < other {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    /// Technical implementation of the floor logic.
    fn floor(self) -> f64 {
        self.floor_approx()
    }
    #[inline(always)]
    /// Technical implementation of the ceil logic.
    fn ceil(self) -> f64 {
        self.ceil_approx()
    }
}

trait InternalMath64 {
    /// Technical implementation of the exp_approx logic.
    fn exp_approx(self) -> f64;
    /// Technical implementation of the ln_approx logic.
    fn ln_approx(self) -> f64;
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f64;
    /// Technical implementation of the ceil_approx logic.
    fn ceil_approx(self) -> f64;
}

impl InternalMath64 for f64 {
    /// Technical implementation of the exp_approx logic.
    fn exp_approx(self) -> f64 {
        let mut res = 1.0;
        let mut term = 1.0;
        for i in 1..15 {
            term *= self / i as f64;
            res += term;
        }
        res
    }
    /// Technical implementation of the ln_approx logic.
    fn ln_approx(self) -> f64 {
        let x = (self - 1.0) / (self + 1.0);
        let x2 = x * x;
        let mut res = 0.0;
        let mut term = x;
        for i in 0..10 {
            res += term / (2 * i + 1) as f64;
            term *= x2;
        }
        2.0 * res
    }
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f64 {
        let i = self as i64;
        if self < i as f64 {
            (i - 1) as f64
        } else {
            i as f64
        }
    }
    /// Technical implementation of the ceil_approx logic.
    fn ceil_approx(self) -> f64 {
        let i = self as i64;
        if self > i as f64 {
            (i + 1) as f64
        } else {
            i as f64
        }
    }
}

/// High-precision phase accumulator for oscillators.
pub struct PhaseAccumulator {
    phase: f32,
    phase_inc: f32,
}

impl PhaseAccumulator {
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        Self { phase: 0.0, phase_inc: freq_hz / sample_rate }
    }

    pub fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        self.phase_inc = freq_hz / sample_rate;
    }

    pub fn next(&mut self) -> f32 {
        let current = self.phase;
        self.phase = (self.phase + self.phase_inc).fract();
        current
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}

///
/// Simple recursive filter for high-end damping and smoothing.
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the OnePoleFilter structure.
pub struct OnePoleFilter {
    state: f32,
    coeff: f32,
}

impl OnePoleFilter {
    /// Initializes a new instance of the associated type.
    pub fn new(coeff: f32) -> Self {
        Self {
            state: 0.0,
            coeff,
        }
    }

    /// Technical implementation of the with_coefficient logic.
    pub fn with_coefficient(coeff: f32) -> Self {
        Self { state: 0.0, coeff }
    }

    /// Technical implementation of the set_coefficient logic.
    pub fn set_coefficient(&mut self, coeff: f32) {
        self.coeff = coeff;
    }

    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        self.state = input * (1.0 - self.coeff) + self.state * self.coeff;
        self.state
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.state = 0.0;
    }
}

#[inline(always)]
/// Technical implementation of the abs_approx logic.
pub fn abs_approx(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFFFFFF)
}

#[inline(always)]
/// Technical implementation of the tanh_approx logic.
pub fn tanh_approx(x: f32) -> f32 {
    let x2 = x * x;
    let a = x * (135135.0 + x2 * (17325.0 + x2 * (378.0 + x2)));
    let b = 135135.0 + x2 * (62370.0 + x2 * (3150.0 + 28.0 * x2));
    a / b
}

#[inline(always)]
/// Technical implementation of the tan_approx logic.
pub fn tan_approx(x: f32) -> f32 {
    let s = sine_approx(x);
    let c = cosine_approx(x);
    if c.abs() < 1e-6 {
        s.signum() * 1e6
    } else {
        s / c
    }
}

#[inline(always)]
/// Technical implementation of the sine_approx logic.
pub fn sine_approx(mut x: f32) -> f32 {
    // Wrap x to [-PI, PI]
    x *= 1.0 / TAU;
    x = x - floor_approx(x + 0.5);
    x *= TAU;

    let mut y = (4.0 / PI) * x - (4.0 / (PI * PI)) * x * x.abs();

    // Extra precision refinement
    y = 0.225 * (y * y.abs() - y) + y;
    y
}

#[inline(always)]
/// Technical implementation of the cosine_approx logic.
pub fn cosine_approx(x: f32) -> f32 {
    sine_approx(x + (PI * 0.5))
}

#[inline(always)]
/// Technical implementation of the exp_approx logic.
pub fn exp_approx(x: f32) -> f32 {
    let mut res = 1.0;
    let mut term = 1.0;
    for i in 1..10 {
        term *= x / i as f32;
        res += term;
    }
    res
}

#[inline(always)]
/// Technical implementation of the floor_approx logic.
pub fn floor_approx(x: f32) -> f32 {
    let i = x as i32;
    if x < i as f32 {
        (i - 1) as f32
    } else {
        i as f32
    }
}

#[inline(always)]
/// Technical implementation of the ceil_approx logic.
pub fn ceil_approx(x: f32) -> f32 {
    let i = x as i32;
    if x > i as f32 {
        (i + 1) as f32
    } else {
        i as f32
    }
}

#[inline(always)]
/// Technical implementation of the round_approx logic.
pub fn round_approx(x: f32) -> f32 {
    floor_approx(x + 0.5)
}

#[inline(always)]
/// Technical implementation of the fast_pow logic.
pub fn fast_pow(x: f32, y: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    exp_approx(y * fast_log2(x) * core::f32::consts::LN_2)
}

#[inline(always)]
/// Technical implementation of the fast_log2 logic.
pub fn fast_log2(x: f32) -> f32 {
    let bits = x.to_bits();
    let exponent = ((bits >> 23) as i32) - 127;
    let mantissa = f32::from_bits((bits & 0x007FFFFF) | 0x3F800000);
    exponent as f32 + (mantissa - 1.0) * (1.19666 * mantissa + 0.40901) / (mantissa + 0.94247)
}

#[inline(always)]
/// Technical implementation of the sqrt_approx logic.
pub fn sqrt_approx(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let mut y = f32::from_bits(i);
    y = y * (1.5 - 0.5 * x * y * y);
    1.0 / y
}

pub trait PowiApprox {
    /// Technical implementation of the powi_approx logic.
    fn powi_approx(self, n: i32) -> f32;
}

impl PowiApprox for f32 {
    /// Technical implementation of the powi_approx logic.
    fn powi_approx(self, mut n: i32) -> f32 {
        let mut res = 1.0;
        let mut base = self;
        if n < 0 {
            base = 1.0 / base;
            n = -n;
        }
        while n > 0 {
            if n % 2 == 1 {
                res *= base;
            }
            base *= base;
            n /= 2;
        }
        res
    }
}

pub const PHI: f32 = 1.618_034;

/// Technical implementation of the amplitude_to_db logic.
pub fn amplitude_to_db(amp: f32) -> f32 {
    20.0 * fast_log2(amp.abs() + 1e-9) * core::f32::consts::LOG10_2
}

/// Technical implementation of the db_to_amplitude logic.
pub fn db_to_amplitude(db: f32) -> f32 {
    fast_pow(10.0, db / 20.0)
}

/// Technical implementation of the hermite_interpolate logic.
pub fn hermite_interpolate(y0: f32, y1: f32, y2: f32, y3: f32, mu: f32) -> f32 {
    let mu2 = mu * mu;
    let a0 = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
    let a1 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let a2 = -0.5 * y0 + 0.5 * y2;
    let a3 = y1;
    a0 * mu * mu2 + a1 * mu2 + a2 * mu + a3
}
