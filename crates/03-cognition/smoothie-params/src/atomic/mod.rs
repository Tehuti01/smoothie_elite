/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb1faf0df | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/atomic/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicU32, Ordering};

/// Technical implementation of the AtomicParameter structure.
pub struct AtomicParameter {
    value: AtomicU32,
}

impl AtomicParameter {
    /// Initializes a new instance of the associated type.
    pub fn new(value: f32) -> Self {
        Self {
            value: AtomicU32::new(value.to_bits()),
        }
    }
    /// Technical implementation of the load logic.
    pub fn load(&self) -> f32 {
        f32::from_bits(self.value.load(Ordering::Acquire))
    }
    /// Technical implementation of the store logic.
    pub fn store(&self, value: f32) {
        self.value.store(value.to_bits(), Ordering::Release);
    }
    /// Technical implementation of the get_normalized logic.
    pub fn get_normalized(&self) -> f32 {
        self.load() // Simplified for now
    }
    /// Technical implementation of the set_normalized logic.
    pub fn set_normalized(&self, value: f32) {
        self.store(value);
    }
}
