/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x61877ac6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/model.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::dense::DenseLayer;
use alloc::vec;
use alloc::vec::Vec;

/// Technical implementation of the NeuralModel structure.
pub struct NeuralModel {
    pub layers: Vec<DenseLayer>,
}

impl NeuralModel {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
    /// Performs vector addition logic.
    pub fn add_layer(&mut self, in_dim: usize, out_dim: usize) {
        self.layers.push(DenseLayer::new(in_dim, out_dim));
    }
    /// Technical implementation of the forward logic.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        let mut h = input.to_vec();
        for layer in &self.layers {
            let mut next = vec![0.0; layer.outputs];
            layer.forward(&h, &mut next);
            h = next;
        }
        for (i, &val) in h.iter().enumerate().take(output.len()) {
            output[i] = val;
        }
    }
}
