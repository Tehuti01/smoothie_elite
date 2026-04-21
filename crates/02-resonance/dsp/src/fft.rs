/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf9c5480e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/dsp/src/fft.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::{F_256, PI, TAU};
use smoothie_core::math::{cosine_approx, sine_approx};
use smoothie_core::primitives::Sample;

/// Complex number for FFT
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the Complex structure.
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

impl Complex {
    /// Create new complex number
    pub const fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    /// Magnitude (absolute value) - Silicon Optimized
    pub fn magnitude(&self) -> f32 {
        let mag_sq = self.real * self.real + self.imag * self.imag;
        fast_sqrt(mag_sq)
    }

    /// Phase angle (Approximate)
    pub fn phase(&self) -> f32 {
        atan2_approx(self.imag, self.real)
    }

    /// Add two complex numbers
    pub fn add(&self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    /// Subtract complex numbers
    pub fn sub(&self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    /// Multiply complex numbers
    pub fn mul(&self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Default for Complex {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// Technical implementation of the FFT structure.
pub struct FFT {
    size: usize,
    twiddle_real: [f32; F_256],
    twiddle_imag: [f32; F_256],
    bit_reverse: [usize; F_256],
}

impl FFT {
    /// Create new FFT of given size (must be power of 2, max 256)
    pub fn new(size: usize) -> Result<Self, &'static str> {
        if size > F_256 || size == 0 || (size & (size - 1)) != 0 {
            return Err("FFT size must be power of 2 and <= 256");
        }

        let mut fft = Self {
            size,
            twiddle_real: [0.0; F_256],
            twiddle_imag: [0.0; F_256],
            bit_reverse: [0; F_256],
        };

        // Precompute twiddle factors (W_N^k = e^(-2πik/N))
        for k in 0..size {
            let angle = -TAU * (k as f32) / (size as f32);
            fft.twiddle_real[k] = cosine_approx(angle);
            fft.twiddle_imag[k] = sine_approx(angle);
        }

        // Precompute bit reversal table
        for i in 0..size {
            fft.bit_reverse[i] = reverse_bits(i, size.trailing_zeros() as usize);
        }

        Ok(fft)
    }

    /// Compute FFT in-place (Absolute Flow Implementation)
    pub fn compute_in_place(&self, data: &mut [Complex]) -> Result<(), &'static str> {
        if data.len() != self.size {
            return Err("Data size must match FFT size");
        }

        // Bit-reversal permutation
        for i in 0..self.size {
            let j = self.bit_reverse[i];
            if i < j {
                data.swap(i, j);
            }
        }

        // Cooley-Tukey radix-2 FFT
        let mut step = 1;
        while step < self.size {
            let step2 = step * 2;
            for k in (0..self.size).step_by(step2) {
                for j in 0..step {
                    let twiddle_idx = (j * self.size) / step2;
                    let w = Complex {
                        real: self.twiddle_real[twiddle_idx],
                        imag: self.twiddle_imag[twiddle_idx],
                    };

                    let t = data[k + j + step].mul(w);
                    let u = data[k + j];

                    data[k + j] = u.add(t);
                    data[k + j + step] = u.sub(t);
                }
            }
            step = step2;
        }

        Ok(())
    }

    /// Inverse FFT in-place (Absolute Flow Implementation)
    pub fn inverse_in_place(&self, data: &mut [Complex]) -> Result<(), &'static str> {
        if data.len() != self.size {
            return Err("Data size must match FFT size");
        }

        // Conjugate input
        for val in data.iter_mut() {
            val.imag = -val.imag;
        }

        // Forward FFT
        self.compute_in_place(data)?;

        // Conjugate output and scale
        let scale = 1.0 / (self.size as f32);
        for val in data.iter_mut() {
            val.real *= scale;
            val.imag *= -scale;
        }

        Ok(())
    }

    /// Get FFT size
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Reverse bits in a number (Silicon optimized)
fn reverse_bits(mut n: usize, width: usize) -> usize {
    let mut result = 0;
    for _ in 0..width {
        result = (result << 1) | (n & 1);
        n >>= 1;
    }
    result
}

/// Approximation of atan2(y, x)
fn atan2_approx(y: f32, x: f32) -> f32 {
    if x > 0.0 {
        atan_approx(y / x)
    } else if x < 0.0 && y >= 0.0 {
        atan_approx(y / x) + PI
    } else if x < 0.0 && y < 0.0 {
        atan_approx(y / x) - PI
    } else if x == 0.0 && y > 0.0 {
        PI / 2.0
    } else if x == 0.0 && y < 0.0 {
        -PI / 2.0
    } else {
        0.0
    }
}

/// Approximation of atan(x) using Taylor series
fn atan_approx(x: f32) -> f32 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    x - x3 / 3.0 + x5 / 5.0 - x7 / 7.0
}

/// Fast square root approximation (Newton-Raphson)
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut result = x / 2.0;
    for _ in 0..3 {
        result = (result + x / result) / 2.0;
    }
    result
}
