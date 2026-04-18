pub struct Gate {
    sample_rate: f32,
    threshold: f32,
    attack_ms: f32,
    hold_ms: f32,
    release_ms: f32,
    envelope: f32,
    hold_counter: usize,
    hold_samples: usize,
    is_open: bool,
}

impl Gate {
    pub fn new(sample_rate: f64) -> Self {
        let sr = sample_rate as f32;
        Self {
            sample_rate: sr,
            threshold: -40.0,
            attack_ms: 0.5,
            hold_ms: 50.0,
            release_ms: 50.0,
            envelope: 0.0,
            hold_counter: 0,
            hold_samples: (50.0 * sr / 1000.0) as usize,
            is_open: false,
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = threshold_db.clamp(-80.0, 0.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.max(0.1).min(50.0);
    }

    pub fn set_hold(&mut self, hold_ms: f32) {
        self.hold_ms = hold_ms.clamp(0.0, 500.0);
        self.hold_samples = (self.hold_ms * self.sample_rate / 1000.0) as usize;
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.max(1.0).max(1000.0);
    }

    fn input_to_db(&self, input: f32) -> f32 {
        let abs = input.abs();
        if abs > 1e-6 {
            20.0 * abs.log10()
        } else {
            -120.0
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let input_db = self.input_to_db(input);

        let attack_coeff = (-1.0 / (self.attack_ms * self.sample_rate / 1000.0)).exp();
        let release_coeff = (-1.0 / (self.release_ms * self.sample_rate / 1000.0)).exp();

        if input_db > self.envelope {
            self.envelope = attack_coeff * self.envelope + (1.0 - attack_coeff) * input_db;
        } else {
            self.envelope = release_coeff * self.envelope + (1.0 - release_coeff) * input_db;
        }

        let threshold = self.threshold;

        if self.envelope > threshold {
            self.hold_counter = self.hold_samples;
            self.is_open = true;
        } else if self.hold_counter > 0 {
            self.hold_counter -= 1;
        } else {
            self.is_open = false;
        }

        if self.is_open {
            let fade_in = 1.0;
            input * fade_in
        } else {
            0.0
        }
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let env_before = self.envelope;
        (self.process(left), {
            self.envelope = env_before;
            self.process(right)
        })
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Default for Gate {
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
