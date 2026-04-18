//! Trig utilities and phasor helpers.

use std::f32::consts::{PI, TAU};

/// A phasor that increments by `phase_inc` per sample. Wraps [0, 1).
pub struct Phasor {
    pub phase:     f32,
    pub phase_inc: f32,
}

impl Phasor {
    pub fn new(hz: f32, sample_rate: f32) -> Self {
        Self {
            phase:     0.0,
            phase_inc: hz / sample_rate,
        }
    }

    pub fn set_freq(&mut self, hz: f32, sample_rate: f32) {
        self.phase_inc = hz / sample_rate;
    }

    #[inline]
    pub fn tick(&mut self) -> f32 {
        let p = self.phase;
        self.phase = (self.phase + self.phase_inc) % 1.0;
        p
    }

    /// Current sine output.
    #[inline]
    pub fn sin(&self) -> f32 { (self.phase * TAU).sin() }

    /// Current cosine output.
    #[inline]
    pub fn cos(&self) -> f32 { (self.phase * TAU).cos() }
}

/// Bandlimited sawtooth via polynomial BLEP correction.
/// Typical usage: call `blep_saw` once per sample and add the correction.
#[inline]
pub fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let t = t / dt;
        2.0 * t - t * t - 1.0
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        t * t + 2.0 * t + 1.0
    } else {
        0.0
    }
}

/// Bandlimited square from saw via BLEP.
#[inline]
pub fn poly_blep_square(phase: f32, dt: f32, pw: f32) -> f32 {
    let mut out = if phase < pw { 1.0 } else { -1.0 };
    out += poly_blep(phase, dt);
    out -= poly_blep((phase + 1.0 - pw) % 1.0, dt);
    out
}

/// Sinc function: sin(πx)/(πx), safe at x=0.
#[inline]
pub fn sinc(x: f32) -> f32 {
    if x.abs() < 1e-6 { 1.0 }
    else { (PI * x).sin() / (PI * x) }
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
