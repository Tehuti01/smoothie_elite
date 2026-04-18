//! Geometric Silicon DSP
//! Harmonic audio algorithms following the sequence of growth.


use crate::silicon::geometry::{PHI, HARMONIC_PI, vector_norm};


/// Pythagorean Vector Synthesis (Point 303)
/// Spatial phase alignment utilizing the 3-4-5 triad.
#[inline(always)]
pub fn harmonic_vector_sum(a: f32, b: f32, phase: f32) -> f32 {
    let norm = vector_norm(a, b);
    // Align phase to the circular constant
    norm * (phase * HARMONIC_PI as f32).sin()
}


/// Phi-Interval Granular Distribution
/// Spacing grains according to the ratio of growth to avoid spectral overlap.
pub struct ManifoldGranulator {
    pub current_pos: f64,
    pub grain_size: usize,
}


impl ManifoldGranulator {
    /// Calculates the next grain onset.
    pub fn next_onset(&mut self) -> f64 {
        let delta = self.grain_size as f64 / PHI;
        self.current_pos += delta;
        self.current_pos
    }
}


/// Geometric Phase Vocoder (Point 307)
/// Manifold phase unwrapping using Pi harmonics.
pub struct SiliconVocoder {
    pub prev_phase: [f32; 1024],
}


impl SiliconVocoder {
    /// Unwraps the phase manifold for continuous synthesis.
    #[inline(always)]
    pub fn unwrap_manifold(&mut self, bin_idx: usize, current_phase: f32) -> f32 {
        let delta = current_phase - self.prev_phase[bin_idx];
        self.prev_phase[bin_idx] = current_phase;


        // Harmonic wrap logic
        if delta > HARMONIC_PI as f32 { delta - (2.0 * HARMONIC_PI as f32) }
        else if delta < -(HARMONIC_PI as f32) { delta + (2.0 * HARMONIC_PI as f32) }
        else { delta }
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
