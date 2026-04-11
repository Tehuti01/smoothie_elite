//! Boolean parameter (toggle).

use std::sync::atomic::{AtomicBool, Ordering};
use crate::{ParamId, Param};

pub struct BoolParam {
    id:      ParamId,
    name:    &'static str,
    default: bool,
    value:   AtomicBool,
}

impl BoolParam {
    pub fn new(id: ParamId, name: &'static str, default: bool) -> Self {
        Self { id, name, default, value: AtomicBool::new(default) }
    }

    #[inline]
    pub fn value(&self) -> bool { self.value.load(Ordering::Relaxed) }

    #[inline]
    pub fn set(&self, v: bool) { self.value.store(v, Ordering::Relaxed); }
}

impl Param for BoolParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }
    fn normalized(&self) -> f32 { if self.value() { 1.0 } else { 0.0 } }
    fn set_normalized(&self, v: f32) { self.set(v >= 0.5); }
    fn display(&self) -> String {
        if self.value() { "On".into() } else { "Off".into() }
    }
}
