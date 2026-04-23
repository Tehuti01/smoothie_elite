/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x43a8d165 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/imager.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::{vec, vec::Vec};
/// Stereo width control and mid-side processing (Silicon Stable)
use smoothie_core::primitives::Sample;

/// Technical implementation of the StereoImager structure.
pub struct StereoImager {
    width: f32,
    azimuth: f32,
    focus: f32,
    mid_gain: f32,
    side_gain: f32,
    high_freq: f32,
    state: [f32; 4],
    #[allow(dead_code)]
    sample_rate: f32,
}

impl StereoImager {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            width: 1.0,
            azimuth: 0.0,
            focus: 0.5,
            mid_gain: 1.0,
            side_gain: 1.0,
            high_freq: 8000.0,
            state: [0.0; 4],
            sample_rate,
        }
    }

    /// Technical implementation of the set_width logic.
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(0.0, 2.0);
        self.update_gains();
    }

    /// Technical implementation of the set_azimuth logic.
    pub fn set_azimuth(&mut self, azimuth: f32) {
        self.azimuth = azimuth.clamp(-1.0, 1.0);
    }

    /// Technical implementation of the set_focus logic.
    pub fn set_focus(&mut self, focus: f32) {
        self.focus = focus.clamp(0.0, 1.0);
        self.update_gains();
    }

    /// Technical implementation of the set_high_freq logic.
    pub fn set_high_freq(&mut self, freq: f32) {
        self.high_freq = freq.max(2000.0).min(18000.0);
    }

    /// Technical implementation of the update_gains logic.
    fn update_gains(&mut self) {
        let side_width = self.width * (1.0 - self.focus * 0.5);
        self.mid_gain = 1.0 / (1.0 + side_width * 0.5).sqrt();
        self.side_gain = side_width;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        let mid = (left + right) * 0.5;
        let side = (left - right) * 0.5;

        let processed_mid = mid * self.mid_gain;
        let processed_side = side * self.side_gain;

        let delay_samples = (self.azimuth * 0.5 * self.sample_rate / 1000.0) as usize;
        let _delay_idx = (self.state[2] as usize) % delay_samples.max(1);
        let _delayed = self.state[3.min(delay_samples.saturating_sub(1))];

        let out_left = processed_mid + processed_side;
        let out_right = processed_mid - processed_side;

        self.state[2] = (self.state[2] + 1.0) % delay_samples.max(1) as f32;

        (out_left, out_right)
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: &[Sample], output: &mut [Sample]) {
        for i in 0..input.len().min(output.len() / 2) {
            let (l, r) = self.process(input[i * 2], input[i * 2 + 1]);
            output[i * 2] = l;
            output[i * 2 + 1] = r;
        }
    }
}

/// Technical implementation of the HaasEffect structure.
pub struct HaasEffect {
    delay_time: f32,
    pan: f32,
    delay_buf: Vec<Sample>,
    write_pos: usize,
    #[allow(dead_code)]
    sample_rate: f32,
}

impl HaasEffect {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let max_delay = (sample_rate * 0.05) as usize;
        Self {
            delay_time: 30.0,
            pan: 0.0,
            delay_buf: vec![0.0; max_delay],
            write_pos: 0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_delay logic.
    pub fn set_delay(&mut self, delay_ms: f32) {
        self.delay_time = delay_ms.clamp(5.0, 50.0);
    }

    /// Technical implementation of the set_pan logic.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> (Sample, Sample) {
        let delay_samples = (self.delay_time * self.sample_rate / 1000.0) as usize;

        let read_pos = self.write_pos.saturating_sub(delay_samples);
        let delayed = self.delay_buf[read_pos % self.delay_buf.len()];

        self.delay_buf[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) % self.delay_buf.len();

        let left_gain = (1.0 - self.pan).max(0.0);
        let right_gain = (1.0 + self.pan).max(0.0);

        (
            input * left_gain + delayed * right_gain,
            input * right_gain + delayed * left_gain,
        )
    }
}

/// Technical implementation of the MonoCompat structure.
pub struct MonoCompat {
    correlation: f32,
    #[allow(dead_code)]
    sample_rate: f32,
}

impl MonoCompat {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            correlation: 0.0,
            sample_rate,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) -> f32 {
        let mid = (left + right) * 0.5;
        let side = (left - right) * 0.5;

        let mid_rms = (mid * mid).sqrt();
        let side_rms = (side * side).sqrt();

        if mid_rms > 0.0001 {
            self.correlation = self.correlation * 0.99 + (side_rms / mid_rms) * 0.01;
        }

        self.correlation
    }

    /// Technical implementation of the get_correlation logic.
    pub fn get_correlation(&self) -> f32 {
        self.correlation
    }
}

impl Default for StereoImager {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for HaasEffect {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for MonoCompat {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
