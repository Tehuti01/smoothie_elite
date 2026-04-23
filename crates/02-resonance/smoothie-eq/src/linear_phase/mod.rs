/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x565e561c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/linear_phase/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// this module provides a linear-phase FIR EQ computed via the frequency-
/// N/2 samples of latency that must be reported to the host.
/// # Design Method
/// 1. Compute the desired frequency response H(ω) on a dense grid (N bins).
/// 3. Apply a symmetric window (Blackman-Harris) to reduce Gibbs phenomenon.
///
///
/// this is ~11.6 ms — acceptable for offline/side-chain but must be
/// declared to the host via the `clap.latency` extension.
use alloc::vec::Vec;
use smoothie_core::math::sine_approx;

/// A linear-phase FIR EQ band specification.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the LinearPhasePoint structure.
pub struct LinearPhasePoint {
    pub freq_hz: f32,
    pub gain_db: f32,
}

/// Technical implementation of the LinearPhaseEq structure.
pub struct LinearPhaseEq {
    kernel: Vec<f32>,
    /// Overlap-save history buffer (size = kernel_size - 1).
    history_l: Vec<f32>,
    history_r: Vec<f32>,
    kernel_size: usize,
    sample_rate: f32,
}

impl LinearPhaseEq {
    /// Construct a linear-phase EQ with `kernel_size` taps.
    /// `kernel_size` must be odd for a strictly symmetric response.
    pub fn new(kernel_size: usize, sample_rate: f32) -> Self {
        debug_assert!(kernel_size % 2 == 1, "kernel_size must be odd");
        let history_len = kernel_size - 1;
        Self {
            kernel: vec![0.0; kernel_size],
            history_l: vec![0.0; history_len],
            history_r: vec![0.0; history_len],
            kernel_size,
            sample_rate,
        }
    }

    /// Rebuild the FIR kernel from a list of `(freq_hz, gain_db)` control points.
    ///
    /// Uses frequency-sampling design with Blackman-Harris windowing.
    pub fn set_response(&mut self, points: &[LinearPhasePoint]) {
        let n = self.kernel_size;
        let half = n / 2;
        let n_float = n as f32;
        let sr = self.sample_rate;

        // Build magnitude response on dense normalised-frequency grid
        let mut h = vec![0.0f32; n];

        // Linear interpolation of control points onto [0..n] grid
        let mut sorted: Vec<LinearPhasePoint> = Vec::with_capacity(points.len() + 2);
        sorted.push(LinearPhasePoint {
            freq_hz: 0.0,
            gain_db: 0.0,
        });
        for &p in points {
            sorted.push(p);
        }
        sorted.push(LinearPhasePoint {
            freq_hz: sr * 0.5,
            gain_db: 0.0,
        });

        for k in 0..=half {
            let freq = k as f32 * sr / n_float;
            // Find bracket in sorted control points
            let gain_db = interp_gain(&sorted, freq);
            // Convert dB → linear
            let gain_lin = db_to_linear_f32(gain_db);
            h[k] = gain_lin;
            if k > 0 && k < half {
                h[n - k] = gain_lin;
            } // conjugate symmetry
        }

        // IFFT (using zero-allocation DFT for the kernel — computed once)
        let mut kernel = vec![0.0f32; n];
        for i in 0..n {
            let mut val = 0.0f32;
            for k in 0..n {
                let phase = (k * i) as f32 / n_float;
                val += h[k] * sine_approx((phase + 0.25) % 1.0);
            }
            kernel[i] = val / n_float;
        }

        // Shift to causal form and apply Blackman-Harris window
        let mut windowed = vec![0.0f32; n];
        for i in 0..n {
            let causal_idx = (i + half) % n;
            let x = i as f32 / (n - 1) as f32;
            let w = blackman_harris_coeff(x);
            windowed[i] = kernel[causal_idx] * w;
        }

        self.kernel = windowed;
        // Reset history on kernel change
        for v in self.history_l.iter_mut() {
            *v = 0.0;
        }
        for v in self.history_r.iter_mut() {
            *v = 0.0;
        }
    }

    /// Reported latency in samples.
    pub fn latency_samples(&self) -> u32 {
        (self.kernel_size / 2) as u32
    }

    /// Process a stereo block via direct convolution (O(N·M) per block).
    /// For production use, replace with FFT-based overlap-save (O(N log N)).
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        let kn = self.kernel_size;
        let hn = kn - 1;

        for (xl, xr) in left.iter_mut().zip(right.iter_mut()) {
            // Shift history and insert new sample
            for i in (1..hn).rev() {
                self.history_l[i] = self.history_l[i - 1];
                self.history_r[i] = self.history_r[i - 1];
            }
            self.history_l[0] = *xl;
            self.history_r[0] = *xr;

            // Convolve with kernel
            let mut out_l = self.kernel[0] * *xl;
            let mut out_r = self.kernel[0] * *xr;
            for k in 1..kn.min(hn + 1) {
                let h_idx = k - 1;
                if h_idx < hn {
                    out_l += self.kernel[k] * self.history_l[h_idx];
                    out_r += self.kernel[k] * self.history_r[h_idx];
                }
            }
            *xl = out_l;
            *xr = out_r;
        }
    }
}

/// Technical implementation of the blackman_harris_coeff logic.
fn blackman_harris_coeff(x: f32) -> f32 {
    let c1 = sine_approx((x + 0.25) % 1.0);
    let c2 = sine_approx((2.0 * x + 0.25) % 1.0);
    let c3 = sine_approx((3.0 * x + 0.25) % 1.0);
    0.35875 - 0.48829 * c1 + 0.14129 * c2 - 0.01168 * c3
}

/// Technical implementation of the interp_gain logic.
fn interp_gain(points: &[LinearPhasePoint], freq: f32) -> f32 {
    for i in 0..points.len().saturating_sub(1) {
        let a = points[i];
        let b = points[i + 1];
        if freq >= a.freq_hz && freq <= b.freq_hz {
            let t = (freq - a.freq_hz) / (b.freq_hz - a.freq_hz).max(1e-6);
            return a.gain_db + t * (b.gain_db - a.gain_db);
        }
    }
    0.0
}

/// Technical implementation of the db_to_linear_f32 logic.
fn db_to_linear_f32(db: f32) -> f32 {
    smoothie_core::math::exp_approx(db * 0.115_129_255_f32)
}
