use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Vibrato {
    sample_rate: f32,
    rate: f32,
    depth: f32,
    lfo_phase: f32,
    delay_buffer: Vec<f32>,
    write_index: usize,
}

impl Vibrato {
    pub fn new(sample_rate: u32) -> Self {
        let buffer_size = ((50.0 * sample_rate as f32 / 1000.0) as usize + 100).max(100);

        Self {
            sample_rate: sample_rate as f32,
            rate: 5.0,
            depth: 0.5,
            lfo_phase: 0.0,
            delay_buffer: vec![0.0; buffer_size],
            write_index: 0,
        }
    }

    pub fn set_rate(&mut self, r: f32) {
        self.rate = r.clamp(0.1, 20.0);
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: Sample) -> Sample {
        self.lfo_phase += 2.0 * PI * self.rate / self.sample_rate;
        if self.lfo_phase > 2.0 * PI {
            self.lfo_phase -= 2.0 * PI;
        }

        let lfo = (self.lfo_phase.sin() * 0.5 + 0.5) * self.depth;
        let delay = 1.0 + lfo * 10.0;

        let idx = (self.write_index + self.delay_buffer.len() - delay as usize - 1)
            % self.delay_buffer.len();
        let idx2 = (idx + 1) % self.delay_buffer.len();

        let delayed = self.delay_buffer[idx]
            + (self.delay_buffer[idx2] - self.delay_buffer[idx]) * (delay - delay as f32);

        self.delay_buffer[self.write_index] = (input.left + input.right) * 0.5;
        self.write_index = (self.write_index + 1) % self.delay_buffer.len();

        Sample {
            left: delayed,
            right: delayed,
        }
    }

    pub fn reset(&mut self) {
        self.delay_buffer.fill(0.0);
        self.write_index = 0;
        self.lfo_phase = 0.0;
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
