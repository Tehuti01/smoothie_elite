//! # Amplifier Modeling Modules
//!
//! This module contains the core components of a guitar amplifier simulation.
//! It includes tube-stage emulation, high-gain preamps, sagging power amps, 
//! and dedicated distortion circuits.

pub mod amplifier;
pub mod distortion;
pub mod poweramp;
pub mod preamp;
pub mod tube_stage;

pub use amplifier::Amplifier;
pub use distortion::Distortion;
pub use poweramp::Poweramp;
pub use preamp::Preamp;
pub use tube_stage::TubeStage;
