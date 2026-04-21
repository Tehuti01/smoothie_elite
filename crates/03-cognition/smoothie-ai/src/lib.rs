/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf76cfd5b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
extern crate smoothie_core;

pub mod activations;
pub mod attention;
pub mod autoencoder;
pub mod batching;
pub mod conformer;
pub mod dense;
pub mod diffusion;
pub mod embedding;
pub mod layers;
pub mod loss;
pub mod model;
pub mod nam;
pub mod normalization;
pub mod optimizer;
pub mod positional;
pub mod rnn;
pub mod training;
pub mod transformer;
pub mod vae;

pub use activations::*;
pub use autoencoder::{AudioEncoder, LayerNorm as AutoLayerNorm};
pub use conformer::{ConformerBlock, PreNorm as ConformerPreNorm};
pub use dense::DenseLayer;
pub use diffusion::{DenoisingDiffusion, NoisePredictor};
pub use transformer::{PreNorm as TransformerPreNorm, TransformerEncoderLayer};
