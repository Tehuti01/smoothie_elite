//! Bitmask Component Tags & Structure of Arrays (SoA)
//! Implements high-speed entity capability sets using bitwise operations.

use core::ops::{BitAnd, BitOr, BitXor};

/// A 64-bit mask representing active component types on an Entity or Event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ComponentMask(pub u64);

impl ComponentMask {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn has(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn with(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn without(&self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
}

impl BitAnd for ComponentMask {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl BitOr for ComponentMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl BitXor for ComponentMask {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
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
