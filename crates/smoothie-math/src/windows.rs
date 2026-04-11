//! Window functions for FFT and spectral analysis.

use std::f32::consts::PI;

/// Fill `buf` with a Hann window.
pub fn hann(buf: &mut [f32]) {
    let n = buf.len();
    for (i, v) in buf.iter_mut().enumerate() {
        *v = 0.5 * (1.0 - (2.0 * PI * i as f32 / (n - 1) as f32).cos());
    }
}

/// Fill `buf` with a Hamming window.
pub fn hamming(buf: &mut [f32]) {
    let n = buf.len();
    for (i, v) in buf.iter_mut().enumerate() {
        *v = 0.54 - 0.46 * (2.0 * PI * i as f32 / (n - 1) as f32).cos();
    }
}

/// Fill `buf` with a Blackman window.
pub fn blackman(buf: &mut [f32]) {
    let n = buf.len();
    for (i, v) in buf.iter_mut().enumerate() {
        let x = 2.0 * PI * i as f32 / (n - 1) as f32;
        *v = 0.42 - 0.5 * x.cos() + 0.08 * (2.0 * x).cos();
    }
}

/// Blackman-Harris window (4-term, lowest sidelobes).
pub fn blackman_harris(buf: &mut [f32]) {
    let n = buf.len();
    let a = [0.35875_f32, 0.48829, 0.14128, 0.01168];
    for (i, v) in buf.iter_mut().enumerate() {
        let x = 2.0 * PI * i as f32 / (n - 1) as f32;
        *v = a[0] - a[1]*x.cos() + a[2]*(2.0*x).cos() - a[3]*(3.0*x).cos();
    }
}

/// Flat-top window (accurate amplitude measurement).
pub fn flat_top(buf: &mut [f32]) {
    let n = buf.len();
    let a = [0.21557895_f32, 0.41663158, 0.27726316, 0.08357895, 0.00694737];
    for (i, v) in buf.iter_mut().enumerate() {
        let x = 2.0 * PI * i as f32 / (n - 1) as f32;
        *v = a[0] - a[1]*x.cos() + a[2]*(2.0*x).cos()
           - a[3]*(3.0*x).cos() + a[4]*(4.0*x).cos();
    }
}

/// Kaiser window with shape parameter `beta`.
pub fn kaiser(buf: &mut [f32], beta: f32) {
    let n  = buf.len();
    let d  = modified_bessel_i0(beta);
    let n1 = (n - 1) as f32;
    for (i, v) in buf.iter_mut().enumerate() {
        let x = 2.0 * i as f32 / n1 - 1.0;
        *v = modified_bessel_i0(beta * (1.0 - x * x).max(0.0).sqrt()) / d;
    }
}

/// Rectangular (no windowing).
pub fn rect(buf: &mut [f32]) {
    buf.fill(1.0);
}

/// Triangular (Bartlett) window.
pub fn triangular(buf: &mut [f32]) {
    let n = buf.len();
    let half = (n - 1) as f32 / 2.0;
    for (i, v) in buf.iter_mut().enumerate() {
        *v = 1.0 - ((i as f32 - half) / half).abs();
    }
}

/// Apply a precomputed window to a buffer (in-place, element-wise multiply).
pub fn apply(buf: &mut [f32], window: &[f32]) {
    for (s, w) in buf.iter_mut().zip(window.iter()) {
        *s *= w;
    }
}

/// Modified Bessel function I₀ used in Kaiser window.
fn modified_bessel_i0(x: f32) -> f32 {
    let mut sum = 1.0f32;
    let mut term = 1.0f32;
    let xh = x * 0.5;
    for k in 1..=25u32 {
        term *= xh / k as f32;
        let t2 = term * term;
        sum += t2;
        if t2 < 1e-12 { break; }
    }
    sum
}
