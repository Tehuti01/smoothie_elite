//! Multi-tap delay processor.

use crate::EffectProcessor;

const MAX_DELAY_SAMPLES: usize = 192000; // 4 seconds @ 48kHz

/// Multi-tap delay processor.
///
/// Produces echoes with feedback and damping.
pub struct DelayProcessor {
    buffer: Vec<f32>,
    write_pos: usize,
    sample_rate: f32,

    // Parameters
    delay_ms: f32,
    feedback: f32,
    damping: f32,
    wet_level: f32,
    dry_level: f32,
}

impl DelayProcessor {
    /// Create a new delay processor.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            buffer: vec![0.0; MAX_DELAY_SAMPLES],
            write_pos: 0,
            sample_rate,
            delay_ms: 500.0,
            feedback: 0.4,
            damping: 0.5,
            wet_level: 0.3,
            dry_level: 0.7,
        }
    }

    /// Set delay time in milliseconds.
    pub fn set_delay(&mut self, ms: f32) {
        self.delay_ms = ms.clamp(1.0, 4000.0);
    }

    /// Set feedback amount (0.0–1.0).
    pub fn set_feedback(&mut self, fb: f32) {
        self.feedback = fb.clamp(0.0, 0.95);
    }

    /// Set damping filter amount (0.0–1.0).
    pub fn set_damping(&mut self, damp: f32) {
        self.damping = damp.clamp(0.0, 1.0);
    }

    /// Set wet/dry mix.
    pub fn set_mix(&mut self, wet: f32, dry: f32) {
        let total = wet + dry;
        if total > 0.0 {
            self.wet_level = wet / total;
            self.dry_level = dry / total;
        }
    }

    fn get_delay_samples(&self) -> usize {
        ((self.delay_ms * self.sample_rate / 1000.0) as usize).min(MAX_DELAY_SAMPLES - 1)
    }
}

impl EffectProcessor for DelayProcessor {
    fn process(&mut self, input: f32) -> f32 {
        let delay_samples = self.get_delay_samples();
        let read_pos = (self.write_pos + MAX_DELAY_SAMPLES - delay_samples) % MAX_DELAY_SAMPLES;

        let delayed = self.buffer[read_pos];

        // Feedback with damping
        let damped = delayed * (1.0 - self.damping) + self.buffer[read_pos] * self.damping;
        let feedback_signal = input + damped * self.feedback;

        self.buffer[self.write_pos] = feedback_signal.clamp(-1.0, 1.0);
        self.write_pos = (self.write_pos + 1) % MAX_DELAY_SAMPLES;

        input * self.dry_level + delayed * self.wet_level
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }
}

impl Default for DelayProcessor {
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
