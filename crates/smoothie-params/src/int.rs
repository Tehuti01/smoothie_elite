//! Integer parameter.

use std::sync::atomic::{AtomicI32, Ordering};
use crate::{ParamId, Param, IntRange};

pub struct IntParam {
    id:      ParamId,
    name:    &'static str,
    default: i32,
    range:   IntRange,
    value:   AtomicI32,
    labels:  Option<&'static [&'static str]>,
}

impl IntParam {
    pub fn new(id: ParamId, name: &'static str, default: i32) -> Self {
        Self {
            id, name, default,
            range: IntRange { min: 0, max: 100 },
            value: AtomicI32::new(default),
            labels: None,
        }
    }

    pub fn range(mut self, range: IntRange) -> Self { self.range = range; self }

    /// Attach enum labels (one per step).
    pub fn labels(mut self, labels: &'static [&'static str]) -> Self {
        self.labels = Some(labels);
        self
    }

    #[inline]
    pub fn value(&self) -> i32 { self.value.load(Ordering::Relaxed) }

    #[inline]
    pub fn set(&self, v: i32) {
        self.value.store(v.clamp(self.range.min, self.range.max), Ordering::Relaxed);
    }
}

impl Param for IntParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }

    fn normalized(&self) -> f32 { self.range.normalize(self.value()) }

    fn set_normalized(&self, v: f32) {
        self.set(self.range.denormalize(v));
    }

    fn display(&self) -> String {
        if let Some(labels) = self.labels {
            let idx = (self.value() - self.range.min) as usize;
            if idx < labels.len() {
                return labels[idx].to_string();
            }
        }
        self.value().to_string()
    }
}
