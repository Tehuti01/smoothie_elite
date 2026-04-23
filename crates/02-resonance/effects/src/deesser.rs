/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x80c7299c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/deesser.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::{amplitude_to_db, db_to_amplitude, exp_approx};
use smoothie_core::primitives::Sample;

/// Technical implementation of the DeEsser structure.
pub struct DeEsser {
    threshold: f32,
    frequency: f32,
    q: f32,
    range: f32,
    mode: DeesserMode,
    envelope: f32,
    detection_buffer: [f32; 512],
    buffer_pos: usize,
    sample_rate: f32,
    filter_state: [f32; 4],
}

#[derive(Clone, Copy, Default)]
/// Technical implementation of the DeesserMode enumeration.
pub enum DeesserMode {
    #[default]
    Wide,
    Narrow,
    Band,
}

impl DeEsser {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            threshold: -18.0,
            frequency: 7500.0,
            q: 2.0,
            range: -15.0,
            mode: DeesserMode::Wide,
            envelope: -60.0,
            detection_buffer: [0.0; 512],
            buffer_pos: 0,
            sample_rate,
            filter_state: [0.0; 4],
        }
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = threshold_db.clamp(-40.0, 0.0);
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq.max(2000.0).min(12000.0);
        self.update_filter();
    }

    /// Technical implementation of the set_q logic.
    pub fn set_q(&mut self, q: f32) {
        self.q = q.clamp(0.5, 10.0);
        self.update_filter();
    }

    /// Technical implementation of the set_range logic.
    pub fn set_range(&mut self, range_db: f32) {
        self.range = range_db.clamp(-30.0, 0.0);
    }

    /// Technical implementation of the set_mode logic.
    pub fn set_mode(&mut self, mode: DeesserMode) {
        self.mode = mode;
    }

    /// Technical implementation of the update_filter logic.
    fn update_filter(&mut self) {
        // Bandpass filter coefficients would be computed here
    }

    /// Technical implementation of the detect_sibilance logic.
    fn detect_sibilance(&mut self, input: f32) -> f32 {
        // Simple high-frequency detection
        let alpha = exp_approx(-2.0 * core::f32::consts::PI * self.frequency / self.sample_rate);
        let hp = input - self.filter_state[0];
        self.filter_state[0] += alpha * hp;

        let detected = hp.abs();

        // Store in buffer for FFT analysis
        self.detection_buffer[self.buffer_pos] = detected;
        self.buffer_pos = (self.buffer_pos + 1) % 512;

        // Envelope follower
        if detected > self.envelope {
            self.envelope += 0.1 * (detected - self.envelope);
        } else {
            self.envelope += 0.001 * (detected - self.envelope);
        }

        self.envelope
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let sibilance_level = self.detect_sibilance(input);
        let sibilance_db = amplitude_to_db(sibilance_level);

        let mut gain_reduction = 0.0;

        if sibilance_db > self.threshold {
            let over_threshold = sibilance_db - self.threshold;

            // Soft knee
            let knee_width = match self.mode {
                DeesserMode::Wide => 10.0,
                DeesserMode::Narrow => 3.0,
                DeesserMode::Band => 5.0,
            };

            if over_threshold < knee_width {
                gain_reduction = (over_threshold * over_threshold) / (2.0 * knee_width);
            } else {
                gain_reduction = over_threshold - knee_width / 2.0;
            }

            gain_reduction = gain_reduction.max(self.range.abs());
        }

        let gain = db_to_amplitude(-gain_reduction);
        input * gain
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        // Detect from sum for proper stereo de-essing
        let sum = (left + right) * 0.5;
        let sibilance_level = self.detect_sibilance(sum);

        let sibilance_db = amplitude_to_db(sibilance_level);
        let mut gain_reduction = 0.0;

        if sibilance_db > self.threshold {
            let over = sibilance_db - self.threshold;
            gain_reduction = over.max(self.range.abs());
        }

        let gain = db_to_amplitude(-gain_reduction);

        (left * gain, right * gain)
    }

    /// Technical implementation of the get_gain_reduction logic.
    pub fn get_gain_reduction(&self) -> f32 {
        -self.envelope + self.threshold
    }
}

impl Default for DeEsser {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
