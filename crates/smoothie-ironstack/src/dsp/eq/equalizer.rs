use std::f32::consts::PI;
use serde::{Serialize, Deserialize};

/// Supported filter shapes for the equalizer bands.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    LowShelf,
    HighShelf,
    Peaking,
    Notch,
}

/// A standard stereo-capable biquad filter section.
#[derive(Clone)]
pub struct BiquadFilter {
    // --- Coefficients ---
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    
    // --- Stereo State ---
    x1: [f32; 2], x2: [f32; 2],
    y1: [f32; 2], y2: [f32; 2],
}

impl BiquadFilter {
    /// Creates a new BiquadFilter initialized to pass-through.
    pub fn new() -> Self {
        Self {
            b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0,
            x1: [0.0; 2], x2: [0.0; 2], y1: [0.0; 2], y2: [0.0; 2],
        }
    }

    pub fn set_params(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
        self.b0 = b0; self.b1 = b1; self.b2 = b2;
        self.a1 = a1; self.a2 = a2;
    }

    /// Processes a single sample on the specified channel.
    pub fn process(&mut self, input: f32, ch: usize) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1[ch] + self.b2 * self.x2[ch]
            - self.a1 * self.y1[ch] - self.a2 * self.y2[ch];

        self.x2[ch] = self.x1[ch];
        self.x1[ch] = input;
        self.y2[ch] = self.y1[ch];
        self.y1[ch] = output;

        output
    }

    pub fn clear(&mut self) {
        self.x1 = [0.0; 2]; self.x2 = [0.0; 2];
        self.y1 = [0.0; 2]; self.y2 = [0.0; 2];
    }
}

impl Default for BiquadFilter {
    fn default() -> Self { Self::new() }
}

/// A multi-band parametric equalizer.
///
/// The Equalizer cascades multiple biquad filter bands to shape the 
/// frequency response of the signal. It supports various filter types
/// and provides independent control over frequency, gain, and Q.
pub struct Equalizer {
    sample_rate: f64,
    bands: Vec<BiquadFilter>,
    gains: Vec<f32>,
    frequencies: Vec<f32>,
    q_values: Vec<f32>,
    filter_types: Vec<FilterType>,
}

impl Equalizer {
    /// Creates a new 7-band parametric equalizer.
    pub fn new(sample_rate: f64) -> Self {
        let frequencies = vec![60.0, 170.0, 350.0, 1000.0, 3500.0, 7000.0, 12000.0];
        let band_count = frequencies.len();
        let mut eq = Self {
            sample_rate,
            bands: vec![BiquadFilter::new(); band_count],
            gains: vec![0.0; band_count],
            q_values: vec![1.4; band_count],
            frequencies,
            filter_types: vec![FilterType::Peaking; band_count],
        };
        eq.update_all_filters();
        eq
    }

    /// Calculates coefficients for a specific band based on its type and parameters.
    fn calc_coeffs(&self, idx: usize) -> (f32, f32, f32, f32, f32) {
        let freq = self.frequencies[idx];
        let gain_db = self.gains[idx];
        let q = self.q_values[idx];

        let fs = self.sample_rate as f32;
        let omega = (2.0 * PI as f32 * freq / fs).min(PI as f32 - 0.001);
        let cos_w = omega.cos();
        let sin_w = omega.sin();
        let alpha = sin_w / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match self.filter_types[idx] {
            FilterType::LowPass => (
                (1.0 - cos_w) / 2.0, 1.0 - cos_w, (1.0 - cos_w) / 2.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha
            ),
            FilterType::HighPass => (
                (1.0 + cos_w) / 2.0, -(1.0 + cos_w), (1.0 + cos_w) / 2.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha
            ),
            FilterType::BandPass => (
                alpha, 0.0, -alpha,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha
            ),
            FilterType::LowShelf => {
                let s = 1.0; // Shelf slope
                let alpha_s = (sin_w / 2.0) * (((a + 1.0/a)*(1.0/s - 1.0) + 2.0).sqrt());
                (
                    a * ((a + 1.0) - (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s),
                    2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w),
                    a * ((a + 1.0) - (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s),
                    (a + 1.0) + (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s,
                    -2.0 * ((a - 1.0) + (a + 1.0) * cos_w),
                    (a + 1.0) + (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s
                )
            },
            FilterType::HighShelf => {
                let s = 1.0;
                let alpha_s = (sin_w / 2.0) * (((a + 1.0/a)*(1.0/s - 1.0) + 2.0).sqrt());
                (
                    a * ((a + 1.0) + (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s),
                    -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w),
                    a * ((a + 1.0) + (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s),
                    (a + 1.0) - (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s,
                    2.0 * ((a - 1.0) - (a + 1.0) * cos_w),
                    (a + 1.0) - (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s
                )
            },
            FilterType::Notch => (
                1.0, -2.0 * cos_w, 1.0,
                1.0 + alpha, -2.0 * cos_w, 1.0 - alpha
            ),
            FilterType::Peaking => (
                1.0 + alpha * a, -2.0 * cos_w, 1.0 - alpha * a,
                1.0 + alpha / a, -2.0 * cos_w, 1.0 - alpha / a
            ),
        };

        (b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    fn update_all_filters(&mut self) {
        for i in 0..self.bands.len() {
            let (b0, b1, b2, a1, a2) = self.calc_coeffs(i);
            self.bands[i].set_params(b0, b1, b2, a1, a2);
        }
    }

    pub fn set_gain(&mut self, band: usize, gain_db: f32) {
        if band < self.gains.len() {
            self.gains[band] = gain_db.clamp(-24.0, 24.0);
            let (b0, b1, b2, a1, a2) = self.calc_coeffs(band);
            self.bands[band].set_params(b0, b1, b2, a1, a2);
        }
    }

    pub fn set_frequency(&mut self, band: usize, freq: f32) {
        if band < self.frequencies.len() {
            self.frequencies[band] = freq.clamp(20.0, (self.sample_rate / 2.0 - 100.0) as f32);
            let (b0, b1, b2, a1, a2) = self.calc_coeffs(band);
            self.bands[band].set_params(b0, b1, b2, a1, a2);
        }
    }

    pub fn set_q(&mut self, band: usize, q: f32) {
        if band < self.q_values.len() {
            self.q_values[band] = q.clamp(0.1, 10.0);
            let (b0, b1, b2, a1, a2) = self.calc_coeffs(band);
            self.bands[band].set_params(b0, b1, b2, a1, a2);
        }
    }

    /// Processes a single sample through all EQ bands.
    pub fn process(&mut self, input: f32, ch: usize) -> f32 {
        let mut output = input;
        for band in &mut self.bands {
            output = band.process(output, ch);
        }
        output
    }

    /// Processes stereo samples through independent sets of EQ delay lines.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left, 0), self.process(right, 1))
    }

    pub fn clear(&mut self) {
        for band in &mut self.bands { band.clear(); }
    }
}

impl Default for Equalizer {
    fn default() -> Self { Self::new(44100.0) }
}
