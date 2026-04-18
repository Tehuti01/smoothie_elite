use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Tremolo {
    sample_rate: f32,
    rate: f32,
    depth: f32,
    shape: TremoloShape,
    lfo_phase: f32,
}

#[derive(Clone, Copy)]
pub enum TremoloShape {
    Sine,
    Triangle,
    Square,
}

impl Tremolo {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            rate: 4.0,
            depth: 0.5,
            shape: TremoloShape::Sine,
            lfo_phase: 0.0,
        }
    }

    pub fn set_rate(&mut self, r: f32) {
        self.rate = r.clamp(0.1, 20.0);
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d.clamp(0.0, 1.0);
    }
    pub fn set_shape(&mut self, s: TremoloShape) {
        self.shape = s;
    }

    pub fn process(&mut self, input: Sample) -> Sample {
        self.lfo_phase += 2.0 * PI * self.rate / self.sample_rate;
        if self.lfo_phase > 2.0 * PI {
            self.lfo_phase -= 2.0 * PI;
        }

        let lfo = match self.shape {
            TremoloShape::Sine => self.lfo_phase.sin() * 0.5 + 0.5,
            TremoloShape::Triangle => 1.0 - (self.lfo_phase / PI).abs() * 2.0,
            TremoloShape::Square => {
                if self.lfo_phase < PI {
                    1.0
                } else {
                    0.0
                }
            }
        };

        let gain = 1.0 - lfo * self.depth;

        Sample {
            left: input.left * gain,
            right: input.right * gain,
        }
    }

    pub fn reset(&mut self) {
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
