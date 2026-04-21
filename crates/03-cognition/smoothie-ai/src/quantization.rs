/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb1a92a56 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/quantization.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::math::floor_approx;

#[inline(always)]
/// Technical implementation of the fast_round logic.
fn fast_round(x: f32) -> f32 {
    if x > 0.0 {
        floor_approx(x + 0.5)
    } else {
        -floor_approx(-x + 0.5)
    }
}

pub trait Quantization {
    /// Technical implementation of the quantize logic.
    fn quantize(&self, weights: &[f32]) -> Vec<i8>;
    /// Technical implementation of the dequantize logic.
    fn dequantize(&self, quantized: &[i8], scale: f32, zero_point: i8) -> Vec<f32>;
}

/// Technical implementation of the Int8Quantizer structure.
pub struct Int8Quantizer {
    pub scale: f32,
    pub zero_point: i8,
}

impl Int8Quantizer {
    /// Initializes a new instance of the associated type.
    pub fn new(weights: &[f32]) -> Self {
        let min = weights.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = weights.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let scale = (max - min) / 255.0;
        let zero_point = fast_round(-min / scale) as i8;

        Self { scale, zero_point }
    }

    /// Technical implementation of the quantize logic.
    pub fn quantize(&self, weights: &[f32]) -> Vec<i8> {
        weights
            .iter()
            .map(|&w| {
                (fast_round(w / self.scale) as i32 + self.zero_point as i32).clamp(-128, 127) as i8
            })
            .collect()
    }

    /// Technical implementation of the dequantize logic.
    pub fn dequantize(&self, quantized: &[i8]) -> Vec<f32> {
        quantized
            .iter()
            .map(|&q| (q as i32 - self.zero_point as i32) as f32 * self.scale)
            .collect()
    }
}

/// Technical implementation of the Int4Quantizer structure.
pub struct Int4Quantizer {
    pub scale: f32,
    pub zero_point: i8,
}

impl Int4Quantizer {
    /// Initializes a new instance of the associated type.
    pub fn new(weights: &[f32]) -> Self {
        let min = weights.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = weights.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let scale = (max - min) / 15.0;
        let zero_point = fast_round(-min / scale) as i8;

        Self { scale, zero_point }
    }

    /// Technical implementation of the quantize logic.
    pub fn quantize(&self, weights: &[f32]) -> Vec<u8> {
        weights
            .iter()
            .map(|&w| {
                (fast_round(w / self.scale) as i32 + self.zero_point as i32).clamp(0, 15) as u8
            })
            .collect()
    }

    /// Technical implementation of the dequantize logic.
    pub fn dequantize(&self, quantized: &[u8]) -> Vec<f32> {
        quantized
            .iter()
            .map(|&q| (q as i32 - self.zero_point as i32) as f32 * self.scale)
            .collect()
    }
}

/// Technical implementation of the BinaryQuantizer structure.
pub struct BinaryQuantizer {
    pub threshold: f32,
}

impl BinaryQuantizer {
    /// Initializes a new instance of the associated type.
    pub fn new(weights: &[f32]) -> Self {
        let mean = weights.iter().sum::<f32>() / weights.len() as f32;
        Self { threshold: mean }
    }

    /// Technical implementation of the quantize logic.
    pub fn quantize(&self, weights: &[f32]) -> Vec<u8> {
        weights
            .iter()
            .map(|&w| (w > self.threshold) as u8)
            .collect()
    }

    /// Technical implementation of the dequantize logic.
    pub fn dequantize(&self, quantized: &[u8]) -> Vec<f32> {
        quantized
            .iter()
            .map(|&q| {
                if q == 1 {
                    self.threshold
                } else {
                    -self.threshold
                }
            })
            .collect()
    }
}

/// Technical implementation of the DynamicQuantizer structure.
pub struct DynamicQuantizer {
    pub per_channel: bool,
}

impl DynamicQuantizer {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { per_channel: false }
    }

    /// Technical implementation of the quantize_matrix logic.
    pub fn quantize_matrix(
        &self,
        weights: &[f32],
        output_channels: usize,
    ) -> (Vec<i8>, Vec<f32>, Vec<i8>) {
        let input_channels = weights.len() / output_channels;
        let mut scales = vec![0.0f32; output_channels];
        let mut zero_points = vec![0i8; output_channels];
        let mut quantized = vec![0i8; weights.len()];

        for oc in 0..output_channels {
            let mut min_val = f32::INFINITY;
            let mut max_val = f32::NEG_INFINITY;

            for ic in 0..input_channels {
                let w = weights[oc * input_channels + ic];
                if w < min_val {
                    min_val = w;
                }
                if w > max_val {
                    max_val = w;
                }
            }

            let scale = (max_val - min_val) / 255.0;
            scales[oc] = scale;
            zero_points[oc] = fast_round(-min_val / scale) as i8;

            for ic in 0..input_channels {
                let w = weights[oc * input_channels + ic];
                let q =
                    (fast_round(w / scale) as i32 + zero_points[oc] as i32).clamp(-128, 127) as i8;
                quantized[oc * input_channels + ic] = q;
            }
        }

        (quantized, scales, zero_points)
    }
}

/// Technical implementation of the QuantizedLayer structure.
pub struct QuantizedLayer {
    pub weights: Vec<i8>,
    pub biases: Vec<f32>,
    pub scales: Vec<f32>,
    pub zero_points: Vec<i8>,
    pub input_scale: f32,
    pub output_channels: usize,
    pub input_channels: usize,
}

impl QuantizedLayer {
    /// Technical implementation of the from_f32 logic.
    pub fn from_f32(weights: &[f32], biases: &[f32], input_scale: f32) -> Self {
        let output_channels = biases.len();
        let input_channels = weights.len() / output_channels;

        let mut min_vals = vec![f32::INFINITY; output_channels];
        let mut max_vals = vec![f32::NEG_INFINITY; output_channels];

        for oc in 0..output_channels {
            for ic in 0..input_channels {
                let w = weights[oc * input_channels + ic];
                if w < min_vals[oc] {
                    min_vals[oc] = w;
                }
                if w > max_vals[oc] {
                    max_vals[oc] = w;
                }
            }
        }

        let scales: Vec<f32> = max_vals
            .iter()
            .zip(min_vals.iter())
            .map(|(max, min)| (max - min) / 255.0)
            .collect();

        let zero_points: Vec<i8> = min_vals
            .iter()
            .zip(scales.iter())
            .map(|(min, scale)| fast_round(-min / scale) as i8)
            .collect();

        let mut quantized = vec![0i8; weights.len()];
        for oc in 0..output_channels {
            for ic in 0..input_channels {
                let w = weights[oc * input_channels + ic];
                let q = (fast_round(w / scales[oc]) as i32 + zero_points[oc] as i32)
                    .clamp(-128, 127) as i8;
                quantized[oc * input_channels + ic] = q;
            }
        }

        Self {
            weights: quantized,
            biases: biases.to_vec(),
            scales,
            zero_points,
            input_scale,
            output_channels,
            input_channels,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        for oc in 0..self.output_channels {
            let mut sum = self.biases[oc] as f64;

            for ic in 0..self.input_channels {
                let q = self.weights[oc * self.input_channels + ic] as i32;
                let dq = (q - self.zero_points[oc] as i32) as f32 * self.scales[oc];
                sum += input[ic] as f64 * dq as f64;
            }

            output[oc] = sum as f32;
        }
    }
}
