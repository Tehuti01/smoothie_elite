use super::tube_stage::TubeStage;
use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Preamp {
    sample_rate: f32,
    stages: [TubeStage; 2],
    bass: f32,
    treble: f32,
    level: f32,
    low_filter: OnePoleFilter,
    high_filter: OnePoleFilter,
}

/// A basic one-pole IIR filter for simple EQ shaping.
struct OnePoleFilter {
    coeff: f32,
    /// Per-channel state (z-1) for [Left, Right].
    state: [f32; 2],
}

impl OnePoleFilter {
    /// Creates a new OnePole filter with the given cutoff and sample rate.
    fn new(cutoff: f32, sample_rate: f32) -> Self {
        let coeff = (-2.0 * PI * cutoff / sample_rate).exp();
        Self { coeff, state: [0.0; 2] }
    }

    /// Processes a single sample on a specific channel.
    fn process(&mut self, input: f32, ch: usize) -> f32 {
        self.state[ch] = self.state[ch] * self.coeff + input * (1.0 - self.coeff);
        self.state[ch]
    }
}

impl Preamp {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        Self {
            sample_rate: sr,
            stages: [TubeStage::new(sr), TubeStage::new(sr)],
            bass: 0.5,
            treble: 0.5,
            level: 1.0,
            low_filter: OnePoleFilter::new(200.0, sr),
            high_filter: OnePoleFilter::new(4000.0, sr),
        }
    }

    pub fn set_bass(&mut self, b: f32) {
        self.bass = b.clamp(0.0, 1.0);
    }
    pub fn set_treble(&mut self, t: f32) {
        self.treble = t.clamp(0.0, 1.0);
    }
    pub fn set_level(&mut self, l: f32) {
        self.level = l.clamp(0.0, 2.0);
    }

    /// Processes audio through several cascaded tube stages and a basic tonestack.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut out = input;

        // Cascade through independent tube stages for left and right
        out.left = self.stages[0].process(out.left, 0);
        out.right = self.stages[1].process(out.right, 1);

        // Basic bass control via low-shelf filtering logic
        out.left = self.low_filter.process(out.left, 0) * (self.bass - 0.5) * 0.5 + out.left;
        out.right = self.low_filter.process(out.right, 1) * (self.bass - 0.5) * 0.5 + out.right;

        out.left *= self.level;
        out.right *= self.level;

        out
    }

    /// Resets all stage states to silence.
    pub fn reset(&mut self) {
        self.stages[0].reset();
        self.stages[1].reset();
        self.low_filter.state = [0.0; 2];
        self.high_filter.state = [0.0; 2];
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
