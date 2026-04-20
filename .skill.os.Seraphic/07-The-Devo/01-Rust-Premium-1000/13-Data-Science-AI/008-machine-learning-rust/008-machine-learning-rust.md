# SKILL 008: MACHINE LEARNING IN RUST - TENSOR COMPUTATION

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        MACHINE LEARNING IN RUST
                     Tensor Computation & Neural Networks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of machine learning in Rust with tensor operations,
neural networks, automatic differentiation, and GPU acceleration.
Covers linear regression, CNNs, RNNs, transformers, and deployment.

## TABLE OF CONTENTS

1. [Tensor Fundamentals](#tensor-fundamentals)
2. [Automatic Differentiation](#automatic-differentiation)
3. [Neural Network Layers](#neural-network-layers)
4. [Optimizers](#optimizers)
5. [CNN Architecture](#cnn-architecture)
6. [RNN & LSTM](#rnn--lstm)
7. [Transformers](#transformers)
8. [GPU Acceleration](#gpu-acceleration)
9. [Model Training](#model-training)
10. [Deployment](#deployment)

---

## TENSOR FUNDAMENTALS

### 1.1 Tensor Structure

```rust
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::sync::Arc;

/// Multi-dimensional array
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
    pub device: Device,
    grad: Option<Arc<Tensor>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Device {
    Cpu,
    Gpu(usize), // GPU device ID
}

impl Tensor {
    pub fn new(shape: &[usize]) -> Self {
        let size: usize = shape.iter().product();
        let strides = Tensor::compute_strides(shape);
        
        Tensor {
            data: vec![0.0; size],
            shape: shape.to_vec(),
            strides,
            device: Device::Cpu,
            grad: None,
        }
    }

    pub fn from_vec(shape: &[usize], data: Vec<f32>) -> Self {
        let strides = Tensor::compute_strides(shape);
        
        Tensor {
            data,
            shape: shape.to_vec(),
            strides,
            device: Device::Cpu,
            grad: None,
        }
    }

    fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![0usize; shape.len()];
        if shape.is_empty() { return strides; }
        
        strides[shape.len() - 1] = 1;
        for i in (0..shape.len() - 1).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }
        strides
    }

    pub fn view(&self) -> TensorView {
        TensorView {
            data: self.data.as_ptr() as *const f32,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
        }
    }

    pub fn reshape(&self, new_shape: &[usize]) -> Option<Tensor> {
        if self.data.len() != new_shape.iter().product() {
            return None;
        }
        
        Some(Tensor {
            data: self.data.clone(),
            shape: new_shape.to_vec(),
            strides: Tensor::compute_strides(new_shape),
            device: self.device.clone(),
            grad: self.grad.clone(),
        })
    }

    pub fn fill(&mut self, value: f32) {
        self.data.fill(value);
    }

    pub fn zeros(shape: &[usize]) -> Self {
        let mut t = Tensor::new(shape);
        t.fill(0.0);
        t
    }

    pub fn ones(shape: &[usize]) -> Self {
        let mut t = Tensor::new(shape);
        t.fill(1.0);
        t
    }

    pub fn randn(shape: &[usize]) -> Self {
        use std::random::OsRandom;
        
        let mut data = vec![0.0f32; shape.iter().product()];
        for v in &mut data {
            // Box-Muller transform
            let u1: f32 = OsRandom.next_f32();
            let u2: f32 = OsRandom.next_f32();
            *v = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos();
        }
        
        Tensor::from_vec(shape, data)
    }

    pub fn index(&self, indices: &[usize]) -> f32 {
        let offset: usize = indices
            .iter()
            .zip(self.strides.iter())
            .map((&i, &s)| i * s)
            .sum();
        
        self.data[offset]
    }

    pub fn set(&mut self, indices: &[usize], value: f32) {
        let offset: usize = indices
            .iter()
            .zip(self.strides.iter())
            .map((&i, &s)| i * s)
            .sum();
        
        self.data[offset] = value;
    }

    pub fn broadcast_to(&self, target_shape: &[usize]) -> Option<Tensor> {
        // Check if broadcasting is valid
        for (i, (s, t)) in self.shape.iter().zip(target_shape.iter()).enumerate() {
            if *s != *t && *s != 1 {
                return None;
            }
        }
        
        let mut result = Tensor::new(target_shape);
        // Apply broadcasting logic
        result
    }
}

pub struct TensorView<'a> {
    data: *const f32,
    shape: Vec<usize>,
    strides: Vec<usize>,
}
```

### 1.2 Tensor Operations

```rust
impl Tensor {
    pub fn add(&self, other: &Tensor) -> Tensor {
        broadcast_binary_op(self, other, |a, b| a + b)
    }

    pub fn sub(&self, other: &Tensor) -> Tensor {
        broadcast_binary_op(self, other, |a, b| a - b)
    }

    pub fn mul(&self, other: &Tensor) -> Tensor {
        broadcast_binary_op(self, other, |a, b| a * b)
    }

    pub fn matmul(&self, other: &Tensor) -> Option<Tensor> {
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return None;
        }
        
        if self.shape[1] != other.shape[0] {
            return None;
        }
        
        let m = self.shape[0];
        let k = self.shape[1];
        let n = other.shape[1];
        let mut result = Tensor::new(&[m, n]);
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0f32;
                for p in 0..k {
                    sum += self.index(&[i, p]) * other.index(&[p, j]);
                }
                result.set(&[i, j], sum);
            }
        }
        
        Some(result)
    }

    pub fn sum(&self, axis: Option<usize>) -> Tensor {
        match axis {
            None => {
                let s: f32 = self.data.iter().sum();
                Tensor::from_vec(&[], vec![s])
            }
            Some(ax) => {
                let mut out_shape = self.shape.clone();
                out_shape[ax] = 1;
                let mut result = Tensor::new(&out_shape);
                
                // Sum along axis
                let step = self.strides[ax];
                let repeat = self.shape[ax];
                let outer: usize = self.data.len() / (step * repeat);
                
                for i in 0..outer {
                    let offset = i * step * repeat;
                    let mut sum = 0.0f32;
                    for j in 0..repeat {
                        sum += self.data[offset + j * step];
                    }
                    result.data[i] = sum;
                }
                
                result
            }
        }
    }

    pub fn mean(&self, axis: Option<usize>) -> Tensor {
        let sum = self.sum(axis);
        let count: f32 = if let Some(ax) = axis {
            self.shape[ax] as f32
        } else {
            self.data.len() as f32
        };
        
        Tensor::from_vec(&sum.shape, sum.data.iter().map(|&v| v / count).collect())
    }

    pub fn transpose(&self) -> Tensor {
        if self.shape.len() != 2 {
            return self.clone();
        }
        
        let mut result = Tensor::new(&[self.shape[1], self.shape[0]]);
        
        for i in 0..self.shape[0] {
            for j in 0..self.shape[1] {
                result.set(&[j, i], self.index(&[i, j]));
            }
        }
        
        result
    }

    pub fn relu(&self) -> Tensor {
        Tensor::from_vec(
            &self.shape,
            self.data.iter().map(|&v| v.max(0.0)).collect()
        )
    }

    pub fn sigmoid(&self) -> Tensor {
        Tensor::from_vec(
            &self.shape,
            self.data.iter().map(|&v| 1.0 / (1.0 + (-v).exp())).collect()
        )
    }

    pub fn softmax(&self, axis: usize) -> Tensor {
        let max = self
            .data
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        let exp_sum: f32 = self
            .data
            .iter()
            .map(|&v| (v - max).exp())
            .sum();
        
        Tensor::from_vec(
            &self.shape,
            self.data
                .iter()
                .map(|&v| (v - max).exp() / exp_sum)
                .collect(),
        )
    }

    pub fn cross_entropy(&self, target: &Tensor) -> Tensor {
        let eps = 1e-8;
        
        let loss: f32 = self
            .data
            .iter()
            .zip(target.data.iter())
            .map(|(&pred, &tgt)| {
                let p = pred.max(eps).min(1.0 - eps);
                -(tgt * p.ln() + (1.0 - tgt) * (1.0 - p).ln())
            })
            .sum();
        
        Tensor::from_vec(&[], vec![loss / self.data.len() as f32])
    }
}
```

---

## AUTOMATIC DIFFERENTIATION

### 2.1 Autograd

```rust
/// Computational graph node
pub struct Variable {
    pub data: Tensor,
    pub grad: Tensor,
    pub requires_grad: bool,
    pub creator: Option<GradientFunction>,
}

enum GradientFunction {
    Add(Arc<Tensor>, Arc<Tensor>),
    Mul(Arc<Tensor>, Arc<Tensor>),
    Matmul(Arc<Tensor>, Arc<Tensor>),
    ReLu(Arc<Tensor>),
    Sigmoid(Arc<Tensor>),
    Softmax(Arc<Tensor>),
    Sum(Arc<Tensor>),
    Mean(Arc<Tensor>),
}

impl Variable {
    pub fn new(data: Tensor) -> Self {
        Variable {
            data,
            grad: Tensor::new(&[]),
            requires_grad: true,
            creator: None,
        }
    }

    pub fn from_data(data: Vec<f32>, shape: &[usize]) -> Self {
        Variable::new(Tensor::from_vec(shape, data))
    }

    pub fn backward(&mut self) {
        if !self.requires_grad {
            return;
        }

        self.grad = Tensor::ones(&self.data.shape);

        match &self.creator {
            Some(GradientFunction::Add(a, b)) => {
                // Add gradient: dL/dx = dL/dy * 1
            }
            Some(GradientFunction::Mul(a, b)) => {
                // Mul gradient: dL/dx = dL/dy * y
            }
            Some(GradientFunction::Matmul(a, b)) => {
                // Matrix multiply chain rule
                let a_data = &a.data;
                let b_data = &b.data;
                // Compute gradients
            }
            Some(GradientFunction::ReLu(x)) => {
                let mask = Tensor::from_vec(
                    &x.data.shape,
                    x.data.iter().map(|&v| if v > 0.0 { 1.0 } else { 0.0 }).collect(),
                );
                self.grad = self.grad.mul(&mask);
            }
            _ => {}
        }
    }

    pub fn zero_grad(&mut self) {
        self.grad = Tensor::zeros(&self.data.shape);
    }
}

/// Context for recording operations
pub struct GradientContext {
    pub stack: Vec<GradientFunction>,
}

impl GradientContext {
    pub fn new() -> Self {
        GradientContext { stack: Vec::new() }
    }

    pub fn record_add(&mut self, a: Variable, b: Variable) -> Variable {
        let result = a.data.add(&b.data);
        
        let mut var = Variable::new(result);
        var.creator = Some(GradientFunction::Add(
            Arc::new(a.data.clone()),
            Arc::new(b.data.clone()),
        ));
        var
    }
}
```

---

## NEURAL NETWORK LAYERS

### 3.1 Dense Layer

```rust
/// Fully connected layer
pub struct Linear {
    pub weight: Tensor,
    pub bias: Tensor,
    pub input_features: usize,
    pub output_features: usize,
}

impl Linear {
    pub fn new(input_features: usize, output_features: usize) -> Self {
        let scale = (1.0 / input_features as f32).sqrt();
        
        Linear {
            weight: Tensor::randn(&[input_features, output_features])
                .data
                .iter()
                .map(|&v| v * scale)
                .collect::<Vec<_>>()
                .into(),
            bias: Tensor::zeros(&[1, output_features]),
            input_features,
            output_features,
        }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let output = input.matmul(&self.weight).unwrap();
        output.add(&self.bias)
    }

    pub fn parameters(&self) -> Vec<&mut Tensor> {
        vec![&mut self.weight, &mut self.bias]
    }
}

/// ReLU activation
pub struct ReLu;

impl ReLu {
    pub fn forward(&self, input: &Tensor) -> Tensor {
        input.relu()
    }
}

/// Sigmoid activation
pub struct Sigmoid;

impl Sigmoid {
    pub fn forward(&self, input: &Tensor) -> Tensor {
        input.sigmoid()
    }
}

/// Dropout
pub struct Dropout {
    pub p: f32,
}

impl Dropout {
    pub fn new(p: f32) -> Self {
        Dropout { p }
    }

    pub fn forward(&self, input: &Tensor, training: bool) -> Tensor {
        if training {
            let mask: Vec<f32> = input
                .data
                .iter()
                .map(|_| if random::<f32>() > self.p { 1.0 / (1.0 - self.p) } else { 0.0 })
                .collect();
            
            Tensor::from_vec(&input.shape, mask).mul(input)
        } else {
            input.clone()
        }
    }
}
```

---

## OPTIMIZERS

### 4.1 SGD

```rust
pub struct SGD {
    pub lr: f32,
    pub momentum: f32,
    pub velocity: Vec<Tensor>,
}

impl SGD {
    pub fn new(lr: f32) -> Self {
        SGD {
            lr,
            momentum: 0.0,
            velocity: Vec::new(),
        }
    }

    pub fn with_momentum(lr: f32, momentum: f32) -> Self {
        SGD {
            lr,
            momentum,
            velocity: Vec::new(),
        }
    }

    pub fn step(&mut self, parameters: &[&mut Tensor], gradients: &[&Tensor]) {
        for (param, grad) in parameters.iter().zip(gradients.iter()) {
            let velocity_idx = parameters.iter().position(|p| p.data.as_ptr() == param.data.as_ptr());
            
            if let Some(idx) = velocity_idx {
                let v = &mut self.velocity[idx];
                *v = v.mul(self.momentum).sub(grad.mul(self.lr));
                param.add(v);
            } else {
                // First time - create velocity
                let velocity = Tensor::from_vec(
                    &param.shape,
                    param.data.iter().map(|&v| -self.lr * v).collect(),
                );
                self.velocity.push(velocity);
                param.add(&self.velocity.last().unwrap());
            }
        }
    }
}

/// Adam optimizer
pub struct Adam {
    pub lr: f32,
    pub beta1: f32,
    pub beta2: f32,
    pub epsilon: f32,
    pub m: Vec<Tensor>,
    pub v: Vec<Tensor>,
    pub t: usize,
}

impl Adam {
    pub fn new(lr: f32) -> Self {
        Adam {
            lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            m: Vec::new(),
            v: Vec::new(),
            t: 0,
        }
    }

    pub fn step(&mut self, parameters: &[&mut Tensor], gradients: &[&Tensor]) {
        self.t += 1;
        
        for (param, grad) in parameters.iter().zip(gradients.iter()) {
            // Update biased first moment estimate
            let m = grad.mul(self.beta1);
            // Update biased second moment estimate
            let v = grad.mul(grad).mul(self.beta2);
            
            // Bias correction
            let m_hat = m.div(1.0 - self.beta1.powi(self.t as i32));
            let v_hat = v.div(1.0 - self.beta2.powi(self.t as i32));
            
            // Update parameter
            let update = m_hat.div(v_hat.sqrt().add(self.epsilon));
            param.sub(&update.mul(self.lr));
        }
    }
}
```

---

## CNN ARCHITECTURE

### 5.1 Convolutional Layer

```rust
pub struct Conv2d {
    pub in_channels: usize,
    pub out_channels: usize,
    pub kernel_size: usize,
    pub stride: usize,
    pub padding: usize,
    pub weight: Tensor,
    pub bias: Tensor,
}

impl Conv2d {
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: usize,
    ) -> Self {
        let scale = (in_channels * kernel_size * kernel_size) as f32;
        
        Conv2d {
            in_channels,
            out_channels,
            kernel_size,
            stride: 1,
            padding: 0,
            weight: Tensor::randn(&[
                out_channels,
                in_channels,
                kernel_size,
                kernel_size,
            ])
            .data
            .iter()
            .map(|&v| v / scale.sqrt())
            .collect::<Vec<_>>()
            .into(),
            bias: Tensor::zeros(&[1, out_channels]),
        }
    }

    pub fn forward(&self, input: &Tensor) -> Tensor {
        let (_, _, h, w) = (
            self.in_channels,
            self.in_channels,
            input.shape[2],
            input.shape[3],
        );
        
        let out_h = (h + 2 * self.padding - self.kernel_size) / self.stride + 1;
        let out_w = (w + 2 * self.padding - self.kernel_size) / self.stride + 1;
        
        let mut output = Tensor::zeros(&[input.shape[0], self.out_channels, out_h, out_w]);
        
        // Convolution implementation
        for batch in 0..input.shape[0] {
            for out_c in 0..self.out_channels {
                for i in 0..out_h {
                    for j in 0..out_w {
                        let mut sum = self.bias.index(&[out_c]);
                        
                        for in_c in 0..self.in_channels {
                            for ki in 0..self.kernel_size {
                                for kj in 0..self.kernel_size {
                                    let ii = i * self.stride + ki - self.padding;
                                    let jj = j * self.stride + kj - self.padding;
                                    
                                    if ii < h && jj < w {
                                        sum += input.index(&[batch, in_c, ii, jj])
                                            * self.weight.index(&[out_c, in_c, ki, kj]);
                                    }
                                }
                            }
                        }
                        
                        output.set(&[batch, out_c, i, j], sum);
                    }
                }
            }
        }
        
        output
    }
}

/// Batch normalization
pub struct BatchNorm2d {
    pub num_features: usize,
    pub gamma: Tensor,
    pub beta: Tensor,
    pub running_mean: Tensor,
    pub running_var: Tensor,
    pub epsilon: f32,
    pub momentum: f32,
}

impl BatchNorm2d {
    pub fn new(num_features: usize) -> Self {
        BatchNorm2d {
            num_features,
            gamma: Tensor::ones(&[1, num_features, 1, 1]),
            beta: Tensor::zeros(&[1, num_features, 1, 1]),
            running_mean: Tensor::zeros(&[num_features]),
            running_var: Tensor::ones(&[num_features]),
            epsilon: 1e-5,
            momentum: 0.1,
        }
    }

    pub fn forward(&self, input: &Tensor, training: bool) -> Tensor {
        // Normalize
        let mean = input.mean(None);
        let var = input.var(None);
        
        let normalized = input.sub(&mean).div(&var.add(self.epsilon));
        
        // Scale and shift
        normalized.mul(&self.gamma).add(&self.beta)
    }
}
```

---

## RNN & LSTM

### 6.1 LSTM Cell

```rust
pub struct LstmCell {
    pub input_features: usize,
    pub hidden_features: usize,
    pub weight_ih: Tensor,
    pub weight_hh: Tensor,
    pub bias_ih: Tensor,
    pub bias_hh: Tensor,
}

impl LstmCell {
    pub fn new(input_features: usize, hidden_features: usize) -> Self {
        let scale = (input_features + hidden_features) as f32;
        
        LstmCell {
            input_features,
            hidden_features,
            weight_ih: Tensor::randn(&[4 * hidden_features, input_features])
                .data
                .iter()
                .map(|&v| v * scale.sqrt())
                .collect::<Vec<_>>()
                .into(),
            weight_hh: Tensor::randn(&[4 * hidden_features, hidden_features])
                .data
                .iter()
                .map(|&v| v * scale.sqrt())
                .collect::<Vec<_>>()
                .into(),
            bias_ih: Tensor::zeros(&[4 * hidden_features]),
            bias_hh: Tensor::zeros(&[4 * hidden_features]),
        }
    }

    pub fn forward(
        &self,
        input: &Tensor,
        hidden: &Tensor,
        cell: &Tensor,
    ) -> (Tensor, Tensor) {
        let gates = input.matmul(&self.weight_ih.t())
            .unwrap()
            .add(&hidden.matmul(&self.weight_hh.t()).unwrap())
            .add(&self.bias_ih)
            .add(&self.bias_hh);
        
        let i = gates.clone().slice(0, self.hidden_features).sigmoid();
        let f = gates.clone().slice(self.hidden_features, 2 * self.hidden_features).sigmoid();
        let g = gates.clone().slice(2 * self.hidden_features, 3 * self.hidden_features).tanh();
        let o = gates.clone().slice(3 * self.hidden_features, 4 * self.hidden_features).sigmoid();
        
        let new_cell = f.mul(cell).add(i.mul(g));
        let new_hidden = o.mul(new_cell.tanh());
        
        (new_hidden, new_cell)
    }
}

/// Multi-layer LSTM
pub struct LSTM {
    pub cells: Vec<LstmCell>,
    pub num_layers: usize,
}

impl LSTM {
    pub fn new(input_size: usize, hidden_size: usize, num_layers: usize) -> Self {
        let cells: Vec<LstmCell> = (0..num_layers)
            .map(|i| {
                let in_features = if i == 0 { input_size } else { hidden_size };
                LstmCell::new(in_features, hidden_size)
            })
            .collect();
        
        LSTM { cells, num_layers }
    }

    pub fn forward(
        &self,
        input: &Tensor,
        hidden: Option<&Tensor>,
    ) -> (Tensor, Vec<Tensor>) {
        let mut h = hidden.unwrap_or(&Tensor::zeros(&[self.num_layers, 1, self.cells[0].hidden_features]));
        let mut c = Tensor::zeros(&[self.num_layers, 1, self.cells[0].hidden_features]);
        
        let mut output_sequence: Vec<Tensor> = Vec::new();
        
        for t in 0..input.shape[1] {
            let x = input.slice(t);
            
            for layer in 0..self.num_layers {
                let (new_h, new_c) = self.cells[layer].forward(&x, &h.slice(layer), &c.slice(layer));
                h.set(&[layer], new_h);
                c.set(&[layer], new_c);
                x = new_h;
            }
            
            output_sequence.push(x);
        }
        
        (h, output_sequence)
    }
}
```

---

## TRANSFORMERS

### 7.1 Attention

```rust
pub struct MultiHeadAttention {
    pub embed_size: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub q_linear: Linear,
    pub k_linear: Linear,
    pub v_linear: Linear,
    pub out_linear: Linear,
}

impl MultiHeadAttention {
    pub fn new(embed_size: usize, num_heads: usize) -> Self {
        let head_dim = embed_size / num_heads;
        
        MultiHeadAttention {
            embed_size,
            num_heads,
            head_dim,
            q_linear: Linear::new(embed_size, embed_size),
            k_linear: Linear::new(embed_size, embed_size),
            v_linear: Linear::new(embed_size, embed_size),
            out_linear: Linear::new(embed_size, embed_size),
        }
    }

    pub fn forward(
        &self,
        query: &Tensor,
        key: &Tensor,
        value: &Tensor,
        mask: Option<&Tensor>,
    ) -> Tensor {
        let q = self.q_linear.forward(query);
        let k = self.k_linear.forward(key);
        let v = self.v_linear.forward(value);
        
        let q = self.split_heads(q);
        let k = self.split_heads(k);
        let v = self.split_heads(v);
        
        let scores = q.matmul(&k.transpose()).unwrap();
        let scores = scores.div((self.head_dim as f32).sqrt());
        
        if let Some(m) = mask {
            // Apply mask
        }
        
        let attention = scores.softmax(self.num_heads - 1);
        let attention = attention.matmul(&v).unwrap();
        
        let output = self.concat_heads(attention);
        self.out_linear.forward(&output)
    }

    fn split_heads(&self, tensor: Tensor) -> Tensor {
        let batch = tensor.shape[0];
        let seq = tensor.shape[1];
        tensor.reshape(&[batch, seq, self.num_heads, self.head_dim])
            .unwrap()
            .transpose()
    }

    fn concat_heads(&self, tensor: Tensor) -> Tensor {
        let batch = tensor.shape[0];
        let seq = tensor.shape[1];
        tensor.transpose()
            .reshape(&[batch, seq, self.embed_size])
            .unwrap()
    }
}

/// Transformer block
pub struct TransformerBlock {
    pub attention: MultiHeadAttention,
    pub norm1: LayerNorm,
    pub norm2: LayerNorm,
    pub feedforward: FeedForward,
}

pub struct FeedForward {
    pub linear1: Linear,
    pub linear2: Linear,
}

impl FeedForward {
    pub fn new(embed_size: usize, ff_size: usize) -> Self {
        FeedForward {
            linear1: Linear::new(embed_size, ff_size),
            linear2: Linear::new(ff_size, embed_size),
        }
    }

    pub fn forward(&self, x: &Tensor) -> Tensor {
        x.relu().linear1()
            .relu()
            .linear2()
    }
}
```

---

## GPU ACCELERATION

### 8.1 GPU Backend

```rust
#[cfg(feature = "cuda")]
pub mod gpu {
    use cuda::prelude::*;
    
    pub struct GpuTensor {
        pub device_ptr: DevicePtr<f32>,
        pub shape: Vec<usize>,
    }
    
    impl GpuTensor {
        pub fn new(shape: &[usize]) -> Self {
            let size = shape.iter().product();
            let ptr = cuda_malloc(size * std::mem::size_of::<f32>());
            
            GpuTensor {
                device_ptr: ptr,
                shape: shape.to_vec(),
            }
        }
        
        pub fn upload(&mut self, data: &[f32]) {
            cuda_memcpy(self.device_ptr, data);
        }
        
        pub fn download(&self) -> Vec<f32> {
            let size = self.shape.iter().product();
            let mut data = vec![0.0f32; size];
            cuda_memcpy_to_host(&data, self.device_ptr);
            data
        }
    }
}
```

---

## MODEL TRAINING

### 9.1 Training Loop

```rust
pub struct Trainer {
    pub model: Model,
    pub optimizer: Box<dyn Optimizer>,
    pub criterion: Box<dyn Loss>,
    pub lr: f32,
}

pub trait Optimizer {
    fn step(&mut self, parameters: &[&mut Tensor], gradients: &[&Tensor]);
}

pub trait Loss {
    fn forward(&self, input: &Tensor, target: &Tensor) -> Tensor;
    fn backward(&self) -> Tensor;
}

impl Trainer {
    pub fn train_epoch(&mut self, dataset: &Dataset) -> f32 {
        let mut total_loss = 0.0;
        
        for batch in dataset.batches(32) {
            let (inputs, targets) = batch;
            
            let output = self.model.forward(&inputs);
            let loss = self.criterion.forward(&output, &targets);
            
            let grad = self.criterion.backward();
            self.model.backward(grad);
            self.optimizer.step(&self.model.parameters());
            
            total_loss += loss.index(&[]);
        }
        
        total_loss / dataset.num_batches() as f32
    }

    pub fn evaluate(&self, dataset: &Dataset) -> f32 {
        let mut correct = 0;
        
        for batch in dataset.batches(32) {
            let (inputs, targets) = batch;
            
            let output = self.model.forward(&inputs);
            let predictions = output.argmax();
            
            if predictions.iter().zip(targets.iter()).all(|(p, t)| p == t) {
                correct += 1;
            }
        }
        
        correct as f32 / dataset.samples() as f32
    }
}
```

---

## DEPLOYMENT

### 10.1 ONNX Export

```rust
pub fn export_onnx(model: &Model, path: &str) -> Result<(), Error> {
    // Export model to ONNX format
    todo!()
}

pub fn load_onnx(path: &str) -> Model {
    // Load model from ONNX format
    todo!()
}

/// Quantization
pub fn quantize(model: &Model, bits: usize) -> QuantizedModel {
    // Quantize model weights
    todo!()
}
```

---

## RECAP

1. **Tensors are foundation** - Build efficient tensor ops first
2. **Autograd enables learning** - Record computation graph
3. **GPU when needed** - Start with CPU, optimize later
4. **Adam usually works** - Good default optimizer
5. **BatchNorm helps** - Stabilizes training
6. **Transformers are powerful** - But computationally expensive

---

*Skill ID: 008 | Category: Data-Science-AI | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*