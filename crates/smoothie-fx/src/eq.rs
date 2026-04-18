//! Parametric EQ — up to 8 bands, each a biquad filter.
//! Supports: Peak, LowShelf, HighShelf, LowPass, HighPass, Notch, AllPass, BandPass.

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EqBandType {
    Peak,
    LowShelf,
    HighShelf,
    LowPass,
    HighPass,
    Notch,
    AllPass,
    BandPass,
    Off,
}

/// Single parametric EQ band (biquad, direct-form II transposed).
#[derive(Clone, Debug)]
pub struct EqBand {
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    z1: f32, z2: f32,
    pub freq:    f32,
    pub gain_db: f32,
    pub q:       f32,
    pub band_type: EqBandType,
    sample_rate: f32,
}

impl EqBand {
    pub fn new(freq: f32, gain_db: f32, q: f32, band_type: EqBandType, sample_rate: f32) -> Self {
        let mut band = Self {
            b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0,
            z1: 0.0, z2: 0.0,
            freq, gain_db, q, band_type, sample_rate,
        };
        band.recalculate();
        band
    }

    pub fn set_freq(&mut self, hz: f32) { self.freq = hz; self.recalculate(); }
    pub fn set_gain(&mut self, db: f32) { self.gain_db = db; self.recalculate(); }
    pub fn set_q(&mut self, q: f32)     { self.q = q; self.recalculate(); }
    pub fn set_type(&mut self, t: EqBandType) { self.band_type = t; self.recalculate(); }

    pub fn recalculate(&mut self) {
        if self.band_type == EqBandType::Off {
            self.b0 = 1.0; self.b1 = 0.0; self.b2 = 0.0;
            self.a1 = 0.0; self.a2 = 0.0;
            return;
        }
        let omega = 2.0 * PI * self.freq / self.sample_rate;
        let cos_w = omega.cos();
        let sin_w = omega.sin();
        let alpha = sin_w / (2.0 * self.q);
        let a = 10.0_f32.powf(self.gain_db / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match self.band_type {
            EqBandType::Peak => (
                1.0 + alpha * a,
                -2.0 * cos_w,
                1.0 - alpha * a,
                1.0 + alpha / a,
                -2.0 * cos_w,
                1.0 - alpha / a,
            ),
            EqBandType::LowShelf => {
                let alpha_s = sin_w / 2.0 * ((a + 1.0 / a) * (1.0 / (1.0 / self.q) - 1.0) + 2.0).sqrt();
                (
                    a * ((a + 1.0) - (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s),
                    2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w),
                    a * ((a + 1.0) - (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s),
                    (a + 1.0) + (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s,
                    -2.0 * ((a - 1.0) + (a + 1.0) * cos_w),
                    (a + 1.0) + (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s,
                )
            }
            EqBandType::HighShelf => {
                let alpha_s = sin_w / 2.0 * ((a + 1.0 / a) * (1.0 / (1.0 / self.q) - 1.0) + 2.0).sqrt();
                (
                    a * ((a + 1.0) + (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s),
                    -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w),
                    a * ((a + 1.0) + (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s),
                    (a + 1.0) - (a - 1.0) * cos_w + 2.0 * a.sqrt() * alpha_s,
                    2.0 * ((a - 1.0) - (a + 1.0) * cos_w),
                    (a + 1.0) - (a - 1.0) * cos_w - 2.0 * a.sqrt() * alpha_s,
                )
            }
            EqBandType::LowPass  => (sin_w / 2.0 * (1.0 - cos_w), sin_w * (1.0 - cos_w) * 0.5, 0.0, 1.0 + alpha, -2.0 * cos_w, 1.0 - alpha),
            EqBandType::HighPass => ((1.0 + cos_w) / 2.0, -(1.0 + cos_w), (1.0 + cos_w) / 2.0, 1.0 + alpha, -2.0 * cos_w, 1.0 - alpha),
            EqBandType::Notch    => (1.0, -2.0 * cos_w, 1.0, 1.0 + alpha, -2.0 * cos_w, 1.0 - alpha),
            EqBandType::AllPass  => (1.0 - alpha, -2.0 * cos_w, 1.0 + alpha, 1.0 + alpha, -2.0 * cos_w, 1.0 - alpha),
            EqBandType::BandPass => (alpha, 0.0, -alpha, 1.0 + alpha, -2.0 * cos_w, 1.0 - alpha),
            EqBandType::Off      => unreachable!(),
        };

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }

    pub fn reset(&mut self) { self.z1 = 0.0; self.z2 = 0.0; }
}

/// 8-band parametric EQ.
pub struct ParametricEq {
    pub bands: [EqBand; 8],
}

impl ParametricEq {
    pub fn new(sample_rate: f32) -> Self {
        let mk = |f, g, q, t| EqBand::new(f, g, q, t, sample_rate);
        Self {
            bands: [
                mk(80.0,    0.0, 0.707, EqBandType::LowShelf),
                mk(200.0,   0.0, 1.0,   EqBandType::Peak),
                mk(500.0,   0.0, 1.0,   EqBandType::Peak),
                mk(1000.0,  0.0, 1.0,   EqBandType::Peak),
                mk(2000.0,  0.0, 1.0,   EqBandType::Peak),
                mk(4000.0,  0.0, 1.0,   EqBandType::Peak),
                mk(8000.0,  0.0, 1.0,   EqBandType::Peak),
                mk(12000.0, 0.0, 0.707, EqBandType::HighShelf),
            ]
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let mut y = x;
        for band in self.bands.iter_mut() {
            y = band.process(y);
        }
        y
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
