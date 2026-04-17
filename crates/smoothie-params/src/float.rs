//! Float parameter — the workhorse of audio plugins.

use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;
use crate::{ParamId, Param, FloatRange, Smoother, SmoothingStyle};

/// A real-time-safe, host-automatable float parameter.
/// 
/// This is the 'Elite' version: zero locks, atomic-backed, and 
/// designed for high-performance DSP threads.
pub struct FloatParam {
    id:       ParamId,
    name:     &'static str,
    default:  f32,
    unit:     &'static str,
    range:    FloatRange,
    /// The target value (usually set by the host or UI).
    value:    AtomicF32,
    /// How the value should be smoothed in the audio thread.
    style:    SmoothingStyle,
}

impl FloatParam {
    pub fn new(id: ParamId, name: &'static str, default: f32) -> Self {
        Self {
            id,
            name,
            default,
            unit: "",
            range: FloatRange::default(),
            value: AtomicF32::new(default),
            style: SmoothingStyle::None,
        }
    }

    /// Create a new FloatParam with a simple name-based ID and min/max range.
    /// Convenience method for plugin development.
    pub fn simple(name: &'static str, default: f32, min: f32, max: f32) -> Self {
        Self {
            id: name,  // Use name as ID
            name,
            default: default.clamp(min, max),
            unit: "",
            range: FloatRange::Linear { min, max },
            value: AtomicF32::new(default.clamp(min, max)),
            style: SmoothingStyle::None,
        }
    }

    /// Set the value range.
    pub fn range(mut self, range: FloatRange) -> Self {
        self.range = range;
        self
    }

    /// Set unit string (appended to display).
    pub fn unit(mut self, unit: &'static str) -> Self {
        self.unit = unit;
        self
    }

    /// Set the smoothing style.
    pub fn smoothing(mut self, style: SmoothingStyle) -> Self {
        self.style = style;
        self
    }

    /// Get current target value (instant, no smoothing).
    #[inline]
    pub fn value(&self) -> f32 {
        self.value.load(Ordering::Relaxed)
    }

    /// Set value. Thread-safe, lock-free.
    #[inline]
    pub fn set(&self, v: f32) {
        let clamped = self.range.denormalize(self.range.normalize(v));
        self.value.store(clamped, Ordering::Release);
    }

    /// Create a new smoother that tracks this parameter.
    /// This smoother lives on the audio thread.
    pub fn create_smoother(&self, sample_rate: f32) -> Smoother {
        Smoother::new(self.value(), sample_rate, self.style)
    }

    pub fn style(&self) -> SmoothingStyle { self.style }
    pub fn range_ref(&self) -> &FloatRange { &self.range }
}

impl Param for FloatParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }
    fn normalized(&self) -> f32 { self.range.normalize(self.value()) }
    fn set_normalized(&self, v: f32) { self.set(self.range.denormalize(v)); }
    fn display(&self) -> String { format!("{:.2}{}", self.value(), self.unit) }
}
