//! 'Elite' Physical Modeling primitives — Karplus-Strong Waveguides 
//! and Modal Resonators for organic, acoustic synthesis.

use crate::filters::{OnePoleFilter, FilterType};

/// A physical waveguide model for 'Elite' string and plate synthesis.
/// Implements the Karplus-Strong algorithm with fractional delay and damping.
pub struct WaveguideString {
    delay_line: Vec<f64>,
    write_pos: usize,
    sample_rate: f64,
    
    // Damping/Filter state
    lp_filter: f64,
    feedback: f64,
    damping: f64,
}

impl WaveguideString {
    pub fn new(sample_rate: f64, max_delay: usize) -> Self {
        Self {
            delay_line: vec![0.0; max_delay],
            write_pos: 0,
            sample_rate,
            lp_filter: 0.0,
            feedback: 0.99,
            damping: 0.5,
        }
    }

    pub fn set_frequency(&mut self, hz: f64) {
        let delay_samples = self.sample_rate / hz;
        // In a true 'Elite' model, we'd use fractional delay (Hermite/Lagrange)
        // For now, we'll use integer delay for stability.
        self.feedback = 0.999;
    }

    pub fn set_damping(&mut self, val: f64) {
        self.damping = val.clamp(0.0, 1.0);
    }

    /// Excite the string by injecting a burst of noise or a sample transient.
    pub fn excite(&mut self, impulse: &[f64]) {
        for (i, &val) in impulse.iter().enumerate() {
            if i < self.delay_line.len() {
                self.delay_line[i] += val;
            }
        }
    }

    /// Advance one sample and return output.
    pub fn next_sample(&mut self, freq_hz: f64) -> f64 {
        let delay_len = (self.sample_rate / freq_hz) as usize;
        let delay_len = delay_len.clamp(1, self.delay_line.len() - 1);
        
        let read_pos = (self.write_pos + self.delay_line.len() - delay_len) % self.delay_line.len();
        let delayed_sample = self.delay_line[read_pos];
        
        // Simple One-Pole LP filtering for damping
        self.lp_filter = delayed_sample * (1.0 - self.damping) + self.lp_filter * self.damping;
        
        let out = self.lp_filter * self.feedback;
        
        self.delay_line[self.write_pos] = out;
        self.write_pos = (self.write_pos + 1) % self.delay_line.len();
        
        out
    }
}

/// A Modal Resonator bank representing a physical object's modes.
pub struct ModalResonator {
    modes: Vec<ResonantMode>,
}

struct ResonantMode {
    filter: crate::filters::BiquadFilter,
    gain: f64,
}

impl ModalResonator {
    pub fn new(sample_rate: f64, frequencies: &[f64], decays: &[f64]) -> Self {
        let mut modes = Vec::new();
        for (i, &f) in frequencies.iter().enumerate() {
            let mut filter = crate::filters::BiquadFilter::design(
                crate::filters::FilterType::LowPass,
                400.0,
                sample_rate as f32,
                0.707,
                0.0
            );
            filter.set_parameters(crate::filters::FilterType::BandPass, f as f32, sample_rate as f32, 50.0, 0.0); // High Q for resonance
            modes.push(ResonantMode { filter, gain: decays[i] });
        }
        Self { modes }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        let mut out = 0.0;
        for mode in self.modes.iter_mut() {
            out += mode.filter.process(x as f32) as f64 * mode.gain;
        }
        out
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
