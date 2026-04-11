//! Float parameter — the workhorse of audio plugins.

use std::sync::atomic::{AtomicU32, Ordering};
use crate::{ParamId, Param, FloatRange, Smoother, SmoothingStyle};

fn f32_to_bits(v: f32) -> u32 { v.to_bits() }
fn bits_to_f32(v: u32) -> f32 { f32::from_bits(v) }

/// A real-time-safe, host-automatable float parameter.
pub struct FloatParam {
    id:       ParamId,
    name:     &'static str,
    default:  f32,
    unit:     &'static str,
    range:    FloatRange,
    value:    AtomicU32,
    smoother: Option<std::sync::Mutex<Smoother>>,
}

impl FloatParam {
    pub fn new(id: ParamId, name: &'static str, default: f32) -> Self {
        Self {
            id,
            name,
            default,
            unit: "",
            range: FloatRange::default(),
            value: AtomicU32::new(f32_to_bits(default)),
            smoother: None,
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

    /// Attach a smoother for the audio thread.
    pub fn smoother(mut self, style: SmoothingStyle) -> Self {
        self.smoother = Some(std::sync::Mutex::new(
            Smoother::new(self.default, 44100.0, style)
        ));
        self
    }

    /// Get current value (instant, no smoothing).
    #[inline]
    pub fn value(&self) -> f32 {
        bits_to_f32(self.value.load(Ordering::Relaxed))
    }

    /// Set value. Thread-safe, lock-free.
    #[inline]
    pub fn set(&self, v: f32) {
        let clamped = self.range.denormalize(self.range.normalize(v));
        self.value.store(f32_to_bits(clamped), Ordering::Relaxed);
    }

    /// Get next smoothed value. Call once per sample on audio thread.
    pub fn smoothed(&self) -> f32 {
        if let Some(smoother) = &self.smoother {
            if let Ok(mut s) = smoother.try_lock() {
                s.set_target(self.value());
                return s.next();
            }
        }
        self.value()
    }

    pub fn range_ref(&self) -> &FloatRange { &self.range }
}

impl Param for FloatParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }
    fn normalized(&self) -> f32 { self.range.normalize(self.value()) }
    fn set_normalized(&self, v: f32) { self.set(self.range.denormalize(v)); }
    fn display(&self) -> String { format!("{:.2}{}", self.value(), self.unit) }
}
