//! Custom filter implementations for effects processing.

use std::f32::consts::PI;

/// Filter type for biquad coefficient design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    /// Low-pass filter.
    LowPass,
    /// High-pass filter.
    HighPass,
    /// Band-pass filter.
    BandPass,
    /// Low-shelf filter (boost/cut at low frequencies).
    LowShelf,
    /// High-shelf filter (boost/cut at high frequencies).
    HighShelf,
    /// Peaking EQ filter.
    PeakingEq,
    /// Notch filter.
    Notch,
}

/// Second-order IIR biquad filter with coefficients.
#[derive(Debug, Clone)]
pub struct BiquadFilter {
    // Filter coefficients
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    // State variables (direct form II)
    z1: f32,
    z2: f32,
}

impl BiquadFilter {
    /// Design a biquad filter with the given parameters.
    pub fn design(
        filter_type: FilterType,
        frequency: f32,
        sample_rate: f32,
        q: f32,
        gain_db: f32,
    ) -> Self {
        let w0 = 2.0 * PI * frequency / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match filter_type {
            FilterType::LowPass => {
                let b0 = (1.0 - cos_w0) / 2.0;
                let b1 = 1.0 - cos_w0;
                let b2 = (1.0 - cos_w0) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::HighPass => {
                let b0 = (1.0 + cos_w0) / 2.0;
                let b1 = -(1.0 + cos_w0);
                let b2 = (1.0 + cos_w0) / 2.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::BandPass => {
                let b0 = alpha;
                let b1 = 0.0;
                let b2 = -alpha;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::LowShelf => {
                let s = 2.0 * (a).sqrt() * alpha;
                let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + s);
                let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
                let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - s);
                let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + s;
                let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
                let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - s;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::HighShelf => {
                let s = 2.0 * (a).sqrt() * alpha;
                let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + s);
                let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
                let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - s);
                let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + s;
                let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
                let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - s;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::PeakingEq => {
                let b0 = 1.0 + alpha * a;
                let b1 = -2.0 * cos_w0;
                let b2 = 1.0 - alpha * a;
                let a0 = 1.0 + alpha / a;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha / a;
                (b0, b1, b2, a0, a1, a2)
            }
            FilterType::Notch => {
                let b0 = 1.0;
                let b1 = -2.0 * cos_w0;
                let b2 = 1.0;
                let a0 = 1.0 + alpha;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha;
                (b0, b1, b2, a0, a1, a2)
            }
        };

        // Normalize by a0
        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    /// Process a single sample through the filter.
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }

    /// Reset filter state.
    pub fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowpass_filter() {
        let mut filter = BiquadFilter::design(FilterType::LowPass, 1000.0, 44100.0, 0.707, 0.0);
        let _ = filter.process(1.0);
        assert!(!filter.process(0.0).is_nan());
    }

    #[test]
    fn test_shelf_filter() {
        let mut filter = BiquadFilter::design(FilterType::LowShelf, 200.0, 44100.0, 0.707, 6.0);
        let _ = filter.process(1.0);
        assert!(!filter.process(0.0).is_nan());
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
