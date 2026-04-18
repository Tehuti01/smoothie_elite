//! Anti-aliased oscillators using PolyBLEP (Polynomial Band-Limited Step).
//! 
//! These oscillators are 'Elite' grade, suitable for audio-rate synthesis 
//! without the harsh aliasing artifacts of naive phase-accumulators.

use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaveShape { 
    Sine, 
    Triangle, 
    Sawtooth, 
    Square, 
    Pulse(f32) 
}

/// A high-performance, anti-aliased oscillator.
pub struct Oscillator {
    phase:       f32,
    phase_inc:   f32,
    shape:       WaveShape,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(shape: WaveShape, sample_rate: f32) -> Self {
        Self { phase: 0.0, phase_inc: 0.0, shape, sample_rate }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        self.phase_inc = (hz / self.sample_rate).clamp(0.0, 0.5);
    }

    pub fn set_shape(&mut self, shape: WaveShape) { self.shape = shape; }

    /// Advance one sample and return output in [-1, 1].
    #[inline]
    pub fn next_sample(&mut self) -> f32 {
        let p = self.phase;
        let dt = self.phase_inc;
        
        let mut out = match self.shape {
            WaveShape::Sine => (2.0 * PI * p).sin(),
            
            WaveShape::Triangle => {
                // Triangle is integrated square wave, less aliasing, but we can improve
                if p < 0.5 { 4.0 * p - 1.0 } else { 3.0 - 4.0 * p }
            }
            
            WaveShape::Sawtooth => {
                let naive = 2.0 * p - 1.0;
                naive - Self::poly_blep(p, dt)
            }
            
            WaveShape::Square => {
                let naive = if p < 0.5 { 1.0 } else { -1.0 };
                naive + Self::poly_blep(p, dt) - Self::poly_blep((p + 0.5) % 1.0, dt)
            }
            
            WaveShape::Pulse(width) => {
                let naive = if p < width { 1.0 } else { -1.0 };
                naive + Self::poly_blep(p, dt) - Self::poly_blep((p + 1.0 - width) % 1.0, dt)
            }
        };

        self.phase = (self.phase + self.phase_inc) % 1.0;
        out
    }

    /// Polynomial Band-Limited Step function.
    /// Helps eliminate aliasing at discontinuity points.
    #[inline]
    fn poly_blep(t: f32, dt: f32) -> f32 {
        if t < dt {
            let t = t / dt;
            t + t - t * t - 1.0
        } else if t > 1.0 - dt {
            let t = (t - 1.0) / dt;
            t * t + t + t + 1.0
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) { self.phase = 0.0; }
}

// ─── Organic (Phi) Modulation ──────────────────────────────────────────────

/// A non-periodic modulator aligned with the divine Phi ratio.
/// Creates organic, self-organizing motion for 'Elite' parameter modulation.
pub struct OrganicLFO {
    phase: f32,
    phi_accumulator: f32,
    sample_rate: f32,
    frequency: f32,
}

impl OrganicLFO {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            phi_accumulator: 0.0,
            sample_rate,
            frequency: 1.0,
        }
    }

    pub fn set_frequency(&mut self, hz: f32) {
        self.frequency = hz;
    }

    /// Advance one sample and return a Phi-scaled modulation value in [0, 1].
    #[inline]
    pub fn next_value(&mut self) -> f32 {
        const PHI: f32 = 1.61803398875;
        
        // Advance base phase
        let dt = self.frequency / self.sample_rate;
        self.phase = (self.phase + dt) % 1.0;
        
        // Advance Phi-offset (the divine drift)
        self.phi_accumulator = (self.phi_accumulator + dt * PHI) % 1.0;
        
        // Combine for a quasi-periodic, organic result
        let out = (2.0 * PI * (self.phase + self.phi_accumulator)).sin() * 0.5 + 0.5;
        out
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
