/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6a9be14e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/normalization.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::math::sqrt_approx;
// use smoothie_core::math::FloatExt;

/// Technical implementation of the LayerNorm structure.
pub struct LayerNorm {
    pub epsilon: f32,
    pub gamma: Vec<f32>,
    pub beta: Vec<f32>,
}

impl LayerNorm {
    /// Initializes a new instance of the associated type.
    pub fn new(size: usize) -> Self {
        Self {
            epsilon: 1e-5,
            gamma: vec![1.0; size],
            beta: vec![0.0; size],
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        let size = input.len();
        if size == 0 {
            return;
        }

        let mut sum = 0.0f32;
        for i in 0..size {
            sum += input[i];
        }
        let mean = sum / size as f32;

        let mut variance = 0.0f32;
        for i in 0..size {
            let diff = input[i] - mean;
            variance += diff * diff;
        }
        variance /= size as f32;

        let inv_std = 1.0 / sqrt_approx(variance + self.epsilon);

        for i in 0..size.min(output.len()) {
            output[i] = (input[i] - mean) * inv_std * self.gamma[i] + self.beta[i];
        }
    }
}

/// Technical implementation of the BatchNorm1D structure.
pub struct BatchNorm1D {
    pub epsilon: f32,
    pub momentum: f32,
    pub gamma: Vec<f32>,
    pub beta: Vec<f32>,
    running_mean: Vec<f32>,
    running_var: Vec<f32>,
    pub training: bool,
}

impl BatchNorm1D {
    /// Initializes a new instance of the associated type.
    pub fn new(num_features: usize) -> Self {
        Self {
            epsilon: 1e-5,
            momentum: 0.1,
            gamma: vec![1.0; num_features],
            beta: vec![0.0; num_features],
            running_mean: vec![0.0; num_features],
            running_var: vec![1.0; num_features],
            training: true,
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&mut self, input: &[f32], output: &mut [f32]) {
        let batch_size = input.len() / self.gamma.len();
        if batch_size == 0 {
            return;
        }

        if self.training {
            let mut mean = 0.0f32;
            for i in 0..input.len() {
                mean += input[i];
            }
            mean /= input.len() as f32;

            let mut variance = 0.0f32;
            for i in 0..input.len() {
                let diff = input[i] - mean;
                variance += diff * diff;
            }
            variance /= input.len() as f32;

            for i in 0..self.running_mean.len() {
                self.running_mean[i] =
                    self.momentum * mean + (1.0 - self.momentum) * self.running_mean[i];
                self.running_var[i] =
                    self.momentum * variance + (1.0 - self.momentum) * self.running_var[i];
            }

            let inv_std = 1.0 / sqrt_approx(variance + self.epsilon);
            for i in 0..input.len().min(output.len()) {
                let idx = i % self.gamma.len();
                output[i] = (input[i] - mean) * inv_std * self.gamma[idx] + self.beta[idx];
            }
        } else {
            let inv_std = 1.0 / sqrt_approx(self.running_var[0] + self.epsilon);
            for i in 0..input.len().min(output.len()) {
                let idx = i % self.gamma.len();
                output[i] = (input[i] - self.running_mean[idx]) * inv_std * self.gamma[idx]
                    + self.beta[idx];
            }
        }
    }
}

/// Technical implementation of the GroupNorm structure.
pub struct GroupNorm {
    pub num_groups: usize,
    pub num_channels: usize,
    pub epsilon: f32,
    pub gamma: Vec<f32>,
    pub beta: Vec<f32>,
}

impl GroupNorm {
    /// Initializes a new instance of the associated type.
    pub fn new(num_groups: usize, num_channels: usize) -> Self {
        let _channels_per_group = num_channels / num_groups;
        Self {
            num_groups,
            num_channels,
            epsilon: 1e-5,
            gamma: vec![1.0; num_channels],
            beta: vec![0.0; num_channels],
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        let channels_per_group = self.num_channels / self.num_groups;
        let group_size = channels_per_group;

        for g in 0..self.num_groups {
            let mut sum = 0.0f32;
            for c in 0..group_size {
                let idx = g * group_size + c;
                if idx < input.len() {
                    sum += input[idx];
                }
            }
            let mean = sum / group_size as f32;

            let mut variance = 0.0f32;
            for c in 0..group_size {
                let idx = g * group_size + c;
                if idx < input.len() {
                    let diff = input[idx] - mean;
                    variance += diff * diff;
                }
            }
            variance /= group_size as f32;
            let inv_std = 1.0 / sqrt_approx(variance + self.epsilon);

            for c in 0..group_size {
                let idx = g * group_size + c;
                if idx < input.len().min(output.len()) {
                    output[idx] = (input[idx] - mean) * inv_std * self.gamma[idx] + self.beta[idx];
                }
            }
        }
    }
}

/// Technical implementation of the InstanceNorm1D structure.
pub struct InstanceNorm1D {
    pub epsilon: f32,
    pub gamma: Vec<f32>,
    pub beta: Vec<f32>,
}

impl InstanceNorm1D {
    /// Initializes a new instance of the associated type.
    pub fn new(num_channels: usize) -> Self {
        Self {
            epsilon: 1e-5,
            gamma: vec![1.0; num_channels],
            beta: vec![0.0; num_channels],
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&self, input: &[f32], output: &mut [f32]) {
        for c in 0..self.gamma.len() {
            let mut sum = 0.0f32;
            let mut count = 0usize;
            for i in (c..input.len()).step_by(self.gamma.len()) {
                sum += input[i];
                count += 1;
            }
            let mean = sum / count as f32;

            let mut variance = 0.0f32;
            for i in (c..input.len()).step_by(self.gamma.len()) {
                let diff = input[i] - mean;
                variance += diff * diff;
            }
            variance /= count as f32;
            let inv_std = 1.0 / sqrt_approx(variance + self.epsilon);

            for i in (c..input.len().min(output.len())).step_by(self.gamma.len()) {
                output[i] = (input[i] - mean) * inv_std * self.gamma[c] + self.beta[c];
            }
        }
    }
}
