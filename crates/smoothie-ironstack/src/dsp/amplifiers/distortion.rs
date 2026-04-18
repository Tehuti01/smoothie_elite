use serde::{Serialize, Deserialize};

/// Defines the clipping characteristics of the distortion effect.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistortionMode {
    /// Smooth, tube-like saturation using tanh logic.
    Soft,
    /// Harsh clipping that caps the signal at the rail limits.
    Hard,
    /// Extreme, high-gain distortion with exponential clipping.
    Fuzz,
}

/// A versatile distortion effect with selectable clipping modes and tone shaping.
///
/// This module provides various types of guitar distortion, ranging from 
/// subtle overdrive to aggressive fuzz. It includes a built-in low-pass 
/// tone filter to smooth out high-frequency fizz.
pub struct Distortion {
    sample_rate: f64,
    /// Gain applied before clipping.
    drive: f32,
    /// High-frequency roll-off control.
    tone: f32,
    /// Final output gain.
    level: f32,
    /// Active clipping algorithm.
    mode: DistortionMode,
    
    // --- Per-channel Filter State ---
    tone_filter_x1: [f32; 2],
    tone_filter_y1: [f32; 2],
}

impl Distortion {
    /// Creates a new Distortion instance.
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            drive: 0.5,
            tone: 0.5,
            level: 1.0,
            mode: DistortionMode::Soft,
            tone_filter_x1: [0.0; 2],
            tone_filter_y1: [0.0; 2],
        }
    }

    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 1.0);
    }

    pub fn set_tone(&mut self, tone: f32) {
        self.tone = tone.clamp(0.0, 1.0);
    }

    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 2.0);
    }

    pub fn set_mode(&mut self, mode: DistortionMode) {
        self.mode = mode;
    }

    fn apply_soft_clip(&self, input: f32) -> f32 {
        let gain = 1.0 + self.drive * 20.0;
        let x = input * gain;
        x.tanh()
    }

    fn apply_hard_clip(&self, input: f32) -> f32 {
        let gain = 1.0 + self.drive * 15.0;
        let x = input * gain;
        x.clamp(-1.0, 1.0)
    }

    fn apply_fuzz(&self, input: f32) -> f32 {
        let gain = 1.0 + self.drive * 25.0;
        let x = input * gain;
        if x > 0.0 {
            1.0 - (-x).exp()
        } else {
            -1.0 + (x).exp()
        }
    }

    /// Internal one-pole low-pass filter for the tone control.
    fn process_tone(&mut self, input: f32, ch: usize) -> f32 {
        let cutoff = 2000.0 + self.tone * 8000.0;
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / self.sample_rate as f32;
        let alpha = dt / (rc + dt);

        let output = alpha * input + (1.0 - alpha) * self.tone_filter_y1[ch];
        self.tone_filter_x1[ch] = input;
        self.tone_filter_y1[ch] = output;
        output
    }

    /// Processes a single sample on a specific channel.
    pub fn process(&mut self, input: f32, ch: usize) -> f32 {
        let distorted = match self.mode {
            DistortionMode::Soft => self.apply_soft_clip(input),
            DistortionMode::Hard => self.apply_hard_clip(input),
            DistortionMode::Fuzz => self.apply_fuzz(input),
        };

        let toned = self.process_tone(distorted, ch);
        toned * self.level
    }

    /// Processes stereo samples through independent distortion/filter chains.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left, 0), self.process(right, 1))
    }

    /// Resets all internal delay lines.
    pub fn reset(&mut self) {
        self.tone_filter_x1 = [0.0; 2];
        self.tone_filter_y1 = [0.0; 2];
    }
}

impl Default for Distortion {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distortion_clipping() {
        let mut dist = Distortion::new(44100.0);
        dist.set_drive(1.0);
        dist.set_level(1.0);
        
        dist.set_mode(DistortionMode::Soft);
        let mut output = 0.0;
        for _ in 0..10 {
            output = dist.process(1.0, 0);
        }
        assert!(output < 1.0);
        assert!(output > 0.5);

        dist.set_mode(DistortionMode::Hard);
        for _ in 0..100 {
            output = dist.process(0.5, 0);
        }
        assert!(output > 0.99);
    }

    #[test]
    fn test_distortion_reset() {
        let mut dist = Distortion::new(44100.0);
        dist.tone_filter_y1[0] = 0.5;
        dist.reset();
        assert_eq!(dist.tone_filter_y1[0], 0.0);
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
