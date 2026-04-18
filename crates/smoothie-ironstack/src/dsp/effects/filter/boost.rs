pub struct Boost {
    gain: f32,
    level: f32,
}

impl Boost {
    pub fn new() -> Self {
        Self {
            gain: 1.0,
            level: 0.0,
        }
    }

    pub fn set_gain(&mut self, gain_db: f32) {
        self.gain = 10.0_f32.powf(gain_db / 20.0);
    }

    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        input * self.gain * (1.0 - self.level) + input * self.level
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }
}

impl Default for Boost {
    fn default() -> Self {
        Self::new()
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
