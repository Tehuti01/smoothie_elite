//! 3-band parametric equalizer.

use crate::EffectProcessor;
use smoothie_dsp::filters::{BiquadFilter, FilterType};

/// 3-band parametric EQ processor.
///
/// Low, Mid, and High bands with adjustable frequency, gain, and Q.
pub struct EqProcessor {
    low_shelf: BiquadFilter,
    mid_peak: BiquadFilter,
    high_shelf: BiquadFilter,

    low_freq: f32,
    low_gain: f32,
    mid_freq: f32,
    mid_gain: f32,
    high_freq: f32,
    high_gain: f32,
    mid_q: f32,

    sample_rate: f32,
}

impl EqProcessor {
    /// Create a new 3-band EQ.
    pub fn new(sample_rate: f32) -> Self {
        let mut eq = Self {
            low_shelf: BiquadFilter::design(FilterType::LowShelf, 200.0, sample_rate, 0.707, 0.0),
            mid_peak: BiquadFilter::design(FilterType::PeakingEq, 1000.0, sample_rate, 1.0, 0.0),
            high_shelf: BiquadFilter::design(FilterType::HighShelf, 5000.0, sample_rate, 0.707, 0.0),
            low_freq: 200.0,
            low_gain: 0.0,
            mid_freq: 1000.0,
            mid_gain: 0.0,
            high_freq: 5000.0,
            high_gain: 0.0,
            mid_q: 1.0,
            sample_rate,
        };
        eq
    }

    /// Set low band frequency and gain.
    pub fn set_low(&mut self, freq: f32, gain_db: f32) {
        self.low_freq = freq.clamp(20.0, 1000.0);
        self.low_gain = gain_db.clamp(-24.0, 24.0);
        self.low_shelf = BiquadFilter::design(
            FilterType::LowShelf,
            self.low_freq,
            self.sample_rate,
            0.707,
            self.low_gain,
        );
    }

    /// Set mid band frequency, gain, and Q.
    pub fn set_mid(&mut self, freq: f32, gain_db: f32, q: f32) {
        self.mid_freq = freq.clamp(200.0, 8000.0);
        self.mid_gain = gain_db.clamp(-24.0, 24.0);
        self.mid_q = q.clamp(0.5, 10.0);
        self.mid_peak = BiquadFilter::design(
            FilterType::PeakingEq,
            self.mid_freq,
            self.sample_rate,
            self.mid_q,
            self.mid_gain,
        );
    }

    /// Set high band frequency and gain.
    pub fn set_high(&mut self, freq: f32, gain_db: f32) {
        self.high_freq = freq.clamp(2000.0, 20000.0);
        self.high_gain = gain_db.clamp(-24.0, 24.0);
        self.high_shelf = BiquadFilter::design(
            FilterType::HighShelf,
            self.high_freq,
            self.sample_rate,
            0.707,
            self.high_gain,
        );
    }
}

impl EffectProcessor for EqProcessor {
    fn process(&mut self, input: f32) -> f32 {
        let low = self.low_shelf.process(input);
        let mid = self.mid_peak.process(low);
        let high = self.high_shelf.process(mid);
        high
    }

    fn reset(&mut self) {
        self.low_shelf.reset();
        self.mid_peak.reset();
        self.high_shelf.reset();
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.set_low(self.low_freq, self.low_gain);
        self.set_mid(self.mid_freq, self.mid_gain, self.mid_q);
        self.set_high(self.high_freq, self.high_gain);
    }
}

impl Default for EqProcessor {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
