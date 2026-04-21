/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x31343098 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/conv1d.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::activations::Activation;
use alloc::vec;
use alloc::vec::Vec;

/// Technical implementation of the Conv1DLayer structure.
pub struct Conv1DLayer {
    weights: Vec<f32>, // [out_channels, in_channels, kernel_size]
    biases: Vec<f32>,
    pub in_channels: usize,
    pub out_channels: usize,
    pub kernel_size: usize,
    pub dilation: usize,
    pub activation: Activation,
}

impl Conv1DLayer {
    /// Initializes a new instance of the associated type.
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: usize,
        dilation: usize,
        activation: Activation,
    ) -> Self {
        Self {
            weights: vec![0.0; out_channels * in_channels * kernel_size],
            biases: vec![0.0; out_channels],
            in_channels,
            out_channels,
            kernel_size,
            dilation,
            activation,
        }
    }

    /// Technical implementation of the load_weights logic.
    pub fn load_weights(&mut self, w: &[f32], b: &[f32]) -> Result<(), &'static str> {
        if w.len() != self.weights.len() || b.len() != self.biases.len() {
            return Err("Conv1D weights mismatch");
        }
        self.weights.copy_from_slice(w);
        self.biases.copy_from_slice(b);
        Ok(())
    }

    /// `history_buffer` holds `in_channels * (kernel_size * dilation)` past samples.
    pub fn forward_step(&self, history_buffer: &[f32], current_idx: usize, output: &mut [f32]) {
        let history_len = history_buffer.len() / self.in_channels;

        for out_c in 0..self.out_channels {
            let mut sum = self.biases[out_c];

            for in_c in 0..self.in_channels {
                for k in 0..self.kernel_size {
                    // Calculate index in dilated history (naive circular index calculation)
                    let hist_idx = (current_idx + history_len - (k * self.dilation)) % history_len;
                    let val = history_buffer[in_c * history_len + hist_idx];

                    let w_idx = (out_c * self.in_channels + in_c) * self.kernel_size + k;
                    sum += val * self.weights[w_idx];
                }
            }
            output[out_c] = self.activation.apply(sum);
        }
    }
}
