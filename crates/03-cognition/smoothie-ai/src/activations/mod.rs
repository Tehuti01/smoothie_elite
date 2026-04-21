/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x26dec703 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod exponential;
pub mod extended;
pub mod linear;
pub mod softmax;
pub mod trigonometric;

pub use exponential::*;
pub use extended::*;
pub use linear::*;
pub use softmax::*;
pub use trigonometric::*;

/// Uniform enum dispatcher for the massive activation library.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the Activation enumeration.
pub enum Activation {
    Linear,
    Relu,
    LeakyRelu(f32),
    Elu(f32),
    Selu,
    HardShrink(f32),
    SoftShrink(f32),
    Sigmoid,
    HardSigmoid,
    Swish,
    Softplus,
    Mish,
    Gelu,
    Softsign,
    Tanh,
    Sine,
    Cosine,
}

impl Activation {
    #[inline]
    /// Technical implementation of the apply logic.
    pub fn apply(&self, x: f32) -> f32 {
        match self {
            Activation::Linear => linear(x),
            Activation::Relu => relu(x),
            Activation::LeakyRelu(a) => leaky_relu(x, *a),
            Activation::Elu(a) => elu(x, *a),
            Activation::Selu => selu(x),
            Activation::HardShrink(lambda) => hard_shrink(x, *lambda),
            Activation::SoftShrink(lambda) => soft_shrink(x, *lambda),
            Activation::Sigmoid => sigmoid(x),
            Activation::HardSigmoid => hard_sigmoid(x),
            Activation::Swish => swish(x),
            Activation::Softplus => softplus(x),
            Activation::Mish => mish(x),
            Activation::Gelu => gelu(x),
            Activation::Softsign => softsign(x),
            Activation::Tanh => tanh(x),
            Activation::Sine => sine(x),
            Activation::Cosine => cosine(x),
        }
    }
}
