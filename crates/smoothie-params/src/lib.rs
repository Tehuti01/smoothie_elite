//! # smoothie-params
//!
//! Type-safe, real-time-safe, host-automatable parameters.
//!
//! ```rust,no_run
//! use smoothie_params::prelude::*;
//!
//! struct MyParams {
//!     pub gain:    FloatParam,
//!     pub enabled: BoolParam,
//!     pub mode:    IntParam,
//! }
//!
//! impl Default for MyParams {
//!     fn default() -> Self {
//!         Self {
//!             gain: FloatParam::new("Gain", 0.5)
//!                 .range(0.0..=1.0)
//!                 .unit(" dB")
//!                 .smoother(SmoothingStyle::Linear(20.0)),
//!             enabled: BoolParam::new("Enabled", true),
//!             mode: IntParam::new("Mode", 0).range(0..=3),
//!         }
//!     }
//! }
//! ```

pub mod float;
pub mod int;
pub mod bool_param;
pub mod smoothing;
pub mod range;

pub use float::FloatParam;
pub use int::{IntParam, EnumParam};
pub use bool_param::BoolParam;
pub use smoothing::{Smoother, SmoothingStyle};
pub use range::{FloatRange, IntRange};

/// Everything needed to use the params system.
pub mod prelude {
    pub use crate::{FloatParam, IntParam, EnumParam, BoolParam, Smoother, SmoothingStyle, FloatRange, IntRange};
}

/// Unique parameter ID for host automation.
pub type ParamId = &'static str;

/// The common interface for all parameter types.
pub trait Param: Send + Sync {
    /// Unique identifier used by the host for automation.
    fn id(&self) -> ParamId;
    /// Display name shown in the DAW.
    fn name(&self) -> &str;
    /// Normalized value in [0.0, 1.0] for host automation.
    fn normalized(&self) -> f32;
    /// Set from normalized value (called by host automation).
    fn set_normalized(&self, v: f32);
    /// Human-readable value string.
    fn display(&self) -> String;
}

/// A registry of parameters for a plugin.
#[derive(Default)]
pub struct ParamRegistry {
    pub params: Vec<std::sync::Arc<dyn Param>>,
}

impl ParamRegistry {
    /// Create a new empty parameter registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a parameter to the registry.
    pub fn add(&mut self, param: std::sync::Arc<dyn Param>) {
        self.params.push(param);
    }
}
