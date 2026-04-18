use std::f32::consts::PI;

pub struct Wah {
    sample_rate: f64,
    min_freq: f32,
    max_freq: f32,
    freq: f32,
    q: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl Wah {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            min_freq: 200.0,
            max_freq: 4000.0,
            freq: 1000.0,
            q: 2.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }

    fn update_coeffs(&mut self) {
        let f = self
            .freq
            .clamp(20.0, (self.sample_rate as f32 / 2.0) - 100.0);
        let q = self.q.clamp(0.5, 10.0);

        let omega = 2.0 * PI * f / self.sample_rate as f32;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        self.b0 = 1.0 + alpha;
        self.b1 = -2.0 * cos_omega;
        self.b2 = 1.0 - alpha;
        self.a1 = -2.0 * cos_omega;
        self.a2 = 1.0 - alpha;

        self.b0 /= self.b0;
        self.b1 /= self.b0;
        self.b2 /= self.b0;
        self.a1 /= self.b0;
        self.a2 /= self.b0;
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.freq = freq.clamp(self.min_freq, self.max_freq);
        self.update_coeffs();
    }

    pub fn set_range(&mut self, min_freq: f32, max_freq: f32) {
        self.min_freq = min_freq.clamp(50.0, 1000.0);
        self.max_freq = max_freq.clamp(2000.0, 8000.0);
        self.freq = self.freq.clamp(self.min_freq, self.max_freq);
        self.update_coeffs();
    }

    pub fn set_q(&mut self, q: f32) {
        self.q = q;
        self.update_coeffs();
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }

    pub fn clear(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl Default for Wah {
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
