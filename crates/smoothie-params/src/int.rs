//! Integer and Enum parameters.

use std::sync::atomic::{AtomicI32, Ordering};
use crate::{ParamId, Param, IntRange};

/// A real-time-safe integer parameter.
pub struct IntParam {
    id:      ParamId,
    name:    &'static str,
    default: i32,
    range:   IntRange,
    value:   AtomicI32,
    /// Optional labels for discrete steps (e.g. ["OFF", "ON"]).
    labels:  Option<&'static [&'static str]>,
    /// Unit suffix (e.g. " ms", " %").
    unit:    &'static str,
}

impl IntParam {
    pub fn new(id: ParamId, name: &'static str, default: i32) -> Self {
        Self {
            id, name, default,
            range: IntRange { min: 0, max: 100 },
            value: AtomicI32::new(default),
            labels: None,
            unit: "",
        }
    }

    pub fn range(mut self, range: IntRange) -> Self { self.range = range; self }

    /// Attach labels for enum-like behavior.
    pub fn labels(mut self, labels: &'static [&'static str]) -> Self {
        self.labels = Some(labels);
        self
    }

    /// Set unit suffix.
    pub fn unit(mut self, unit: &'static str) -> Self {
        self.unit = unit;
        self
    }

    #[inline]
    pub fn value(&self) -> i32 { self.value.load(Ordering::Relaxed) }

    #[inline]
    pub fn set(&self, v: i32) {
        self.value.store(v.clamp(self.range.min, self.range.max), Ordering::Release);
    }

    pub fn range_ref(&self) -> &IntRange { &self.range }
}

impl Param for IntParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }
    fn normalized(&self) -> f32 { self.range.normalize(self.value()) }
    fn set_normalized(&self, v: f32) { self.set(self.range.denormalize(v)); }

    fn display(&self) -> String {
        if let Some(labels) = self.labels {
            let idx = (self.value() - self.range.min) as usize;
            if idx < labels.len() {
                return labels[idx].to_string();
            }
        }
        format!("{}{}", self.value(), self.unit)
    }
}

/// A wrapper around IntParam for type-safe Enums.
pub struct EnumParam<T: Into<i32> + From<i32> + Copy> {
    inner: IntParam,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Into<i32> + From<i32> + Copy> EnumParam<T> {
    pub fn new(id: ParamId, name: &'static str, default: T, labels: &'static [&'static str]) -> Self {
        let default_val = default.into();
        Self {
            inner: IntParam::new(id, name, default_val)
                .range(IntRange { min: 0, max: (labels.len() as i32 - 1).max(0) })
                .labels(labels),
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn value(&self) -> T {
        T::from(self.inner.value())
    }

    pub fn set(&self, v: T) {
        self.inner.set(v.into());
    }

    pub fn inner(&self) -> &IntParam { &self.inner }
}
