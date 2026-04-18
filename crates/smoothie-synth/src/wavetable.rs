//! Wavetable synthesis engine with band-limiting and morphing.

use std::f32::consts::PI;

/// Wavetable entry (band-limited wave).
pub struct Wavetable {
    samples: Vec<f32>,
    size: usize,
}

impl Wavetable {
    /// Create a sine wavetable.
    pub fn sine(size: usize) -> Self {
        let mut samples = vec![0.0; size];
        for i in 0..size {
            let phase = 2.0 * PI * (i as f32 / size as f32);
            samples[i] = phase.sin();
        }
        Self { samples, size }
    }

    /// Create a triangle wavetable (band-limited).
    pub fn triangle(size: usize) -> Self {
        let mut samples = vec![0.0; size];
        let max_harmonics = (size / 4).max(1);

        for i in 0..size {
            let phase = 2.0 * PI * (i as f32 / size as f32);
            let mut sample = 0.0;

            // Sum of band-limited harmonics (odd multiples only)
            for h in (1..max_harmonics).step_by(2) {
                let harmonic = h as f32;
                let amplitude = 8.0 / (PI * PI * harmonic * harmonic);
                sample += amplitude * (harmonic * phase).sin();
            }

            samples[i] = sample;
        }
        Self { samples, size }
    }

    /// Create a sawtooth wavetable (band-limited).
    pub fn sawtooth(size: usize) -> Self {
        let mut samples = vec![0.0; size];
        let max_harmonics = (size / 4).max(1);

        for i in 0..size {
            let phase = 2.0 * PI * (i as f32 / size as f32);
            let mut sample = 0.0;

            // Sum of band-limited harmonics (all multiples)
            for h in 1..max_harmonics {
                let harmonic = h as f32;
                let amplitude = 2.0 / (PI * harmonic);
                sample += amplitude * (harmonic * phase).sin();
            }

            samples[i] = sample;
        }
        Self { samples, size }
    }

    /// Create a square wavetable (band-limited).
    pub fn square(size: usize) -> Self {
        let mut samples = vec![0.0; size];
        let max_harmonics = (size / 4).max(1);

        for i in 0..size {
            let phase = 2.0 * PI * (i as f32 / size as f32);
            let mut sample = 0.0;

            // Sum of band-limited harmonics (odd multiples only)
            for h in (1..max_harmonics).step_by(2) {
                let harmonic = h as f32;
                let amplitude = 4.0 / (PI * harmonic);
                sample += amplitude * (harmonic * phase).sin();
            }

            samples[i] = sample;
        }
        Self { samples, size }
    }

    /// Sample the wavetable at a normalized position (0.0–1.0).
    pub fn sample(&self, pos: f32) -> f32 {
        let pos = pos % 1.0;
        let idx = (pos * self.size as f32) as usize;
        self.samples[idx.min(self.size - 1)]
    }

    /// Sample with linear interpolation for better quality.
    pub fn sample_interpolated(&self, pos: f32) -> f32 {
        let pos = pos % 1.0;
        let fpos = pos * self.size as f32;
        let idx = fpos as usize;
        let frac = fpos - idx as f32;

        let s0 = self.samples[idx.min(self.size - 1)];
        let s1 = self.samples[(idx + 1).min(self.size - 1)];

        s0 * (1.0 - frac) + s1 * frac
    }

    /// Get raw sample data.
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}

/// Wavetable synthesis engine.
pub struct WavetableEngine {
    tables: Vec<Wavetable>,
    current_table: usize,
    sample_rate: f32,
}

impl WavetableEngine {
    /// Create a new wavetable engine with all waveforms.
    pub fn new(sample_rate: f32, resolution: usize) -> Self {
        let tables = vec![
            Wavetable::sine(resolution),
            Wavetable::triangle(resolution),
            Wavetable::sawtooth(resolution),
            Wavetable::square(resolution),
        ];

        Self {
            tables,
            current_table: 0,
            sample_rate,
        }
    }

    /// Get the current wavetable.
    pub fn current(&self) -> &Wavetable {
        &self.tables[self.current_table.min(self.tables.len() - 1)]
    }

    /// Switch to a different waveform (0=sine, 1=tri, 2=saw, 3=square).
    pub fn set_waveform(&mut self, waveform: usize) {
        self.current_table = waveform;
    }

    /// Morph between two waveforms (0.0–1.0).
    pub fn morph(&self, t: f32, phase: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        let idx = t * (self.tables.len() as f32 - 1.0);
        let table_idx = idx as usize;
        let blend = idx - table_idx as f32;

        let t0 = self.tables[table_idx.min(self.tables.len() - 1)].sample_interpolated(phase);
        let t1 = self.tables[(table_idx + 1).min(self.tables.len() - 1)].sample_interpolated(phase);

        t0 * (1.0 - blend) + t1 * blend
    }
}

impl Default for WavetableEngine {
    fn default() -> Self {
        Self::new(44100.0, 2048)
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
