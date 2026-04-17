use std::collections::VecDeque;

/// A noise gate for suppressing low-level signal noise during silence.
///
/// This implementation uses a look-ahead-style level detection and a 
/// hold/release mechanism to prevent 'chatter' or premature cutting of notes.
/// It features stereo linking to maintain imaging consistency.
pub struct NoiseGate {
    sample_rate: f32,
    /// Level at which the gate opens (dB).
    threshold: f32,
    attack_ms: f32,
    hold_ms: f32,
    release_ms: f32,
    /// Circular buffer for RMS level detection sidechain.
    filter_state: VecDeque<f32>,
    filter_size: usize,
    hold_counter: usize,
    hold_samples: usize,
    /// Whether the gate is currently open.
    is_open: bool,
}

impl NoiseGate {
    /// Creates a new NoiseGate.
    pub fn new(sample_rate: f64) -> Self {
        let sr = sample_rate as f32;
        let filter_size = (sr * 0.005).max(1.0) as usize; // 5ms window
        Self {
            sample_rate: sr,
            threshold: -50.0,
            attack_ms: 0.5,
            hold_ms: 30.0,
            release_ms: 100.0,
            filter_state: VecDeque::with_capacity(filter_size),
            filter_size,
            hold_counter: 0,
            hold_samples: (30.0 * sr / 1000.0) as usize,
            is_open: false,
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold = threshold_db.clamp(-80.0, 0.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.clamp(0.1, 50.0);
    }

    pub fn set_hold(&mut self, hold_ms: f32) {
        self.hold_ms = hold_ms.clamp(0.0, 500.0);
        self.hold_samples = (self.hold_ms * self.sample_rate / 1000.0) as usize;
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.clamp(1.0, 1000.0);
    }

    /// Calculates the current RMS level of the sidechain buffer.
    fn get_rms_level(&self) -> f32 {
        if self.filter_state.is_empty() {
            return -120.0;
        }
        let sum: f32 = self.filter_state.iter().map(|&x| x * x).sum();
        let rms = (sum / self.filter_state.len() as f32).sqrt();
        if rms > 1e-6 {
            20.0 * rms.log10()
        } else {
            -120.0
        }
    }

    /// Internal method to update the gate state based on the sidechain level.
    fn update_gate_state(&mut self, level_db: f32) {
        if level_db > self.threshold {
            self.hold_counter = self.hold_samples;
            self.is_open = true;
        } else if self.hold_counter > 0 {
            self.hold_counter -= 1;
            self.is_open = true;
        } else {
            self.is_open = false;
        }
    }

    /// Processes a single mono sample.
    pub fn process(&mut self, input: f32) -> f32 {
        self.filter_state.push_back(input.abs());
        if self.filter_state.len() > self.filter_size {
            self.filter_state.pop_front();
        }

        let level_db = self.get_rms_level();
        self.update_gate_state(level_db);

        if self.is_open { input } else { 0.0 }
    }

    /// Processes stereo samples with linked gating.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        // Detect sidechain level from both channels (max peak)
        let mono_sum = (left.abs() + right.abs()) * 0.5;
        self.filter_state.push_back(mono_sum);
        if self.filter_state.len() > self.filter_size {
            self.filter_state.pop_front();
        }

        let level_db = self.get_rms_level();
        self.update_gate_state(level_db);

        if self.is_open {
            (left, right)
        } else {
            (0.0, 0.0)
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn clear(&mut self) {
        self.filter_state.clear();
        self.hold_counter = 0;
        self.is_open = false;
    }
}

impl Default for NoiseGate {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
