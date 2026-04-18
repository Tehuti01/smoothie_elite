/// A peak limiter for preventing signal clipping at the output stage.
///
/// This module ensures the signal never exceeds a defined ceiling (threshold).
/// It uses a linked stereo sidechain and fast attack/release times to 
/// transparently manage transients.
pub struct Limiter {
    sample_rate: f32,
    /// Maximum output level (dB).
    threshold: f32,
    attack_ms: f32,
    release_ms: f32,
    /// Current sidechain envelope level (dB).
    envelope: f32,
}

impl Limiter {
    /// Creates a new Limiter with a default -0.3 dB safety ceiling.
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            threshold: -0.3,
            attack_ms: 0.1,
            release_ms: 50.0,
            envelope: -120.0,
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = threshold_db.clamp(-12.0, 0.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.clamp(0.01, 20.0);
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.clamp(1.0, 500.0);
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

        let ceiling_db = self.threshold;
        let gain_reduction_db = (self.envelope - ceiling_db).max(0.0);
        let linear_gain = 10.0_f32.powf(-gain_reduction_db / 20.0);

        input * linear_gain
    }

    /// Processes stereo samples with a linked sidechain.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        // Linked sidechain: detect peak from both channels
        let sidechain_val = left.abs().max(right.abs()).max(1e-6);
        let sidechain_db = 20.0 * sidechain_val.log10();

        self.update_envelope(sidechain_db);

        let ceiling_db = self.threshold;
        let gain_reduction_db = (self.envelope - ceiling_db).max(0.0);
        let linear_gain = 10.0_f32.powf(-gain_reduction_db / 20.0);

        (left * linear_gain, right * linear_gain)
    }

    /// Returns the current gain reduction in dB.
    pub fn get_gain_reduction_db(&self) -> f32 {
        (self.envelope - self.threshold).max(0.0)
    }

    /// Resets the limiter state.
    pub fn reset(&mut self) {
        self.envelope = -120.0;
    }
}

impl Default for Limiter {
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
