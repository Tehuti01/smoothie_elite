use super::tube_stage::TubeStage;
use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Amplifier {
    sample_rate: f32,
    gain: f32,
    volume: f32,
    presence: f32,
    bass: f32,
    middle: f32,
    treble: f32,
    sag: f32,
    sag_state: [f32; 2],
    pre_gain: f32,
    post_gain: f32,
    low_shelf: BiquadFilter,
    mid_peak: BiquadFilter,
    high_shelf: BiquadFilter,
    tube_stage: TubeStage,
}

/// A standard second-order IIR (infinite impulse response) filter implementation.
///
/// This implementation uses two independent sets of delay lines for stereo
/// processing to prevent channel crosstalk.
struct BiquadFilter {
    // --- Coefficients ---
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,

    // --- State (Delay Lines) ---
    /// Previous input samples (x[n-1], x[n-2]) for [Left, Right].
    x1: [f32; 2],
    x2: [f32; 2],
    /// Previous output samples (y[n-1], y[n-2]) for [Left, Right].
    y1: [f32; 2],
    y2: [f32; 2],
}

impl BiquadFilter {
    /// Configures the filter as a Low-Shelf equalizer.
    fn low_shelf(sr: f32, freq: f32, gain_db: f32) -> Self {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = 0.5 * (a + 1.0 / a) * (1.0 / 0.707 - 1.0) + (a - 1.0 / a) * 0.5_f32.sqrt();

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha);
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0) / a0,
            a2: ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Configures the filter as a High-Shelf equalizer.
    fn high_shelf(sr: f32, freq: f32, gain_db: f32) -> Self {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = 0.5 * (a + 1.0 / a) * (1.0 / 0.707 - 1.0) + (a - 1.0 / a) * 0.5_f32.sqrt();

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha);
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0) / a0,
            a2: ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Configures the filter as a Peaking/Bell equalizer.
    fn peak(sr: f32, freq: f32, q: f32, gain_db: f32) -> Self {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = w0.sin() / (2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: [0.0; 2],
            x2: [0.0; 2],
            y1: [0.0; 2],
            y2: [0.0; 2],
        }
    }

    /// Processes a single sample through the filter using the Difference Equation.
    fn process(&mut self, sample: f32, ch: usize) -> f32 {
        let y0 = self.b0 * sample 
            + self.b1 * self.x1[ch] 
            + self.b2 * self.x2[ch] 
            - self.a1 * self.y1[ch] 
            - self.a2 * self.y2[ch];

        self.x2[ch] = self.x1[ch];
        self.x1[ch] = sample;
        self.y2[ch] = self.y1[ch];
        self.y1[ch] = y0;

        y0
    }
}

impl Amplifier {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        Self {
            sample_rate: sr,
            gain: 0.75,
            volume: 0.85,
            presence: 0.5,
            bass: 0.5,
            middle: 0.5,
            treble: 0.5,
            sag: 0.15,
            sag_state: [0.0; 2],
            pre_gain: 1.0,
            post_gain: 1.0,
            low_shelf: BiquadFilter::low_shelf(sr, 80.0, 0.0),
            mid_peak: BiquadFilter::peak(sr, 1000.0, 2.0, 0.0),
            high_shelf: BiquadFilter::high_shelf(sr, 8000.0, 0.0),
            tube_stage: TubeStage::new(sr),
        }
    }

    pub fn set_gain(&mut self, g: f32) {
        self.gain = g.clamp(0.0, 1.0);
    }
    pub fn set_volume(&mut self, v: f32) {
        self.volume = v.clamp(0.0, 1.0);
    }
    pub fn set_presence(&mut self, p: f32) {
        self.presence = p.clamp(0.0, 1.0);
    }

    pub fn set_tone(&mut self, bass: f32, middle: f32, treble: f32) {
        self.bass = bass.clamp(0.0, 1.0);
        self.middle = middle.clamp(0.0, 1.0);
        self.treble = treble.clamp(0.0, 1.0);
        self.low_shelf = BiquadFilter::low_shelf(self.sample_rate, 80.0, (self.bass - 0.5) * 12.0);
        self.mid_peak =
            BiquadFilter::peak(self.sample_rate, 1000.0, 2.0, (self.middle - 0.5) * 12.0);
        self.high_shelf =
            BiquadFilter::high_shelf(self.sample_rate, 8000.0, (self.treble - 0.5) * 12.0);
    }

    /// Processes a stereo sample through the entire amplifier signal chain.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut s = input;

        // Apply input gain
        s.left = self.pre_gain * s.left;
        s.right = self.pre_gain * s.right;

        // Vacuum tube emulation stage
        s.left = self.tube_stage.process(s.left, 0);
        s.right = self.tube_stage.process(s.right, 1);

        // Power supply sag (compression emulation)
        s.left = self.sag_effect(s.left, 0);
        s.right = self.sag_effect(s.right, 1);

        // Tonestack (EQ)
        s.left = self.low_shelf.process(s.left, 0);
        s.right = self.low_shelf.process(s.right, 1);
        s.left = self.mid_peak.process(s.left, 0);
        s.right = self.mid_peak.process(s.right, 1);
        s.left = self.high_shelf.process(s.left, 0);
        s.right = self.high_shelf.process(s.right, 1);

        // Apply master volume
        s.left *= self.post_gain * self.volume;
        s.right *= self.post_gain * self.volume;

        s.clip()
    }

    fn sag_effect(&mut self, input: f32, ch: usize) -> f32 {
        let sag_amount = self.sag * 0.3;
        self.sag_state[ch] += (input - self.sag_state[ch]) * (1.0 - sag_amount);
        input * (1.0 - sag_amount) + self.sag_state[ch] * sag_amount
    }

    /// Resets all internal delay lines and states to silence.
    pub fn reset(&mut self) {
        self.sag_state = [0.0; 2];
        self.tube_stage.reset();
        self.low_shelf.x1 = [0.0; 2];
        self.low_shelf.x2 = [0.0; 2];
        self.low_shelf.y1 = [0.0; 2];
        self.low_shelf.y2 = [0.0; 2];
        self.mid_peak.x1 = [0.0; 2];
        self.mid_peak.x2 = [0.0; 2];
        self.mid_peak.y1 = [0.0; 2];
        self.mid_peak.y2 = [0.0; 2];
        self.high_shelf.x1 = [0.0; 2];
        self.high_shelf.x2 = [0.0; 2];
        self.high_shelf.y1 = [0.0; 2];
        self.high_shelf.y2 = [0.0; 2];
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
