/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x550c0909 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/fft/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// Twiddle factors are pre-baked at construction using `sine_approx`.
use alloc::vec::Vec;
use smoothie_core::math::sine_approx;
use smoothie_math::complex::Complex32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the FftSize enumeration.
pub enum FftSize {
    N64 = 64,
    N128 = 128,
    N256 = 256,
    N512 = 512,
    N1024 = 1024,
    N2048 = 2048,
    N4096 = 4096,
    N8192 = 8192,
    N16384 = 16384,
    N32768 = 32768,
}

impl FftSize {
    /// Technical implementation of the n logic.
    pub fn n(&self) -> usize {
        *self as usize
    }
    /// Technical implementation of the log2_n logic.
    pub fn log2_n(&self) -> u32 {
        (self.n() as u32).trailing_zeros()
    }
}

/// Technical implementation of the FftProcessor structure.
pub struct FftProcessor {
    size: FftSize,
    twiddles: Vec<Complex32>,
    bit_rev: Vec<usize>,
    scratch: Vec<Complex32>,
}

impl FftProcessor {
    /// Initializes a new instance of the associated type.
    pub fn new(size: FftSize) -> Self {
        let n = size.n();
        let half_n = n / 2;

        let mut twiddles = Vec::with_capacity(half_n);
        for k in 0..half_n {
            // Phase = -k/N, normalised. Negate imaginary for forward DFT.
            let phase = -(k as f32) / (n as f32);
            let cos_val = sine_approx(frac(phase + 0.25));
            let sin_val = sine_approx(frac(phase));
            twiddles.push(Complex32::new(cos_val, sin_val));
        }

        let log2 = size.log2_n() as usize;
        let mut bit_rev = vec![0usize; n];
        for i in 0..n {
            let mut rev = 0usize;
            let mut x = i;
            for _ in 0..log2 {
                rev = (rev << 1) | (x & 1);
                x >>= 1;
            }
            bit_rev[i] = rev;
        }

        Self {
            size,
            twiddles,
            bit_rev,
            scratch: vec![Complex32::default(); n],
        }
    }

    /// Technical implementation of the forward logic.
    pub fn forward(&mut self, buffer: &mut [Complex32]) {
        debug_assert_eq!(buffer.len(), self.size.n());
        self.bit_reverse_copy(buffer);
        self.butterfly_passes(buffer, false);
    }

    /// Technical implementation of the inverse logic.
    pub fn inverse(&mut self, buffer: &mut [Complex32]) {
        debug_assert_eq!(buffer.len(), self.size.n());
        self.bit_reverse_copy(buffer);
        self.butterfly_passes(buffer, true);
        let scale = 1.0 / self.size.n() as f32;
        for c in buffer.iter_mut() {
            c.re *= scale;
            c.im *= scale;
        }
    }

    /// Technical implementation of the bit_reverse_copy logic.
    fn bit_reverse_copy(&mut self, buffer: &mut [Complex32]) {
        let _n = self.size.n();
        for (i, &rev_i) in self.bit_rev.iter().enumerate() {
            self.scratch[rev_i] = buffer[i];
        }
        buffer.copy_from_slice(&self.scratch);
    }

    /// Technical implementation of the butterfly_passes logic.
    fn butterfly_passes(&self, buffer: &mut [Complex32], inverse: bool) {
        let n = self.size.n();
        let log2 = self.size.log2_n() as usize;
        let mut step = 1usize;

        for _ in 0..log2 {
            let half_step = step;
            step <<= 1;
            let twiddle_stride = n / step;
            let mut k = 0;
            while k < n {
                for j in 0..half_step {
                    let u = buffer[k + j];
                    let tw = &self.twiddles[j * twiddle_stride];
                    let tw_v = if inverse {
                        Complex32::new(tw.re, -tw.im)
                    } else {
                        *tw
                    };
                    let t = tw_v * buffer[k + j + half_step];
                    buffer[k + j] = u + t;
                    buffer[k + j + half_step] = u - t;
                }
                k += step;
            }
        }
    }

    /// Technical implementation of the compute_magnitudes logic.
    pub fn compute_magnitudes(buffer: &[Complex32], magnitudes: &mut [f32]) {
        let half = buffer.len() / 2;
        debug_assert_eq!(magnitudes.len(), half);
        for (i, bin) in buffer.iter().take(half).enumerate() {
            magnitudes[i] = fast_sqrt_complex(bin);
        }
    }
}

#[inline(always)]
/// Technical implementation of the fast_sqrt_complex logic.
fn fast_sqrt_complex(c: &Complex32) -> f32 {
    let m2 = c.re * c.re + c.im * c.im;
    if m2 == 0.0 {
        return 0.0;
    }
    let xhalf = 0.5 * m2;
    let mut i = m2.to_bits();
    i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    m2 * (y * (1.5 - xhalf * y * y))
}

#[inline(always)]
/// Technical implementation of the frac logic.
fn frac(x: f32) -> f32 {
    let i = x as i32;
    let f = x - if (i as f32) > x {
        i as f32 - 1.0
    } else {
        i as f32
    };
    if f < 0.0 {
        f + 1.0
    } else {
        f
    }
}
