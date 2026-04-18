pub struct Octave {
    sample_rate: f64,
    up_octave: Vec<f32>,
    down_octave: Vec<f32>,
    up_idx: usize,
    down_idx: usize,
    up_step: f32,
    down_step: f32,
    mix: f32,
}

impl Octave {
    pub fn new(sample_rate: f64) -> Self {
        let buffer_size = (sample_rate * 0.02) as usize;
        Self {
            sample_rate,
            up_octave: vec![0.0; buffer_size.max(1)],
            down_octave: vec![0.0; buffer_size.max(1)],
            up_idx: 0,
            down_idx: 0,
            up_step: 0.5,
            down_step: 2.0,
            mix: 0.5,
        }
    }

    pub fn set_octave(&mut self, octave_shift: i32) {
        match octave_shift {
            -2 => {
                self.up_step = 0.25;
                self.down_step = 4.0;
            }
            -1 => {
                self.up_step = 0.5;
                self.down_step = 2.0;
            }
            1 => {
                self.up_step = 2.0;
                self.down_step = 0.5;
            }
            2 => {
                self.up_step = 4.0;
                self.down_step = 0.25;
            }
            _ => {
                self.up_step = 1.0;
                self.down_step = 1.0;
            }
        }
    }

    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.up_octave[self.up_idx] = input;
        self.up_idx = (self.up_idx + 1) % self.up_octave.len();

        let up_read_idx = ((self.up_idx as f32 - self.up_octave.len() as f32 / self.up_step)
            as usize)
            % self.up_octave.len();
        let up_sample = self.up_octave[up_read_idx];

        self.down_octave[self.down_idx] = input;
        self.down_idx = (self.down_idx + 1) % self.down_octave.len();

        let down_read_idx = ((self.down_idx as f32 - self.down_octave.len() as f32 * self.down_step)
            as usize)
            % self.down_octave.len();
        let down_sample = self.down_octave[down_read_idx];

        if self.up_step == 1.0 && self.down_step == 1.0 {
            input
        } else {
            input * (1.0 - self.mix) + (up_sample + down_sample) * 0.5 * self.mix
        }
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }

    pub fn clear(&mut self) {
        self.up_octave.fill(0.0);
        self.down_octave.fill(0.0);
    }
}

impl Default for Octave {
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
