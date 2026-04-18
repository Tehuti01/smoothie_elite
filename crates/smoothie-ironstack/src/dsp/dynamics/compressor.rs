/// A dynamic range compressor for controlling signal peaks and average levels.
///
/// This implementation features a soft-knee curve, variable attack/release 
/// times, and stereo linking. It uses an envelope-follower sidechain to 
/// calculate gain reduction smoothly.
pub struct Compressor {
    sample_rate: f32,
    /// Level at which compression starts (dB).
    threshold: f32,
    /// Compression ratio (e.g., 4.0 for 4:1).
    ratio: f32,
    attack_ms: f32,
    release_ms: f32,
    /// Width of the soft-knee transition (dB).
    knee_db: f32,
    /// Output gain compensation (dB).
    makeup_gain: f32,
    /// Current sidechain envelope level (dB).
    envelope: f32,
}

impl Compressor {
    /// Creates a new Compressor.
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            threshold: -20.0,
            ratio: 4.0,
            attack_ms: 10.0,
            release_ms: 100.0,
            knee_db: 6.0,
            makeup_gain: 0.0,
            envelope: -120.0,
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = threshold_db.clamp(-60.0, 0.0);
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(1.0, 20.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.clamp(0.1, 100.0);
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.clamp(10.0, 1000.0);
    }

    pub fn set_knee(&mut self, knee_db: f32) {
        self.knee_db = knee_db.clamp(0.0, 24.0);
    }

    pub fn set_makeup_gain(&mut self, gain_db: f32) {
        self.makeup_gain = gain_db.clamp(0.0, 24.0);
    }

    /// Calculates gain reduction in dB based on the input sidechain level.
    fn get_gain_reduction(&self, input_db: f32) -> f32 {
        let knee_start = self.threshold - self.knee_db / 2.0;
        let knee_end = self.threshold + self.knee_db / 2.0;

        if input_db < knee_start {
            0.0
        } else if input_db > knee_end {
            (input_db - self.threshold) * (1.0 - 1.0 / self.ratio)
        } else {
            // Soft-knee transition
            let x = (input_db - knee_start) / self.knee_db;
            let ratio_reduction = 1.0 - 1.0 / self.ratio;
            x * x * self.knee_db / 4.0 * ratio_reduction
        }
    }

    /// Internal method to update the envelope follower.
    fn update_envelope(&mut self, input_db: f32) {
        let attack_coeff = (1.0 - (-1.0 / (self.attack_ms * self.sample_rate / 1000.0)).exp());
        let release_coeff = (1.0 - (-1.0 / (self.release_ms * self.sample_rate / 1000.0)).exp());

        if input_db > self.envelope {
            self.envelope += attack_coeff * (input_db - self.envelope);
        } else {
            self.envelope += release_coeff * (input_db - self.envelope);
        }
    }

    /// Processes a single mono sample.
    pub fn process(&mut self, input: f32) -> f32 {
        let input_abs = input.abs().max(1e-6);
        let input_db = 20.0 * input_abs.log10();

        self.update_envelope(input_db);

        let reduction_db = self.get_gain_reduction(self.envelope);
        let total_gain_db = self.makeup_gain - reduction_db;
        let linear_gain = 10.0_f32.powf(total_gain_db / 20.0);

        input * linear_gain
    }

    /// Processes stereo samples with a linked sidechain.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        // Linked sidechain: detect peak from both channels
        let sidechain_val = left.abs().max(right.abs()).max(1e-6);
        let sidechain_db = 20.0 * sidechain_val.log10();

        self.update_envelope(sidechain_db);

        let reduction_db = self.get_gain_reduction(self.envelope);
        let total_gain_db = self.makeup_gain - reduction_db;
        let linear_gain = 10.0_f32.powf(total_gain_db / 20.0);

        (left * linear_gain, right * linear_gain)
    }

    /// Returns the current gain reduction in dB.
    pub fn get_gain_reduction_db(&self) -> f32 {
        self.get_gain_reduction(self.envelope)
    }
}

impl Default for Compressor {
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
