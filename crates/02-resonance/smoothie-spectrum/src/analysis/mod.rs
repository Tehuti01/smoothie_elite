/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xefc84049 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/analysis/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::fft::{FftProcessor, FftSize};
use super::window::{Window, WindowFunction};
use alloc::vec;
/// Spectral Analysis State Machine — overlap-add hop processor.
use alloc::vec::Vec;
use smoothie_core::math::FloatExt;
use smoothie_math::complex::Complex32;

/// Technical implementation of the SpectrumFrame structure.
pub struct SpectrumFrame {
    pub magnitudes: Vec<f32>,
    pub n: usize,
}

/// Technical implementation of the SpectrumAnalyzer structure.
pub struct SpectrumAnalyzer {
    fft: FftProcessor,
    window: Window,
    input_ring: Vec<f32>,
    fft_buffer: Vec<Complex32>,
    mag_buffer: Vec<f32>,
    smoothed: Vec<f32>,
    write_pos: usize,
    hop_size: usize,
    hop_counter: usize,
    pub smoothing: f32,
    n: usize,
}

impl SpectrumAnalyzer {
    /// Initializes a new instance of the associated type.
    pub fn new(size: FftSize, hop_fraction: usize, smoothing: f32) -> Self {
        let n = size.n();
        let hop_size = n / hop_fraction.max(1);
        Self {
            fft: FftProcessor::new(size),
            window: Window::new(WindowFunction::BlackmanHarris, n),
            input_ring: vec![0.0; n],
            fft_buffer: vec![Complex32::default(); n],
            mag_buffer: vec![0.0; n / 2],
            smoothed: vec![0.0; n / 2],
            write_pos: 0,
            hop_size,
            hop_counter: 0,
            smoothing: smoothing.clamp(0.0, 0.9999),
            n,
        }
    }

    /// Technical implementation of the push_sample logic.
    pub fn push_sample(&mut self, sample: f32) -> Option<&[f32]> {
        self.input_ring[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.n;
        self.hop_counter += 1;

        if self.hop_counter >= self.hop_size {
            self.hop_counter = 0;
            self.run_fft();
            return Some(&self.smoothed);
        }
        None
    }

    /// Technical implementation of the run_fft logic.
    fn run_fft(&mut self) {
        let n = self.n;
        for i in 0..n {
            let src_idx = (self.write_pos + i) % n;
            let windowed = self.input_ring[src_idx] * self.window.coefficients[i];
            self.fft_buffer[i] = Complex32::new(windowed, 0.0);
        }
        self.fft.forward(&mut self.fft_buffer);
        FftProcessor::compute_magnitudes(&self.fft_buffer, &mut self.mag_buffer);

        let s = self.smoothing;
        for (out, &new_val) in self.smoothed.iter_mut().zip(self.mag_buffer.iter()) {
            *out = *out * s + new_val * (1.0 - s);
        }
    }

    /// Technical implementation of the bins logic.
    pub fn bins(&self) -> &[f32] {
        &self.smoothed
    }
}
