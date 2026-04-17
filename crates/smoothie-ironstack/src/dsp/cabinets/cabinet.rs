use crate::audio::Sample;
use std::f32::consts::PI;
use serde::{Serialize, Deserialize};

/// A physical cabinet simulation using multi-band equalization and resonance.
///
/// This module emulates the frequency response of various guitar speaker 
/// cabinets (e.g., Marshall 4x12, Fender Twin). It combines pre-set EQ curves
/// with a physical resonance model to simulate speaker 'thump' and 'air'.
pub struct Cabinet {
    sample_rate: f32,
    model: CabinetModel,
    bass: f32,
    middle: f32,
    treble: f32,
    presence: f32,
    mic_position: f32,
    
    // --- Filters ---
    low_shelf: BiquadFilter,
    mid_peak: BiquadFilter,
    high_shelf: BiquadFilter,

    // --- Resonance Model ---
    /// Delay buffer for physical resonance simulation. Expanded to 1024 for low-end.
    delays: [[f32; 1024]; 2],
    write_idx: usize,
}

/// Available speaker cabinet models.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CabinetModel {
    Marshall4x12,
    FenderTwin,
    VoxAC30,
    Mesa4x12,
    Orange4x12,
}

struct BiquadFilter {
    b0: f32, b1: f32, b2: f32, a1: f32, a2: f32,
    x1: [f32; 2], x2: [f32; 2], y1: [f32; 2], y2: [f32; 2],
}

impl BiquadFilter {
    fn new() -> Self {
        Self {
            b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0,
            x1: [0.0; 2], x2: [0.0; 2], y1: [0.0; 2], y2: [0.0; 2],
        }
    }

    fn update_low_shelf(&mut self, sr: f32, freq: f32, gain_db: f32) {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = 0.5 * (a + 1.0 / a) * (1.0 / 0.707 - 1.0) + (a - 1.0 / a) * 0.5_f32.sqrt();

        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;
        self.b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha) / a0;
        self.b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0) / a0;
        self.b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0;
        self.a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0) / a0;
        self.a2 = ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0;
    }

    fn update_high_shelf(&mut self, sr: f32, freq: f32, gain_db: f32) {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = 0.5 * (a + 1.0 / a) * (1.0 / 0.707 - 1.0) + (a - 1.0 / a) * 0.5_f32.sqrt();

        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha;
        self.b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * a.sqrt() * alpha) / a0;
        self.b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0) / a0;
        self.b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0;
        self.a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0) / a0;
        self.a2 = ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * a.sqrt() * alpha) / a0;
    }

    fn update_peak(&mut self, sr: f32, freq: f32, q: f32, gain_db: f32) {
        let w0 = 2.0 * PI * freq / sr;
        let cos_w0 = w0.cos();
        let a = 10_f32.powf(gain_db / 40.0);
        let alpha = w0.sin() / (2.0 * q);

        let a0 = 1.0 + alpha / a;
        self.b0 = (1.0 + alpha * a) / a0;
        self.b1 = (-2.0 * cos_w0) / a0;
        self.b2 = (1.0 - alpha * a) / a0;
        self.a1 = (-2.0 * cos_w0) / a0;
        self.a2 = (1.0 - alpha / a) / a0;
    }

    fn process(&mut self, sample: f32, ch: usize) -> f32 {
        let y0 = self.b0 * sample + self.b1 * self.x1[ch] + self.b2 * self.x2[ch]
            - self.a1 * self.y1[ch] - self.a2 * self.y2[ch];

        self.x2[ch] = self.x1[ch];
        self.x1[ch] = sample;
        self.y2[ch] = self.y1[ch];
        self.y1[ch] = y0;
        y0
    }
}

impl Cabinet {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            model: CabinetModel::Marshall4x12,
            bass: 0.5,
            middle: 0.5,
            treble: 0.5,
            presence: 0.5,
            mic_position: 0.5,
            low_shelf: BiquadFilter::new(),
            mid_peak: BiquadFilter::new(),
            high_shelf: BiquadFilter::new(),
            delays: [[0.0; 1024]; 2],
            write_idx: 0,
        }
    }

    pub fn set_model(&mut self, model: CabinetModel) {
        self.model = model;
    }
    pub fn set_bass(&mut self, b: f32) { self.bass = b.clamp(0.0, 1.0); }
    pub fn set_middle(&mut self, m: f32) { self.middle = m.clamp(0.0, 1.0); }
    pub fn set_treble(&mut self, t: f32) { self.treble = t.clamp(0.0, 1.0); }
    pub fn set_presence(&mut self, p: f32) { self.presence = p.clamp(0.0, 1.0); }
    pub fn set_mic_position(&mut self, m: f32) { self.mic_position = m.clamp(0.0, 1.0); }

    /// Processes audio through the cabinet model.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut out = input;

        // Model-specific characteristic gain
        let model_gain = match self.model {
            CabinetModel::Marshall4x12 => 1.2,
            CabinetModel::FenderTwin => 1.0,
            CabinetModel::VoxAC30 => 1.1,
            CabinetModel::Mesa4x12 => 1.3,
            CabinetModel::Orange4x12 => 1.15,
        };

        // Update filters based on current parameters
        self.low_shelf.update_low_shelf(self.sample_rate, 80.0, (self.bass - 0.5) * 6.0);
        self.mid_peak.update_peak(self.sample_rate, 1000.0, 1.5, (self.middle - 0.5) * 6.0);
        self.high_shelf.update_high_shelf(self.sample_rate, 6000.0, (self.treble - 0.5) * 6.0);

        // Apply filters per channel
        out.left = self.low_shelf.process(out.left, 0);
        out.right = self.low_shelf.process(out.right, 1);
        out.left = self.mid_peak.process(out.left, 0);
        out.right = self.mid_peak.process(out.right, 1);
        out.left = self.high_shelf.process(out.left, 0);
        out.right = self.high_shelf.process(out.right, 1);

        // Apply cabinet resonance ('thump')
        out.left = self.cabinet_resonance(out.left, 0);
        out.right = self.cabinet_resonance(out.right, 1);
        
        // Update circular buffer index
        self.write_idx = (self.write_idx + 1) % 1024;

        out.left *= model_gain;
        out.right *= model_gain;

        out.clip()
    }

    /// Simulates the physical reflections and air movement inside the cabinet.
    fn cabinet_resonance(&mut self, input: f32, ch: usize) -> f32 {
        let resonance_freq = 80.0 + self.bass * 40.0;
        let r = 0.5 + self.presence * 0.2; // Feedback amount

        // Calculate delay in samples for the target resonance frequency
        let delay_samps = (self.sample_rate / resonance_freq) as usize;
        let read_idx = if self.write_idx >= delay_samps {
            self.write_idx - delay_samps
        } else {
            1024 + self.write_idx - delay_samps
        };

        let delayed = self.delays[ch][read_idx % 1024];
        let res_signal = input + delayed * r;
        
        self.delays[ch][self.write_idx] = res_signal;

        // Combine dry signal with resonant 'thump'
        input * 0.7 + res_signal * 0.3
    }

    /// Resets all internal delay lines and filter states.
    pub fn reset(&mut self) {
        self.delays = [[0.0; 1024]; 2];
        self.low_shelf = BiquadFilter::new();
        self.mid_peak = BiquadFilter::new();
        self.high_shelf = BiquadFilter::new();
        self.write_idx = 0;
    }
}
