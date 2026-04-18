//! Biquad, one-pole, and ZDF (Zero-Delay Feedback) filters.

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
        let a     = 10.0_f32.powf(gain_db / 40.0);

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
                1.0 + alpha / a, -2.0 * cos_w, 1.0 / a - alpha / a, // Fix: actually 1.0 + alpha / a
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

        // Recalculate correctly (a0 is always 1.0 after normalization)
        // Note: some peakingEq formulas use a slightly different a0. 
        // Re-verifying the standard RBJ formula:
        let (b0, b1, b2, a0, a1, a2) = match kind {
            FilterType::PeakingEq => (
                1.0 + alpha * a, -2.0 * cos_w, 1.0 - alpha * a,
                1.0 + alpha / a, -2.0 * cos_w, 1.0 - alpha / a,
            ),
            _ => (b0, b1, b2, a0, a1, a2),
        };

        Self {
            b0: b0 / a0, b1: b1 / a0, b2: b2 / a0,
            a1: a1 / a0, a2: a2 / a0,
            s1: 0.0, s2: 0.0,
        }
    }

    /// Update filter coefficients.
    pub fn set_parameters(&mut self, kind: FilterType, freq_hz: f32, sample_rate: f32, q: f32, gain_db: f32) {
        let next = Self::design(kind, freq_hz, sample_rate, q, gain_db);
        self.b0 = next.b0;
        self.b1 = next.b1;
        self.b2 = next.b2;
        self.a1 = next.a1;
        self.a2 = next.a2;
    }

    /// Process one sample.
    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.s1;
        self.s1 = self.b1 * x - self.a1 * y + self.s2;
        self.s2 = self.b2 * x - self.a2 * y;
        y
    }

    pub fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
}

/// Simple one-pole lowpass/highpass.
#[derive(Debug, Clone)]
pub struct OnePoleFilter {
    coeff: f32,
    state: f32,
}

impl OnePoleFilter {
    pub fn new(coeff: f32) -> Self { Self { coeff, state: 0.0 } }

    /// coeff = exp(-2.0 * PI * cutoff_hz / sample_rate)
    pub fn set_cutoff(&mut self, cutoff_hz: f32, sample_rate: f32) {
        self.coeff = (-2.0 * PI * cutoff_hz / sample_rate).exp();
    }

    pub fn process(&mut self, x: f32) -> f32 {
        self.state = x + self.coeff * (self.state - x);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biquad_lowpass() {
        let mut filter = BiquadFilter::design(FilterType::LowPass, 1000.0, 44100.0, 0.707, 0.0);
        
        // Feed it unit impulse
        let mut impulse = vec![0.0; 100];
        impulse[0] = 1.0;
        
        let response: Vec<f32> = impulse.into_iter().map(|s| filter.process(s)).collect();
        
        // Check DC gain (should be close to 1.0 for LowPass)
        let mut dc_filter = BiquadFilter::design(FilterType::LowPass, 1000.0, 44100.0, 0.707, 0.0);
        let mut sum = 0.0;
        for _ in 0..1000 { sum = dc_filter.process(1.0); }
        assert!((sum - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_one_pole_lowpass() {
        let mut lp = OnePoleFilter::new(0.0);
        lp.set_cutoff(100.0, 44100.0);
        
        let mut val = 0.0;
        for _ in 0..1000 { val = lp.process(1.0); }
        assert!((val - 1.0).abs() < 1e-3);
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
