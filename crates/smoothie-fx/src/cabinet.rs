//! Cabinet IR simulator — simple FIR convolution with short cabinet impulses.
//! For full convolution, use smoothie-fx's ConvolutionEngine.
//! This module provides built-in short IRs for quick guitar cabinet emulation.

/// Very short (32-tap) cabinet FIR filter. Allocates at init, zero-alloc at runtime.
pub struct CabinetSim {
    ir:      Vec<f32>,
    buf:     Vec<f32>,
    ptr:     usize,
    /// Wet/dry [0, 1].
    pub mix: f32,
}

impl CabinetSim {
    /// Create with a custom IR.
    pub fn from_ir(ir: Vec<f32>) -> Self {
        let len = ir.len();
        Self { ir, buf: vec![0.0; len], ptr: 0, mix: 1.0 }
    }

    /// Built-in 1x12 combo cabinet character (32-tap approximation).
    pub fn combo_1x12(sample_rate: f32) -> Self {
        let ir = Self::generate_cabinet_ir(32, sample_rate, 180.0, 6000.0, 0.3);
        Self::from_ir(ir)
    }

    /// Built-in 4x12 stack cabinet character (64-tap approximation).
    pub fn stack_4x12(sample_rate: f32) -> Self {
        let ir = Self::generate_cabinet_ir(64, sample_rate, 120.0, 5000.0, 0.5);
        Self::from_ir(ir)
    }

    /// Synthetic IR: bandpass-shaped FIR using windowed sinc.
    fn generate_cabinet_ir(taps: usize, sr: f32, lo: f32, hi: f32, resonance: f32) -> Vec<f32> {
        let mut ir = vec![0.0f32; taps];
        let center = taps as f32 / 2.0;
        for (i, v) in ir.iter_mut().enumerate() {
            let n = i as f32 - center;
            // Bandpass = highpass - lowpass
            let hp = if n == 0.0 { 1.0 - 2.0 * lo / sr }
                     else { -(2.0 * lo / sr * (std::f32::consts::PI * n * 2.0 * lo / sr).sin() / (std::f32::consts::PI * n)) };
            let lp = if n == 0.0 { 2.0 * hi / sr }
                     else { 2.0 * hi / sr * (std::f32::consts::PI * n * 2.0 * hi / sr).sinc() };
            // Hann window
            let w = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (taps - 1) as f32).cos());
            *v = (lp - hp) * w * (1.0 + resonance);
        }
        // Normalize
        let sum: f32 = ir.iter().map(|x| x.abs()).sum();
        if sum > 0.0 { for v in ir.iter_mut() { *v /= sum; } }
        ir
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.buf[self.ptr] = x;
        let n = self.ir.len();
        let mut acc = 0.0f32;
        for k in 0..n {
            acc += self.ir[k] * self.buf[(self.ptr + n - k) % n];
        }
        self.ptr = (self.ptr + 1) % n;
        x * (1.0 - self.mix) + acc * self.mix
    }
}

trait Sinc {
    fn sinc(self) -> Self;
}
impl Sinc for f32 {
    fn sinc(self) -> f32 {
        if self.abs() < 1e-6 { 1.0 }
        else { (std::f32::consts::PI * self).sin() / (std::f32::consts::PI * self) }
    }
}
