/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xda278071 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/dense.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::math::FloatExt;
use wide::*;

/// Technical implementation of the DenseLayer structure.
pub struct DenseLayer {
    pub weights: Vec<f32>, // Flat [out_dim, in_dim]
    pub bias: Vec<f32>,
    pub inputs: usize,
    pub outputs: usize,
}

impl DenseLayer {
    /// Initializes a new instance of the associated type.
    pub fn new(in_dim: usize, out_dim: usize) -> Self {
        Self {
            weights: alloc::vec![0.1; in_dim * out_dim],
            bias: alloc::vec![0.0; out_dim],
            inputs: in_dim,
            outputs: out_dim,
        }
    }

    /// Technical implementation of the forward logic with SIMD optimization.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), self.inputs);
        assert_eq!(output.len(), self.outputs);

        for i in 0..self.outputs {
            let mut sum_simd = f32x4::ZERO;
            let weight_row = &self.weights[i * self.inputs..(i + 1) * self.inputs];

            // SIMD Vectorization
            let mut j = 0;
            while j + 4 <= self.inputs {
                let w = f32x4::from(&weight_row[j..j + 4]);
                let inp = f32x4::from(&input[j..j + 4]);
                sum_simd += w * inp;
                j += 4;
            }

            // Scalar remainder
            let mut sum = sum_simd.reduce_add();
            while j < self.inputs {
                sum += weight_row[j] * input[j];
                j += 1;
            }

            output[i] = sum + self.bias[i];
        }
    }
}
