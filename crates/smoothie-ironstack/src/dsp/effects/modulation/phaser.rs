use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Phaser {
    sample_rate: f32,
    rate: f32,
    depth: f32,
    resonance: f32,
    mix: f32,
    lfo_phase: f32,
    stages: usize,
    stage_states: Vec<[[f32; 2]; 2]>,
}

impl Phaser {
    pub fn new(sample_rate: u32) -> Self {
        let stages = 4;
        Self {
            sample_rate: sample_rate as f32,
            rate: 1.0,
            depth: 0.7,
            resonance: 0.5,
            mix: 0.5,
            lfo_phase: 0.0,
            stages,
            stage_states: vec![[[0.0; 2]; 2]; stages],
        }
    }

    pub fn set_rate(&mut self, r: f32) {
        self.rate = r.clamp(0.01, 10.0);
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d.clamp(0.0, 1.0);
    }
    pub fn set_resonance(&mut self, q: f32) {
        self.resonance = q.clamp(0.0, 1.0);
    }
    pub fn set_mix(&mut self, m: f32) {
        self.mix = m.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: Sample) -> Sample {
        self.lfo_phase += 2.0 * PI * self.rate / self.sample_rate;
        if self.lfo_phase > 2.0 * PI {
            self.lfo_phase -= 2.0 * PI;
        }

        let min_freq = 440.0_f32;
        let max_freq = 16000.0_f32;
        let center_freq = min_freq * (max_freq / min_freq).powf(self.lfo_phase.sin() * 0.5 + 0.5);

        let mut out = (input.left + input.right) * 0.5;

        for s in 0..self.stages {
            out = self.allpass(out, center_freq, 0.5 + self.resonance * 10.0, s);
        }

        out *= self.depth;

        let wet = out * self.mix;
        let dry = 1.0 - self.mix * 0.5;

        Sample {
            left: input.left * dry + wet,
            right: input.right * dry + wet,
        }
    }

    fn allpass(&mut self, input: f32, freq: f32, q: f32, stage: usize) -> f32 {
        let w0 = 2.0 * PI * freq / self.sample_rate;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);

        let b0 = 1.0 - alpha;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 + alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        let x1 = self.stage_states[stage][0][0];
        let x2 = self.stage_states[stage][0][1];
        let y1 = self.stage_states[stage][1][0];
        let y2 = self.stage_states[stage][1][1];

        let y0 =
            (b0 / a0) * input + (b1 / a0) * x1 + (b2 / a0) * x2 - (a1 / a0) * y1 - (a2 / a0) * y2;

        self.stage_states[stage][0][0] = input;
        self.stage_states[stage][0][1] = x1;
        self.stage_states[stage][1][0] = y0;
        self.stage_states[stage][1][1] = y1;

        y0
    }

    pub fn reset(&mut self) {
        self.stage_states = vec![[[0.0; 2]; 2]; self.stages];
        self.lfo_phase = 0.0;
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let sample = Sample::new(left, right);
        let result = self.process(sample);
        (result.left, result.right)
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
