//! Boolean parameter (toggle).

use std::sync::atomic::{AtomicBool, Ordering};
use crate::{ParamId, Param};

/// A real-time-safe boolean parameter.
pub struct BoolParam {
    id:      ParamId,
    name:    &'static str,
    default: bool,
    value:   AtomicBool,
    /// Custom labels for [false, true] states (e.g. ["OFF", "ON"]).
    labels:  [&'static str; 2],
}

impl BoolParam {
    pub fn new(id: ParamId, name: &'static str, default: bool) -> Self {
        Self { 
            id, 
            name, 
            default, 
            value: AtomicBool::new(default),
            labels: ["Off", "On"],
        }
    }

    /// Set custom labels for the toggle.
    pub fn labels(mut self, labels: [&'static str; 2]) -> Self {
        self.labels = labels;
        self
    }

    #[inline]
    pub fn value(&self) -> bool { self.value.load(Ordering::Relaxed) }

    #[inline]
    pub fn set(&self, v: bool) { self.value.store(v, Ordering::Release); }
}

impl Param for BoolParam {
    fn id(&self) -> ParamId { self.id }
    fn name(&self) -> &str { self.name }
    fn normalized(&self) -> f32 { if self.value() { 1.0 } else { 0.0 } }
    fn set_normalized(&self, v: f32) { self.set(v >= 0.5); }
    fn display(&self) -> String {
        let idx = if self.value() { 1 } else { 0 };
        self.labels[idx].to_string()
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
