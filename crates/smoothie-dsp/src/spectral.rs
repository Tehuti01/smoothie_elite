use realfft::{RealFftPlanner, RealToComplexfft, ComplexToRealfft};
use std::sync::Arc;

/// A high-performance 'Elite' spectral processor for real-time frequency-domain manipulation.
/// Handles windowing, FFT/IFFT, and overlap-add logic with zero allocation in the process loop.
pub struct SpectralProcessor {
    fft_size: usize,
    hop_size: usize,
    planner: RealFftPlanner<f64>,
    r2c: Arc<dyn RealToComplexfft<f64>>,
    c2r: Arc<dyn ComplexToRealfft<f64>>,
    
    // Internal buffers
    window: Vec<f64>,
    input_buffer: Vec<f64>,
    output_buffer: Vec<f64>,
    fft_in: Vec<f64>,
    fft_out: Vec<num_complex::Complex<f64>>,
    olap_add: Vec<f64>,
    
    write_pos: usize,
    read_pos: usize,
}

impl SpectralProcessor {
    pub fn new(fft_size: usize, hop_size: usize) -> Self {
        let mut planner = RealFftPlanner::<f64>::new();
        let r2c = planner.plan_fft_forward(fft_size);
        let c2r = planner.plan_fft_inverse(fft_size);
        
        // Hann window
        let window: Vec<f64> = (0..fft_size)
            .map(|i| 0.5 * (1.0 - (2.0 * std::f64::consts::PI * i as f64 / (fft_size - 1) as f64).cos()))
            .collect();

        Self {
            fft_size,
            hop_size,
            planner,
            r2c,
            c2r,
            window,
            input_buffer: vec![0.0; fft_size],
            output_buffer: vec![0.0; fft_size],
            fft_in: vec![0.0; fft_size],
            fft_out: vec![num_complex::Complex::new(0.0, 0.0); fft_size / 2 + 1],
            olap_add: vec![0.0; fft_size * 2],
            write_pos: 0,
            read_pos: 0,
        }
    }

    /// Process a block of samples. 
    /// `manipulator` is a closure that receives the frequency bins and should modify them in place.
    pub fn process<F>(&mut self, input: &[f64], output: &mut [f64], mut manipulator: F)
    where
        F: FnMut(&mut [num_complex::Complex<f64>]),
    {
        for i in 0..input.len() {
            // 1. Fill input buffer
            self.input_buffer[self.write_pos] = input[i];
            
            // 2. Extract output from overlap-add buffer
            output[i] = self.olap_add[self.read_pos];
            self.olap_add[self.read_pos] = 0.0; // Clear for next round
            
            self.write_pos += 1;
            self.read_pos = (self.read_pos + 1) % self.olap_add.len();

            // 3. If we have a hop's worth of data, run the FFT
            if self.write_pos >= self.hop_size {
                self.run_cycle(&mut manipulator);
                self.write_pos = 0;
            }
        }
    }

    fn run_cycle<F>(&mut self, manipulator: &mut F)
    where
        F: FnMut(&mut [num_complex::Complex<f64>]),
    {
        // A. Window input
        for i in 0..self.fft_size {
            self.fft_in[i] = self.input_buffer[i] * self.window[i];
        }

        // B. Forward FFT
        let _ = self.r2c.process(&mut self.fft_in, &mut self.fft_out);

        // C. Spectral Manipulation
        manipulator(&mut self.fft_out);

        // D. Inverse FFT
        let _ = self.c2r.process(&mut self.fft_out, &mut self.fft_in);

        // E. Window output and overlap-add
        for i in 0..self.fft_size {
            let out_sample = (self.fft_in[i] / self.fft_size as f64) * self.window[i];
            let olap_idx = (self.read_pos + i) % self.olap_add.len();
            self.olap_add[olap_idx] += out_sample;
        }

        // F. Shift input buffer by hop_size
        self.input_buffer.copy_within(self.hop_size..self.fft_size, 0);
    }
}
