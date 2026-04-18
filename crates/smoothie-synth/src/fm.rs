//! FM (Frequency Modulation) synthesis engine.

use std::f32::consts::PI;

/// FM operator (oscillator with modulation inputs).
pub struct FmOperator {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    envelope_level: f32,
    feedback: f32,
    feedback_phase: f32,
}

impl FmOperator {
    /// Create a new FM operator.
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate,
            envelope_level: 1.0,
            feedback: 0.0,
            feedback_phase: 0.0,
        }
    }

    /// Process one sample with modulation input.
    pub fn process(&mut self, modulation: f32) -> f32 {
        // Add modulation to phase (frequency modulation)
        let modulated_phase = (self.phase + modulation) % 1.0;

        // Add feedback
        let feedback_contribution = self.feedback * self.feedback_phase;
        let final_phase = (modulated_phase + feedback_contribution) % 1.0;

        // Generate sine output
        let output = (2.0 * PI * final_phase).sin() * self.envelope_level;

        // Store feedback phase for next iteration
        self.feedback_phase = final_phase;

        // Advance phase for next sample
        let phase_inc = self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_inc) % 1.0;

        output
    }

    /// Set operator frequency in Hz.
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    /// Set envelope level (amplitude).
    pub fn set_envelope(&mut self, level: f32) {
        self.envelope_level = level.clamp(0.0, 1.0);
    }

    /// Set feedback amount (0.0–1.0).
    pub fn set_feedback(&mut self, amount: f32) {
        self.feedback = amount.clamp(0.0, 1.0);
    }

    /// Reset operator state.
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.feedback_phase = 0.0;
    }
}

/// FM synthesis algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FmAlgorithm {
    /// Carrier modulated by modulator
    Simple,
    /// Two modulators, one carrier
    Stack,
    /// Four operators with complex routing
    Complex,
}

/// FM synthesizer with 4 operators.
pub struct FmSynth {
    ops: [FmOperator; 4],
    algorithm: FmAlgorithm,
    sample_rate: f32,
    feedback_amount: f32,
}

impl FmSynth {
    /// Create a new FM synthesizer.
    pub fn new(sample_rate: f32) -> Self {
        let ops = [
            FmOperator::new(sample_rate, 440.0),
            FmOperator::new(sample_rate, 660.0),
            FmOperator::new(sample_rate, 880.0),
            FmOperator::new(sample_rate, 1320.0),
        ];

        Self {
            ops,
            algorithm: FmAlgorithm::Simple,
            sample_rate,
            feedback_amount: 0.0,
        }
    }

    /// Process one sample.
    pub fn process(&mut self) -> f32 {
        match self.algorithm {
            FmAlgorithm::Simple => {
                // Op1 (modulator) → Op0 (carrier)
                let mod1 = self.ops[1].process(0.0);
                self.ops[0].process(mod1)
            }
            FmAlgorithm::Stack => {
                // Op1 + Op2 (modulators) → Op0 (carrier)
                let mod1 = self.ops[1].process(0.0);
                let mod2 = self.ops[2].process(0.0);
                self.ops[0].process(mod1 + mod2)
            }
            FmAlgorithm::Complex => {
                // Complex 4-operator routing
                let mod3 = self.ops[3].process(0.0);
                let mod2 = self.ops[2].process(mod3);
                let mod1 = self.ops[1].process(mod2);
                self.ops[0].process(mod1)
            }
        }
    }

    /// Set algorithm.
    pub fn set_algorithm(&mut self, algo: FmAlgorithm) {
        self.algorithm = algo;
    }

    /// Set operator frequency (frequency ratio to base).
    pub fn set_op_ratio(&mut self, op_idx: usize, ratio: f32, base_freq: f32) {
        if op_idx < 4 {
            self.ops[op_idx].set_frequency(base_freq * ratio);
        }
    }

    /// Set operator envelope level.
    pub fn set_op_level(&mut self, op_idx: usize, level: f32) {
        if op_idx < 4 {
            self.ops[op_idx].set_envelope(level);
        }
    }

    /// Set feedback amount (0.0–1.0).
    pub fn set_feedback(&mut self, amount: f32) {
        self.feedback_amount = amount.clamp(0.0, 1.0);
        if !self.ops.is_empty() {
            self.ops[0].set_feedback(amount);
        }
    }

    /// Reset all operators.
    pub fn reset(&mut self) {
        for op in self.ops.iter_mut() {
            op.reset();
        }
    }

    /// Set sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        for op in self.ops.iter_mut() {
            op.sample_rate = sr;
        }
    }
}

impl Default for FmSynth {
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
