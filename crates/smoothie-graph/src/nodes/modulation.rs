//! 'Elite' Neural Modulation Node for the modular ecosystem.

use crate::ModularNode;
use smoothie_dsp::modulation::NeuralLfo;
use smoothie_params::Param;
use dasp_graph::{Buffer, Node};
use std::sync::Arc;

/// A modular node containing chaotic neural modulators.
pub struct NeuralModNode {
    lfo1: NeuralLfo,
    lfo2: NeuralLfo,
}

impl NeuralModNode {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            lfo1: NeuralLfo::new(sample_rate, 16),
            lfo2: NeuralLfo::new(sample_rate, 32),
        }
    }
}

impl ModularNode for NeuralModNode {
    fn process(&mut self, _inputs: &[&Buffer], outputs: &mut [Buffer], _sample_rate: f64) {
        if outputs.is_empty() { return; }
        
        let out1 = self.lfo1.next_sample();
        let out2 = self.lfo2.next_sample();
        
        // This node outputs its chaotic signals for other nodes to consume
        for sample in outputs[0].iter_mut() {
            *sample = out1; 
        }
        if outputs.len() > 1 {
            for sample in outputs[1].iter_mut() {
                *sample = out2;
            }
        }
    }

    fn parameters(&self) -> Vec<Arc<dyn Param>> {
        vec![] // To be implemented
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
