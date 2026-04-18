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
    /// Optional modulation source aligned with the divine Phi-ratio.
    modulator: Option<std::sync::Arc<dyn crate::modulation::Modulator>>,
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
            modulator: None,
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
            modulator: None,
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

    /// Set a modulation source for this parameter.
    pub fn modulation(mut self, modulator: std::sync::Arc<dyn crate::modulation::Modulator>) -> Self {
        self.modulator = Some(modulator);
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

    /// Get the modulated value [normalized].
    #[inline]
    pub fn modulated_normalized(&self) -> f32 {
        let base = self.normalized();
        if let Some(modulator) = &self.modulator {
            let mod_val = modulator.next_value();
            let depth = modulator.depth();
            (base + mod_val * depth).clamp(0.0, 1.0)
        } else {
            base
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_param_normalization() {
        let param = FloatParam::simple("Gain", 0.0, -100.0, 0.0);
        assert_eq!(param.normalized(), 1.0); // 0.0 is max of -100.0..0.0
        
        param.set_normalized(0.5);
        assert_eq!(param.value(), -50.0);
    }

    #[test]
    fn test_float_param_clamping() {
        let param = FloatParam::simple("Freq", 500.0, 20.0, 20000.0);
        param.set(25000.0);
        assert_eq!(param.value(), 20000.0);
        
        param.set(10.0);
        assert_eq!(param.value(), 20.0);
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
