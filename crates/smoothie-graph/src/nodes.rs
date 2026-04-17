//! 'Elite' Modular Nodes for the SeFi-Sam ecosystem.

pub mod sampler;
pub mod spectral;
pub mod modulation;
pub mod physical;

pub use sampler::SamplerNode;
pub use spectral::SpectralNode;
pub use modulation::NeuralModNode;
pub use physical::PhysicalNode;
