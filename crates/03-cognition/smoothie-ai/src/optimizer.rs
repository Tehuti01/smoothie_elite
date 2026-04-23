/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x560c30ce | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/optimizer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::math::{fast_pow, floor_approx, sqrt_approx};

#[inline(always)]
/// Technical implementation of the fast_round logic.
fn _fast_round(x: f32) -> f32 {
    if x > 0.0 {
        floor_approx(x + 0.5)
    } else {
        -floor_approx(-x + 0.5)
    }
}

pub trait Optimizer {
    /// Technical implementation of the step logic.
    fn step(&mut self, weights: &mut [f32], gradients: &[f32]);
    /// Technical implementation of the learning_rate logic.
    fn learning_rate(&self) -> f32;
    /// Technical implementation of the set_learning_rate logic.
    fn set_learning_rate(&mut self, lr: f32);
}

/// Technical implementation of the SGD structure.
pub struct SGD {
    pub lr: f32,
    pub momentum: f32,
    pub damping: f32,
    velocity: Vec<f32>,
}

impl SGD {
    /// Initializes a new instance of the associated type.
    pub fn new(lr: f32, momentum: f32, size: usize) -> Self {
        Self {
            lr,
            momentum,
            damping: 1.0,
            velocity: vec![0.0; size],
        }
    }
}

impl Optimizer for SGD {
    #[inline]
    /// Technical implementation of the step logic.
    fn step(&mut self, weights: &mut [f32], gradients: &[f32]) {
        for i in 0..weights.len() {
            self.velocity[i] = self.momentum * self.velocity[i] - self.lr * gradients[i];
            weights[i] += self.velocity[i] * self.damping;
        }
    }

    /// Technical implementation of the learning_rate logic.
    fn learning_rate(&self) -> f32 {
        self.lr
    }
    /// Technical implementation of the set_learning_rate logic.
    fn set_learning_rate(&mut self, lr: f32) {
        self.lr = lr;
    }
}

/// Technical implementation of the Adam structure.
pub struct Adam {
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub epsilon: f32,
    pub weight_decay: f32,
    m: Vec<f32>,
    v: Vec<f32>,
    t: u32,
}

impl Adam {
    /// Initializes a new instance of the associated type.
    pub fn new(lr: f32, size: usize) -> Self {
        Self {
            lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.0,
            m: vec![0.0; size],
            v: vec![0.0; size],
            t: 0,
        }
    }
}

impl Optimizer for Adam {
    /// Technical implementation of the step logic.
    fn step(&mut self, weights: &mut [f32], gradients: &[f32]) {
        self.t += 1;
        let t_f = self.t as f32;
        let beta2_t = fast_pow(self.beta2, t_f);
        let beta1_t = fast_pow(self.beta1, t_f);
        let lr = self.lr * sqrt_approx(1.0 - beta2_t) / (1.0 - beta1_t);

        for i in 0..weights.len() {
            self.m[i] = self.beta1 * self.m[i] + (1.0 - self.beta1) * gradients[i];
            self.v[i] = self.beta2 * self.v[i] + (1.0 - self.beta2) * gradients[i] * gradients[i];

            let update = self.m[i] / (sqrt_approx(self.v[i]) + self.epsilon);
            weights[i] -= lr * (update + self.weight_decay * weights[i]);
        }
    }

    /// Technical implementation of the learning_rate logic.
    fn learning_rate(&self) -> f32 {
        self.lr
    }
    /// Technical implementation of the set_learning_rate logic.
    fn set_learning_rate(&mut self, lr: f32) {
        self.lr = lr;
    }
}

/// Technical implementation of the RMSprop structure.
pub struct RMSprop {
    pub lr: f32,
    pub alpha: f32,
    pub epsilon: f32,
    pub momentum: f32,
    square_avg: Vec<f32>,
    #[allow(dead_code)]
    grad_avg: Vec<f32>,
}

impl RMSprop {
    /// Initializes a new instance of the associated type.
    pub fn new(lr: f32, size: usize) -> Self {
        Self {
            lr,
            alpha: 0.99,
            epsilon: 1e-8,
            momentum: 0.0,
            square_avg: vec![0.0; size],
            grad_avg: vec![0.0; size],
        }
    }
}

impl Optimizer for RMSprop {
    /// Technical implementation of the step logic.
    fn step(&mut self, weights: &mut [f32], gradients: &[f32]) {
        for i in 0..weights.len() {
            self.square_avg[i] =
                self.alpha * self.square_avg[i] + (1.0 - self.alpha) * gradients[i] * gradients[i];
            let avg = sqrt_approx(self.square_avg[i]) + self.epsilon;
            weights[i] -= self.lr * gradients[i] / avg;
        }
    }

    /// Technical implementation of the learning_rate logic.
    fn learning_rate(&self) -> f32 {
        self.lr
    }
    /// Technical implementation of the set_learning_rate logic.
    fn set_learning_rate(&mut self, lr: f32) {
        self.lr = lr;
    }
}

/// Technical implementation of the AdamW structure.
pub struct AdamW {
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub epsilon: f32,
    pub weight_decay: f32,
    m: Vec<f32>,
    v: Vec<f32>,
    t: u32,
}

impl AdamW {
    /// Initializes a new instance of the associated type.
    pub fn new(lr: f32, size: usize) -> Self {
        Self {
            lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            weight_decay: 0.01,
            m: vec![0.0; size],
            v: vec![0.0; size],
            t: 0,
        }
    }
}

impl Optimizer for AdamW {
    /// Technical implementation of the step logic.
    fn step(&mut self, weights: &mut [f32], gradients: &[f32]) {
        self.t += 1;
        let t_f = self.t as f32;
        let beta2_t = fast_pow(self.beta2, t_f);
        let beta1_t = fast_pow(self.beta1, t_f);
        let lr = self.lr * sqrt_approx(1.0 - beta2_t) / (1.0 - beta1_t);

        for i in 0..weights.len() {
            self.m[i] = self.beta1 * self.m[i] + (1.0 - self.beta1) * gradients[i];
            self.v[i] = self.beta2 * self.v[i] + (1.0 - self.beta2) * gradients[i] * gradients[i];

            let update = self.m[i] / (sqrt_approx(self.v[i]) + self.epsilon);
            weights[i] -= lr * (update + self.weight_decay * weights[i]);
        }
    }

    /// Technical implementation of the learning_rate logic.
    fn learning_rate(&self) -> f32 {
        self.lr
    }
    /// Technical implementation of the set_learning_rate logic.
    fn set_learning_rate(&mut self, lr: f32) {
        self.lr = lr;
    }
}
