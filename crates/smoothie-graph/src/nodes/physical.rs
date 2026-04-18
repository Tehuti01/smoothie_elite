//! 'Elite' Physical Modeling Node for acoustic resynthesis.

use crate::ModularNode;
use smoothie_dsp::physical::{WaveguideString, ModalResonator};
use smoothie_params::Param;
use dasp_graph::{Buffer, Node};
use std::sync::Arc;

/// A modular node for waveguide and modal synthesis.
pub struct PhysicalNode {
    string: WaveguideString,
    resonator: ModalResonator,
    blend: f64,
}

impl PhysicalNode {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            string: WaveguideString::new(sample_rate, 4096),
            resonator: ModalResonator::new(sample_rate, &[440.0, 880.0, 1320.0], &[0.5, 0.2, 0.1]),
            blend: 0.0,
        }
    }
}

impl ModularNode for PhysicalNode {
    fn process(&mut self, inputs: &[&Buffer], outputs: &mut [Buffer], _sample_rate: f64) {
        if outputs.is_empty() { return; }
        
        // Physical modeling can be an effect or a source.
        // If an input exists, it excites the resonator.
        let excitation = if !inputs.is_empty() { inputs[0][0] } else { 0.0 };
        
        let out = self.string.next_sample(440.0) + self.resonator.process(excitation);
        
        for sample in outputs[0].iter_mut() {
            *sample = out; // Simple mono physical for now
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
