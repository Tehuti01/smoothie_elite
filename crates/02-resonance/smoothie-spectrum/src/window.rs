/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd4832e51 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/window.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::math::sine_approx;

/// Tag enum selecting the windowing algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the WindowFunction enumeration.
pub enum WindowFunction {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Nuttall,
    Gaussian { sigma_tenths: u32 },
}

/// Technical implementation of the Window structure.
pub struct Window {
    pub function: WindowFunction,
    pub coefficients: Vec<f32>,
    pub power_correction: f32,
    pub amplitude_correction: f32,
}

impl Window {
    /// Initializes a new instance of the associated type.
    pub fn new(function: WindowFunction, n: usize) -> Self {
        let mut coefficients = Vec::with_capacity(n);
        let mut sum = 0.0_f32;
        let mut sum_sq = 0.0_f32;

        for i in 0..n {
            let w = compute_coefficient(function, i, n);
            coefficients.push(w);
            sum += w;
            sum_sq += w * w;
        }

        let amplitude_correction = n as f32 / sum;
        // Fast sqrt for power correction
        let power_correction = fast_sqrt(n as f32 / sum_sq);

        Self {
            function,
            coefficients,
            power_correction,
            amplitude_correction,
        }
    }

    #[inline]
    /// Technical implementation of the apply logic.
    pub fn apply(&self, buffer: &mut [f32]) {
        debug_assert_eq!(buffer.len(), self.coefficients.len());
        for (sample, &coeff) in buffer.iter_mut().zip(self.coefficients.iter()) {
            *sample *= coeff;
        }
    }

    /// Technical implementation of the apply_into logic.
    pub fn apply_into(&self, src: &[f32], dst: &mut [f32]) {
        for ((s, d), &c) in src.iter().zip(dst.iter_mut()).zip(self.coefficients.iter()) {
            *d = *s * c;
        }
    }

    /// Technical implementation of the n logic.
    pub fn n(&self) -> usize {
        self.coefficients.len()
    }
}

/// Technical implementation of the compute_coefficient logic.
fn compute_coefficient(function: WindowFunction, i: usize, n: usize) -> f32 {
    let x = i as f32 / (n - 1) as f32;

    match function {
        WindowFunction::Rectangular => 1.0,

        WindowFunction::Hann => {
            let cos_val = sine_approx(frac(x + 0.25));
            0.5 - 0.5 * cos_val
        }

        WindowFunction::Hamming => {
            let cos_val = sine_approx(frac(x + 0.25));
            0.54 - 0.46 * cos_val
        }

        WindowFunction::Blackman => {
            let cos1 = sine_approx(frac(x + 0.25));
            let cos2 = sine_approx(frac(2.0 * x + 0.25));
            0.42 - 0.5 * cos1 + 0.08 * cos2
        }

        WindowFunction::BlackmanHarris => {
            let cos1 = sine_approx(frac(x + 0.25));
            let cos2 = sine_approx(frac(2.0 * x + 0.25));
            let cos3 = sine_approx(frac(3.0 * x + 0.25));
            0.35875 - 0.48829 * cos1 + 0.14128 * cos2 - 0.01168 * cos3
        }

        WindowFunction::FlatTop => {
            let cos1 = sine_approx(frac(x + 0.25));
            let cos2 = sine_approx(frac(2.0 * x + 0.25));
            let cos3 = sine_approx(frac(3.0 * x + 0.25));
            let cos4 = sine_approx(frac(4.0 * x + 0.25));
            0.21557895 - 0.41663158 * cos1 + 0.27726316 * cos2 - 0.08357895 * cos3
                + 0.00694737 * cos4
        }

        WindowFunction::Nuttall => {
            let cos1 = sine_approx(frac(x + 0.25));
            let cos2 = sine_approx(frac(2.0 * x + 0.25));
            let cos3 = sine_approx(frac(3.0 * x + 0.25));
            0.355768 - 0.487396 * cos1 + 0.144232 * cos2 - 0.012604 * cos3
        }

        WindowFunction::Gaussian { sigma_tenths } => {
            let sigma = sigma_tenths as f32 / 10.0;
            let centered = x - 0.5;
            let exponent = -0.5 * (centered / sigma) * (centered / sigma);
            smoothie_core::math::exp_approx(exponent)
        }
    }
}

/// `rem_euclid` replacement for no_std f32 — ensures value is in [0, 1).
#[inline(always)]
/// Technical implementation of the frac logic.
fn frac(x: f32) -> f32 {
    let f = x - x.floor_approx();
    if f < 0.0 {
        f + 1.0
    } else {
        f
    }
}

trait FloorApprox {
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f32;
}
impl FloorApprox for f32 {
    #[inline(always)]
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f32 {
        let i = self as i32;
        if (i as f32) > self {
            i as f32 - 1.0
        } else {
            i as f32
        }
    }
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    x * (y * (1.5 - xhalf * y * y))
}
