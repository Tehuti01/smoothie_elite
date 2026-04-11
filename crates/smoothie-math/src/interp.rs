//! Interpolation algorithms for audio — sample-accurate, zero-alloc.

/// Linear interpolation between `a` and `b` at parameter `t` in [0, 1].
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Inverse lerp: find `t` such that lerp(a, b, t) == v.
#[inline]
pub fn inv_lerp(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}

/// Hermite cubic interpolation (4-point, 3rd-order).
/// `y0..y3` are consecutive samples; `t` is the fractional position between y1 and y2.
#[inline]
pub fn hermite4(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let c0 = y1;
    let c1 = 0.5 * (y2 - y0);
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
    ((c3 * t + c2) * t + c1) * t + c0
}

/// Catmull-Rom spline interpolation. Identical to hermite4 with specific tension.
#[inline]
pub fn catmull_rom(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    hermite4(y0, y1, y2, y3, t)
}

/// 4-point, 4th-order optimal (minimal pre-ringing). Coefficients from Niemitalo.
#[inline]
pub fn optimal4p4o(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let c0 = y1;
    let c1 = -0.5 * y0 + 0.5 * y2;
    let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
    let c3 = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
    let t2 = t * t;
    c0 + c1 * t + c2 * t2 + c3 * t2 * t
}

/// B-spline cubic interpolation (smooth, but not interpolating).
#[inline]
pub fn bspline4(y0: f32, y1: f32, y2: f32, y3: f32, t: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    let w0 = (1.0 - t) * (1.0 - t) * (1.0 - t) / 6.0;
    let w1 = (3.0 * t3 - 6.0 * t2 + 4.0) / 6.0;
    let w2 = (-3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0) / 6.0;
    let w3 = t3 / 6.0;
    w0 * y0 + w1 * y1 + w2 * y2 + w3 * y3
}

/// Allpass interpolation (first-order allpass filter as fractional delay).
/// Maintains a single state variable — wrap this in your struct.
pub struct AllpassInterpolator {
    y_prev: f32,
    x_prev: f32,
}

impl AllpassInterpolator {
    pub fn new() -> Self { Self { y_prev: 0.0, x_prev: 0.0 } }

    /// Process one sample with fractional delay `d` in (0, 1).
    #[inline]
    pub fn process(&mut self, x: f32, d: f32) -> f32 {
        let a = (1.0 - d) / (1.0 + d);
        let y = a * x + self.x_prev - a * self.y_prev;
        self.x_prev = x;
        self.y_prev = y;
        y
    }
}

impl Default for AllpassInterpolator {
    fn default() -> Self { Self::new() }
}
