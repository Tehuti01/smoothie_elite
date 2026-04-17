//! Algorithmic reverb processor.

use crate::EffectProcessor;

const COMB_FILTER_TUNINGS: &[usize] = &[1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
const ALLPASS_FILTER_TUNINGS: &[usize] = &[556, 441, 341, 225];
const MUTED: f32 = 0.0;
const FIXED_GAIN: f32 = 0.015;
const SCALE_WET: f32 = 3.0;
const SCALE_DRY: f32 = 2.0;
const SCALE_DAMP: f32 = 0.4;
const SCALE_ROOM: f32 = 0.28;
const STEREO_SPREAD: usize = 23;

/// Comb filter for reverb.
struct CombFilter {
    buffer: Vec<f32>,
    buffer_index: usize,
    filter_store: f32,
    damp1: f32,
    damp2: f32,
    feedback: f32,
}

impl CombFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            buffer_index: 0,
            filter_store: MUTED,
            damp1: 0.5,
            damp2: 0.5,
            feedback: 0.5,
        }
    }

    fn set_damp(&mut self, value: f32) {
        self.damp1 = value * SCALE_DAMP;
        self.damp2 = (1.0 - value) * SCALE_DAMP;
    }

    fn set_feedback(&mut self, value: f32) {
        self.feedback = value * SCALE_ROOM;
    }

    fn process(&mut self, input: f32) -> f32 {
        let buffer_out = self.buffer[self.buffer_index];
        self.filter_store = buffer_out * self.damp2 + self.filter_store * self.damp1;
        self.buffer[self.buffer_index] = input + self.filter_store * self.feedback;
        self.buffer_index = (self.buffer_index + 1) % self.buffer.len();
        buffer_out
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.filter_store = 0.0;
        self.buffer_index = 0;
    }
}

/// Allpass filter for reverb.
struct AllpassFilter {
    buffer: Vec<f32>,
    buffer_index: usize,
    feedback: f32,
}

impl AllpassFilter {
    fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            buffer_index: 0,
            feedback: 0.5,
        }
    }

    fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    fn process(&mut self, input: f32) -> f32 {
        let buffer_out = self.buffer[self.buffer_index];
        let output = -input + buffer_out;
        self.buffer[self.buffer_index] = input + buffer_out * self.feedback;
        self.buffer_index = (self.buffer_index + 1) % self.buffer.len();
        output
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.buffer_index = 0;
    }
}

/// Algorithmic reverb processor (Freeverb-style).
///
/// Uses parallel comb filters + series allpass for high-quality reverb.
pub struct ReverbProcessor {
    comb_filters: Vec<CombFilter>,
    allpass_filters: Vec<AllpassFilter>,
    wet1: f32,
    wet2: f32,
    dry: f32,
    width: f32,
    room_size: f32,
    damp: f32,
    sample_rate: f32,
}

impl ReverbProcessor {
    /// Create a new reverb processor.
    pub fn new(sample_rate: f32) -> Self {
        let mut comb_filters = Vec::new();
        let mut allpass_filters = Vec::new();

        for &size in COMB_FILTER_TUNINGS {
            comb_filters.push(CombFilter::new(size));
        }

        for &size in ALLPASS_FILTER_TUNINGS {
            allpass_filters.push(AllpassFilter::new(size));
        }

        let mut reverb = Self {
            comb_filters,
            allpass_filters,
            wet1: 0.0,
            wet2: 0.0,
            dry: 0.0,
            width: 1.0,
            room_size: 0.5,
            damp: 0.5,
            sample_rate,
        };
        reverb.update_gains();
        reverb
    }

    fn update_gains(&mut self) {
        self.wet1 = self.room_size * SCALE_WET;
        self.wet2 = (1.0 - self.room_size) * SCALE_WET;
        self.dry = SCALE_DRY;

        for filter in &mut self.comb_filters {
            filter.set_feedback(self.room_size);
            filter.set_damp(self.damp);
        }

        for filter in &mut self.allpass_filters {
            filter.set_feedback(0.5);
        }
    }

    /// Set room size (0.0–1.0).
    pub fn set_room_size(&mut self, size: f32) {
        self.room_size = size.clamp(0.0, 1.0);
        self.update_gains();
    }

    /// Set damping (0.0–1.0).
    pub fn set_damp(&mut self, damp: f32) {
        self.damp = damp.clamp(0.0, 1.0);
        self.update_gains();
    }

    /// Set stereo width (0.0–1.0).
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(0.0, 1.0);
    }
}

impl EffectProcessor for ReverbProcessor {
    fn process(&mut self, input: f32) -> f32 {
        let mut out_l = 0.0;
        let mut out_r = 0.0;

        // Parallel comb filters
        for (i, filter) in self.comb_filters.iter_mut().enumerate() {
            let out = filter.process(input) * self.wet1;
            if i < 4 {
                out_l += out;
            } else {
                out_r += out;
            }
        }

        // Series allpass filters (same for both channels)
        let mut mid = out_l + out_r;
        for filter in &mut self.allpass_filters {
            mid = filter.process(mid);
        }

        // Stereo separation
        let out_final = mid * (1.0 - self.width) + (out_l - out_r) * self.width * 0.5;
        input * self.dry + out_final
    }

    fn reset(&mut self) {
        for filter in &mut self.comb_filters {
            filter.reset();
        }
        for filter in &mut self.allpass_filters {
            filter.reset();
        }
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }
}

impl Default for ReverbProcessor {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
