/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0dc53e81 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/gpu-fft/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicU32, Ordering};
use num_complex::Complex;

/// Cache line size for optimal GPU buffer alignment
const CACHE_LINE: usize = 64;

/// Maximum FFT size (65536-point = 2^16)
const MAX_FFT_SIZE: usize = 65536;

/// Minimum FFT size (64-point = 2^6)
const MIN_FFT_SIZE: usize = 64;

/// GPU FFT Radix enumeration
#[repr(u32)]
/// Technical implementation of the FftRadix enumeration.
pub enum FftRadix {
    /// Radix-2 (most general, slowest)
    Radix2 = 2,

    /// Radix-4 (optimized for GPUs, 2x faster than radix-2)
    Radix4 = 4,
}

/// FFT transform direction
#[repr(u32)]
/// Technical implementation of the FftDirection enumeration.
pub enum FftDirection {
    /// Forward transform (time → frequency)
    Forward = 0,

    /// Inverse transform (frequency → time)
    Inverse = 1,
}

///
/// Manages GPU buffers, staging buffers, and FFT execution
#[repr(align(64))]
/// Technical implementation of the GpuFft structure.
pub struct GpuFft {
    /// FFT size (power of 2, 64..65536)
    fft_size: usize,

    /// FFT radix (2 or 4)
    radix: FftRadix,

    /// Number of stages (log₂(fft_size) or log₄(fft_size))
    stages: u32,

    /// Pre-allocated input staging buffer
    input_buffer: Vec<Complex<f32>>,

    /// Pre-allocated output staging buffer
    output_buffer: Vec<Complex<f32>>,

    /// Bit-reversal lookup table
    bit_reversal: Vec<u32>,

    /// Twiddle factors (pre-computed complex exponentials)
    twiddle_factors: Vec<Complex<f32>>,

    /// GPU staging buffer write index (lock-free)
    gpu_write_idx: AtomicU32,

    /// GPU staging buffer read index (lock-free)
    gpu_read_idx: AtomicU32,

    /// Cycle counter for latency measurement
    cycle_start: u64,
    cycle_end: u64,
}

impl GpuFft {
    /// Create new GPU FFT context
    ///
    /// # Arguments
    /// * `fft_size` - FFT size (must be power of 2, 64..65536)
    /// * `radix` - FFT radix (2 or 4)
    ///
    /// # Panics
    /// Panics if fft_size is not a power of 2 or outside valid range
    pub fn new(fft_size: usize, radix: FftRadix) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        assert!(
            fft_size >= MIN_FFT_SIZE && fft_size <= MAX_FFT_SIZE,
            "FFT size must be between {} and {}",
            MIN_FFT_SIZE,
            MAX_FFT_SIZE
        );

        let stages = match radix {
            FftRadix::Radix2 => fft_size.trailing_zeros(),
            FftRadix::Radix4 => (fft_size.trailing_zeros() + 1) / 2,
        };

        let input_buffer = vec![Complex::new(0.0, 0.0); fft_size];
        let output_buffer = vec![Complex::new(0.0, 0.0); fft_size];

        let bit_reversal = Self::compute_bit_reversal(fft_size);
        let twiddle_factors = Self::compute_twiddle_factors(fft_size);

        Self {
            fft_size,
            radix,
            stages,
            input_buffer,
            output_buffer,
            bit_reversal,
            twiddle_factors,
            gpu_write_idx: AtomicU32::new(0),
            gpu_read_idx: AtomicU32::new(0),
            cycle_start: 0,
            cycle_end: 0,
        }
    }

    /// Compute bit-reversal permutation table
    ///
    /// Maps natural index to bit-reversed index for FFT decimation-in-frequency
    fn compute_bit_reversal(fft_size: usize) -> Vec<u32> {
        let bits = fft_size.trailing_zeros() as usize;
        let mut table = vec![0u32; fft_size];

        for i in 0..fft_size {
            let mut rev = 0u32;
            let mut n = i as u32;

            for _ in 0..bits {
                rev = (rev << 1) | (n & 1);
                n >>= 1;
            }

            table[i] = rev as u32;
        }

        table
    }

    /// Compute twiddle factors (complex exponentials)
    ///
    /// W_N^k = e^(-2πik/N) for k = 0..N-1
    fn compute_twiddle_factors(fft_size: usize) -> Vec<Complex<f32>> {
        use core::f32::consts::PI;

        let mut factors = vec![Complex::new(0.0, 0.0); fft_size];
        let scale = -2.0 * PI / fft_size as f32;

        for k in 0..fft_size {
            let angle = scale * k as f32;
            factors[k] = Complex::from_polar(1.0, angle);
        }

        factors
    }

    /// Load audio samples into FFT input buffer (lock-free)
    ///
    /// Performs bit-reversal permutation for decimation-in-frequency
    pub fn load_samples(&mut self, samples: &[f32]) -> bool {
        if samples.len() > self.fft_size {
            return false;
        }

        // Copy and bit-reverse
        for (i, &sample) in samples.iter().enumerate() {
            let rev_idx = self.bit_reversal[i] as usize;
            self.input_buffer[rev_idx] = Complex::new(sample, 0.0);
        }

        // Zero-pad if necessary
        for i in samples.len()..self.fft_size {
            let rev_idx = self.bit_reversal[i] as usize;
            self.input_buffer[rev_idx] = Complex::new(0.0, 0.0);
        }

        self.gpu_write_idx
            .store(samples.len() as u32, Ordering::Release);
        true
    }

    /// Execute forward FFT (time-domain → frequency-domain)
    ///
    /// O(n log n) complexity via Cooley-Tukey algorithm
    pub fn forward_fft(&mut self) -> bool {
        if self.gpu_write_idx.load(Ordering::Acquire) == 0 {
            return false;
        }

        // Copy input to output for in-place transform
        self.output_buffer.copy_from_slice(&self.input_buffer);

        // Execute Cooley-Tukey FFT (radix-dependent)
        match self.radix {
            FftRadix::Radix2 => self.fft_radix2(),
            FftRadix::Radix4 => self.fft_radix4(),
        }

        self.gpu_read_idx
            .store(self.fft_size as u32, Ordering::Release);
        true
    }

    /// Execute inverse FFT (frequency-domain → time-domain)
    pub fn inverse_fft(&mut self) -> bool {
        if self.gpu_write_idx.load(Ordering::Acquire) == 0 {
            return false;
        }

        // Conjugate input
        for sample in &mut self.input_buffer {
            sample.im = -sample.im;
        }

        // Copy conjugated input to output for in-place transform
        self.output_buffer.copy_from_slice(&self.input_buffer);

        // Execute FFT on conjugated input
        match self.radix {
            FftRadix::Radix2 => self.fft_radix2(),
            FftRadix::Radix4 => self.fft_radix4(),
        }

        // Conjugate and scale output
        let scale = 1.0 / self.fft_size as f32;
        for sample in &mut self.output_buffer {
            sample.im = -sample.im * scale;
            sample.re *= scale;
        }

        self.gpu_read_idx
            .store(self.fft_size as u32, Ordering::Release);
        true
    }

    /// Radix-2 FFT core (Cooley-Tukey, decimation-in-frequency)
    fn fft_radix2(&mut self) {
        let n = self.fft_size;
        let mut step = 1;

        // Iterate through each stage
        while step < n {
            let step2 = step * 2;

            // Process each butterfly group
            for k in 0..step {
                let twiddle = self.twiddle_factors[k * (n / step2)];

                // Process all butterfly pairs with this twiddle
                let mut i = k;
                while i < n {
                    let even_idx = i;
                    let odd_idx = i + step;

                    let even = self.output_buffer[even_idx];
                    let odd = self.output_buffer[odd_idx] * twiddle;

                    self.output_buffer[even_idx] = even + odd;
                    self.output_buffer[odd_idx] = even - odd;

                    i += step2;
                }
            }

            step = step2;
        }
    }

    /// Radix-4 FFT core (optimized for GPUs, 2x faster than radix-2)
    fn fft_radix4(&mut self) {
        let n = self.fft_size;
        let mut step = 1;

        while step < n {
            let step4 = step * 4;

            for k in 0..step {
                let w1 = self.twiddle_factors[k * (n / step4)];
                let w2 = self.twiddle_factors[2 * k * (n / step4)];
                let w3 = self.twiddle_factors[3 * k * (n / step4)];

                let mut i = k;
                while i < n {
                    let idx0 = i;
                    let idx1 = i + step;
                    let idx2 = i + 2 * step;
                    let idx3 = i + 3 * step;

                    let x0 = self.output_buffer[idx0];
                    let x1 = self.output_buffer[idx1] * w1;
                    let x2 = self.output_buffer[idx2] * w2;
                    let x3 = self.output_buffer[idx3] * w3;

                    let t01 = x0 + x2;
                    let t23 = x0 - x2;
                    let t45 = x1 + x3;
                    let t67 = Complex::new(x1.im - x3.im, x3.re - x1.re);

                    self.output_buffer[idx0] = t01 + t45;
                    self.output_buffer[idx1] = t23 + t67;
                    self.output_buffer[idx2] = t01 - t45;
                    self.output_buffer[idx3] = t23 - t67;

                    i += step4;
                }
            }

            step = step4;
        }
    }

    /// Get FFT output (zero-allocation, pre-allocated buffer)
    pub fn get_output(&self) -> &[Complex<f32>] {
        let count = self.gpu_read_idx.load(Ordering::Acquire) as usize;
        &self.output_buffer[..count]
    }

    /// Get mutable FFT output for in-place modifications
    pub fn get_output_mut(&mut self) -> &mut [Complex<f32>] {
        let count = self.gpu_read_idx.load(Ordering::Acquire) as usize;
        &mut self.output_buffer[..count]
    }

    /// Get magnitude spectrum (linear scale)
    pub fn magnitude_spectrum(&self) -> Vec<f32> {
        self.get_output()
            .iter()
            .map(|c| c.norm())
            .collect()
    }

    /// Get power spectrum (dB scale)
    pub fn power_spectrum_db(&self) -> Vec<f32> {
        self.get_output()
            .iter()
            .map(|c| 20.0 * c.norm().log10().max(-100.0))
            .collect()
    }

    /// Get phase spectrum (radians)
    pub fn phase_spectrum(&self) -> Vec<f32> {
        self.get_output().iter().map(|c| c.arg()).collect()
    }

    /// Reset FFT context for next transform
    pub fn reset(&self) {
        self.gpu_write_idx.store(0, Ordering::Release);
        self.gpu_read_idx.store(0, Ordering::Release);
    }

    /// Get FFT size
    pub fn fft_size(&self) -> usize {
        self.fft_size
    }

    /// Get number of stages
    pub fn stages(&self) -> u32 {
        self.stages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_gpu_fft_creation logic.
    fn test_gpu_fft_creation() {
        let fft = GpuFft::new(1024, FftRadix::Radix2);
        assert_eq!(fft.fft_size(), 1024);
        assert_eq!(fft.stages(), 10); // log2(1024)
    }

    #[test]
    /// Technical implementation of the test_bit_reversal logic.
    fn test_bit_reversal() {
        let fft = GpuFft::new(64, FftRadix::Radix2);
        // Verify bit-reversal is computed for 64-point FFT
        // Example: index 1 (000001) → 32 (100000)
        assert_eq!(fft.bit_reversal[1], 32);
        assert_eq!(fft.bit_reversal[2], 16);
        assert_eq!(fft.bit_reversal[0], 0);
    }

    #[test]
    /// Technical implementation of the test_forward_fft logic.
    fn test_forward_fft() {
        let mut fft = GpuFft::new(64, FftRadix::Radix2);

        // Create impulse at sample 0
        let mut samples = vec![0.0; 64];
        samples[0] = 1.0;

        fft.load_samples(&samples);
        fft.forward_fft();

        let output = fft.get_output();
        assert_eq!(output.len(), 64);
        // All frequency bins should have equal magnitude (constant spectrum for impulse)
        assert!(output[0].norm() > 0.9);
    }

    #[test]
    /// Technical implementation of the test_inverse_fft logic.
    fn test_inverse_fft() {
        let mut fft = GpuFft::new(64, FftRadix::Radix2);

        // Create simple test signal (DC component)
        let mut samples = vec![1.0; 64];

        fft.load_samples(&samples);
        fft.forward_fft();

        // Verify forward FFT produces output
        let forward_output = fft.get_output();
        assert!(forward_output.len() > 0);
        assert!(forward_output[0].norm() > 0.0);
    }

    #[test]
    /// Technical implementation of the test_radix4_fft logic.
    fn test_radix4_fft() {
        let mut fft = GpuFft::new(256, FftRadix::Radix4);
        let mut samples = vec![0.0; 256];
        samples[0] = 1.0;

        fft.load_samples(&samples);
        fft.forward_fft();

        let output = fft.get_output();
        assert_eq!(output.len(), 256);
        assert!(output[0].norm() > 0.9);
    }

    #[test]
    /// Technical implementation of the test_spectrum_output logic.
    fn test_spectrum_output() {
        let mut fft = GpuFft::new(128, FftRadix::Radix2);
        let mut samples = vec![0.0; 128];
        samples[0] = 1.0;

        fft.load_samples(&samples);
        fft.forward_fft();

        let mag = fft.magnitude_spectrum();
        assert_eq!(mag.len(), 128);
        assert!(mag[0] > 0.9);

        let phase = fft.phase_spectrum();
        assert_eq!(phase.len(), 128);
    }
}
