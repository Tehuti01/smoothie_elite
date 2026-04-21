---
id: fi-2458-zero-cost-wrapper.rs
category: f-01-secbrain
---

/// 🏗️ Example: Zero-Cost Abstraction
/// High-level logic with zero overhead.
pub struct Frequency(f64);

impl Frequency {
    #[inline(always)]
    pub fn to_radians(&self) -> f64 {
        self.0 * 2.0 * std::f64::consts::PI
    }
}
// Compiles down to the same ASM as a raw f64 multiplication.
