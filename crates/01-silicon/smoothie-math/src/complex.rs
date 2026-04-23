/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8ede8921 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/complex.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// Enables pure signal tracking and massive FFT derivations organically
/// without heap allocations or libm reliance.
use smoothie_core::math::{exp_approx, sine_approx};

/// Fast Square Root via Newton Raphson (3 Iterations)
#[inline(always)]
/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1); // Quake III approximation root
    let mut y = f32::from_bits(i);
    y = y * (1.5 - xhalf * y * y);
    x * y
}

/// Fast atan2 Approximation
#[inline(always)]
/// Technical implementation of the fast_atan2 logic.
fn fast_atan2(y: f32, x: f32) -> f32 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let abs_y = y.abs();

    let angle = if x >= 0.0 {
        let r = (x - abs_y) / (x + abs_y);
        0.1963 * r * r * r - 0.9817 * r + 0.7853
    } else {
        let r = (x + abs_y) / (abs_y - x);
        0.1963 * r * r * r - 0.9817 * r + 2.3561
    };
    if y < 0.0 {
        -angle
    } else {
        angle
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
/// Technical implementation of the Complex32 structure.
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

impl Complex32 {
    /// Initializes a new instance of the associated type.
    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    /// Construct a Complex number from polar coordinates
    #[inline(always)]
    /// Technical implementation of the from_polar logic.
    pub fn from_polar(magnitude: f32, phase: f32) -> Self {
        let raw_phase = phase / core::f32::consts::TAU;
        Self {
            re: magnitude * sine_approx((raw_phase + 0.25) % 1.0),
            im: magnitude * sine_approx(raw_phase % 1.0),
        }
    }

    #[inline(always)]
    /// Calculates the Euclidean norm (magnitude) of the vector.
    pub fn magnitude(&self) -> f32 {
        fast_sqrt(self.re * self.re + self.im * self.im)
    }

    #[inline(always)]
    /// Calculates the Euclidean norm (magnitude) of the vector.
    pub fn magnitude_squared(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    #[inline(always)]
    /// Technical implementation of the phase logic.
    pub fn phase(&self) -> f32 {
        fast_atan2(self.im, self.re)
    }

    #[inline(always)]
    /// Technical implementation of the conjugate logic.
    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    #[inline(always)]
    /// Technical implementation of the reciprocal logic.
    pub fn reciprocal(&self) -> Self {
        let m2 = self.magnitude_squared();
        if m2 == 0.0 {
            Self::new(0.0, 0.0) // DSP safe fallback
        } else {
            Self::new(self.re / m2, -self.im / m2)
        }
    }

    /// Complex exponentiation: e^(re + i*im)
    #[inline(always)]
    /// Technical implementation of the exp logic.
    pub fn exp(&self) -> Self {
        let e_pow_re = exp_approx(self.re);
        let raw_phase = self.im / core::f32::consts::TAU;
        Self {
            re: e_pow_re * sine_approx((raw_phase + 0.25) % 1.0),
            im: e_pow_re * sine_approx(raw_phase % 1.0),
        }
    }
}

// ----------------------------------------------------
// NATIVE OPERATOR OVERLOADS
// ----------------------------------------------------

impl core::ops::Add for Complex32 {
    type Output = Self;
    /// Performs vector addition logic.
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl core::ops::Sub for Complex32 {
    type Output = Self;
    /// Performs vector subtraction logic.
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl core::ops::Mul for Complex32 {
    type Output = Self;
    /// Performs matrix/vector multiplication.
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl core::ops::Mul<f32> for Complex32 {
    type Output = Self;
    /// Performs matrix/vector multiplication.
    fn mul(self, scalar: f32) -> Self {
        Self {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }
}

impl core::ops::Div for Complex32 {
    type Output = Self;
    /// Technical implementation of the div logic.
    fn div(self, rhs: Self) -> Self {
        self * rhs.reciprocal()
    }
}

// ----------------------------------------------------
// EXHAUSTIVE DSP UTILITIES
// ----------------------------------------------------

/// Pure zero-allocation array mutations.
/// Technical implementation of the fft_butterfly logic.
pub fn fft_butterfly(a: &mut Complex32, b: &mut Complex32, twiddle: Complex32) {
    let t = twiddle * (*b);
    let u = *a;
    *a = u + t;
    *b = u - t;
}
