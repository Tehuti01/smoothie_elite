//! Chorus/Flanger effect processor.

use crate::{EffectProcessor, Lfo, LfoShape};

const MAX_CHORUS_DELAY: usize = 48000; // 1 second @ 48kHz

/// Chorus/Flanger effect processor.
///
/// Creates width and movement by modulating a delay line.
pub struct ChorusProcessor {
    buffer: Vec<f32>,
    write_pos: usize,
    sample_rate: f32,

    lfo: Lfo,
    depth: f32,
    rate: f32,
    delay_ms: f32,
    wet_level: f32,
    dry_level: f32,
}

impl ChorusProcessor {
    /// Create a new chorus processor.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: vec![0.0; MAX_CHORUS_DELAY],
            write_pos: 0,
            sample_rate,
            lfo: Lfo::new(1.5, LfoShape::Sine, sample_rate),
            depth: 5.0,
            rate: 1.5,
            delay_ms: 20.0,
            wet_level: 0.5,
            dry_level: 0.5,
        }
    }

    /// Set LFO rate (Hz).
    pub fn set_rate(&mut self, hz: f32) {
        self.rate = hz.clamp(0.1, 10.0);
        self.lfo.set_frequency(self.rate);
    }

    /// Set modulation depth in milliseconds.
    pub fn set_depth(&mut self, ms: f32) {
        self.depth = ms.clamp(1.0, 50.0);
    }

    /// Set center delay time in milliseconds.
    pub fn set_delay(&mut self, ms: f32) {
        self.delay_ms = ms.clamp(5.0, 100.0);
    }

    /// Set wet/dry mix.
    pub fn set_mix(&mut self, wet: f32, dry: f32) {
        let total = wet + dry;
        if total > 0.0 {
            self.wet_level = wet / total;
            self.dry_level = dry / total;
        }
    }
}

impl EffectProcessor for ChorusProcessor {
    fn process(&mut self, input: f32) -> f32 {
        // Get LFO modulation
        let lfo_out = self.lfo.next_sample();

        // Calculate modulated delay time
        let min_delay = (self.delay_ms - self.depth / 2.0).max(0.1);
        let max_delay = self.delay_ms + self.depth / 2.0;
        let delay_ms = min_delay + (max_delay - min_delay) * lfo_out;
        let delay_samples = ((delay_ms * self.sample_rate / 1000.0) as usize).min(MAX_CHORUS_DELAY - 1);

        // Read from delay buffer
        let read_pos = (self.write_pos + MAX_CHORUS_DELAY - delay_samples) % MAX_CHORUS_DELAY;
        let delayed = self.buffer[read_pos];

        // Write to buffer
        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % MAX_CHORUS_DELAY;

        // Mix
        input * self.dry_level + delayed * self.wet_level
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo.reset();
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.lfo.set_sample_rate(sr);
    }
}

impl Default for ChorusProcessor {
    fn default() -> Self {
        Self::new(44100.0)
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
