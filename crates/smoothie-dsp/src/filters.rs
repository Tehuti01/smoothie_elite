//! Biquad and one-pole IIR filters.
//!
//! All filters are direct-form II transposed — numerically stable,
//! zero-allocation, suitable for the audio thread.

use std::f32::consts::PI;

/// Filter topology type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    AllPass,
    PeakingEq,
    LowShelf,
    HighShelf,
}

/// Second-order (biquad) IIR filter — direct-form II transposed.
///
/// Covers every EQ and crossover shape needed in audio production.
#[derive(Debug, Clone)]
pub struct BiquadFilter {
    // Coefficients
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    // State (two delay elements)
    s1: f32, s2: f32,
}

impl BiquadFilter {
    /// Identity (pass-through) filter.
    pub fn identity() -> Self {
        Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0, s1: 0.0, s2: 0.0 }
    }

    /// Design a second-order filter.
    /// - `freq_hz` : cutoff / centre frequency
    /// - `q`       : quality factor (0.707 = Butterworth)
    /// - `gain_db` : shelf / peak gain (unused for LP/HP/BP/Notch)
    pub fn design(kind: FilterType, freq_hz: f32, sample_rate: f32, q: f32, gain_db: f32) -> Self {
        let w0    = 2.0 * PI * freq_hz / sample_rate;
        let cos_w = w0.cos();
        let sin_w = w0.sin();
        let alpha = sin_w / (2.0 * q);
        let a     = 10.0_f32.powf(gain_db / 40.0); // linear amplitude for shelf/peak

        let (b0, b1, b2, a0, a1, a2) = match kind {
            FilterType::LowPass => (
                (1.0 - cos_w) / 2.0,
                1.0 - cos_w,
                (1.0 - cos_w) / 2.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha,
            ),
            FilterType::HighPass => (
                (1.0 + cos_w) / 2.0,
                -(1.0 + cos_w),
                (1.0 + cos_w) / 2.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha,
            ),
            FilterType::BandPass => (
                sin_w / 2.0, 0.0, -sin_w / 2.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha,
            ),
            FilterType::Notch => (
                1.0, -2.0 * cos_w, 1.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha,
            ),
            FilterType::AllPass => (
                1.0 - alpha, -2.0 * cos_w, 1.0 + alpha,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha,
            ),
            FilterType::PeakingEq => (
                1.0 + alpha * a, -2.0 * cos_w, 1.0 - alpha * a,
                1.0 + alpha / a, -2.0 * cos_w, 1.0 - alpha / a,
            ),
            FilterType::LowShelf => {
                let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;
                (
                    a * ((a + 1.0) - (a - 1.0) * cos_w + two_sqrt_a_alpha),
                    2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w),
                    a * ((a + 1.0) - (a - 1.0) * cos_w - two_sqrt_a_alpha),
                    (a + 1.0) + (a - 1.0) * cos_w + two_sqrt_a_alpha,
                    -2.0 * ((a - 1.0) + (a + 1.0) * cos_w),
                    (a + 1.0) + (a - 1.0) * cos_w - two_sqrt_a_alpha,
                )
            }
            FilterType::HighShelf => {
                let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;
                (
                    a * ((a + 1.0) + (a - 1.0) * cos_w + two_sqrt_a_alpha),
                    -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w),
                    a * ((a + 1.0) + (a - 1.0) * cos_w - two_sqrt_a_alpha),
                    (a + 1.0) - (a - 1.0) * cos_w + two_sqrt_a_alpha,
                    2.0 * ((a - 1.0) - (a + 1.0) * cos_w),
                    (a + 1.0) - (a - 1.0) * cos_w - two_sqrt_a_alpha,
                )
            }
        };

        // Normalise by a0
        Self {
            b0: b0 / a0, b1: b1 / a0, b2: b2 / a0,
            a1: a1 / a0, a2: a2 / a0,
            s1: 0.0, s2: 0.0,
        }
    }

    /// Process one sample. Call once per sample on the audio thread.
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.s1;
        self.s1 = self.b1 * x - self.a1 * y + self.s2;
        self.s2 = self.b2 * x - self.a2 * y;
        y
    }

    /// Process a buffer in place.
    #[inline]
    pub fn process_block(&mut self, buf: &mut [f32]) {
        for s in buf.iter_mut() { *s = self.process(*s); }
    }

    /// Reset internal state (call on transport restart).
    #[inline]
    pub fn reset(&mut self) { self.s1 = 0.0; self.s2 = 0.0; }
}

/// Simple one-pole IIR — the workhorse for parameter smoothing and DC blocking.
pub struct OnePoleFilter {
    coeff: f32,
    state: f32,
}

impl OnePoleFilter {
    /// `coeff` in (0, 1). Higher = slower. Use `1 - exp(-2π * fc/fs)`.
    pub fn new(coeff: f32) -> Self { Self { coeff, state: 0.0 } }

    pub fn from_cutoff(cutoff_hz: f32, sample_rate: f32) -> Self {
        let c = 1.0 - (-2.0 * PI * cutoff_hz / sample_rate).exp();
        Self::new(c)
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.state += self.coeff * (x - self.state);
        self.state
    }

    #[inline]
    pub fn reset(&mut self) { self.state = 0.0; }
}
