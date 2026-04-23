/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6d7463bc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/rnn/gru.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use wide::*;

/// Technical implementation of the GRULayer structure.
/// Weights are stored in row-major order: [hidden_size, input_size] or [hidden_size, hidden_size]
pub struct GRULayer {
    pub input_size: usize,
    pub hidden_size: usize,

    // Weights Update Gate
    w_ir: Vec<f32>,
    w_hr: Vec<f32>,
    b_r: Vec<f32>, // Combined bias

    // Weights Reset Gate
    w_iz: Vec<f32>,
    w_hz: Vec<f32>,
    b_z: Vec<f32>,

    // Weights New Gate
    w_in: Vec<f32>,
    w_hn: Vec<f32>,
    b_n: Vec<f32>,

    // Hidden state
    pub h_t: Vec<f32>,
}

impl GRULayer {
    /// Initializes a new instance of the associated type.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        let w_size_i = input_size * hidden_size;
        let w_size_h = hidden_size * hidden_size;

        Self {
            input_size,
            hidden_size,
            w_ir: alloc::vec![0.1; w_size_i],
            w_hr: alloc::vec![0.1; w_size_h],
            b_r: alloc::vec![0.0; hidden_size],
            w_iz: alloc::vec![0.1; w_size_i],
            w_hz: alloc::vec![0.1; w_size_h],
            b_z: alloc::vec![0.0; hidden_size],
            w_in: alloc::vec![0.1; w_size_i],
            w_hn: alloc::vec![0.1; w_size_h],
            b_n: alloc::vec![0.0; hidden_size],
            h_t: alloc::vec![0.0; hidden_size],
        }
    }

    /// Pure zero-allocation step using SIMD optimization.
    pub fn step(&mut self, input: &[f32], output: &mut [f32]) {
        assert_eq!(input.len(), self.input_size);
        assert_eq!(output.len(), self.hidden_size);

        for j in 0..self.hidden_size {
            // 1. Calculate gates using row-major dot products
            let (r_gate, z_gate) = self.calc_gates(j, input);

            // 2. New gate calculation
            let n_gate = self.calc_new_gate(j, input, r_gate);

            // 3. Update hidden state
            // h_t = (1 - z) * n + z * h_{t-1}
            let next_h = (1.0 - z_gate) * n_gate + z_gate * self.h_t[j];

            self.h_t[j] = next_h;
            output[j] = next_h;
        }
    }

    #[inline(always)]
    fn calc_gates(&self, j: usize, input: &[f32]) -> (f32, f32) {
        let mut r_sum_simd = f32x4::ZERO;
        let mut z_sum_simd = f32x4::ZERO;

        let row_off_i = j * self.input_size;
        let row_off_h = j * self.hidden_size;

        // Input weights
        let mut k = 0;
        while k + 4 <= self.input_size {
            let x = f32x4::from(&input[k..k + 4]);
            r_sum_simd += x * f32x4::from(&self.w_ir[row_off_i + k..row_off_i + k + 4]);
            z_sum_simd += x * f32x4::from(&self.w_iz[row_off_i + k..row_off_i + k + 4]);
            k += 4;
        }

        let mut r_sum = r_sum_simd.reduce_add();
        let mut z_sum = z_sum_simd.reduce_add();

        while k < self.input_size {
            r_sum += input[k] * self.w_ir[row_off_i + k];
            z_sum += input[k] * self.w_iz[row_off_i + k];
            k += 1;
        }

        // Recurrent weights
        let mut h = 0;
        let mut r_h_simd = f32x4::ZERO;
        let mut z_h_simd = f32x4::ZERO;

        while h + 4 <= self.hidden_size {
            let prev_h = f32x4::from(&self.h_t[h..h + 4]);
            r_h_simd += prev_h * f32x4::from(&self.w_hr[row_off_h + h..row_off_h + h + 4]);
            z_h_simd += prev_h * f32x4::from(&self.w_hz[row_off_h + h..row_off_h + h + 4]);
            h += 4;
        }

        r_sum += r_h_simd.reduce_add();
        z_sum += z_h_simd.reduce_add();

        while h < self.hidden_size {
            r_sum += self.h_t[h] * self.w_hr[row_off_h + h];
            z_sum += self.h_t[h] * self.w_hz[row_off_h + h];
            h += 1;
        }

        // Apply activation (Sigmoid)
        let r = 1.0 / (1.0 + (-(r_sum + self.b_r[j])).exp());
        let z = 1.0 / (1.0 + (-(z_sum + self.b_z[j])).exp());

        (r, z)
    }

    #[inline(always)]
    fn calc_new_gate(&self, j: usize, input: &[f32], r: f32) -> f32 {
        let row_off_i = j * self.input_size;
        let row_off_h = j * self.hidden_size;

        let mut n_sum = self.b_n[j];

        // x * W_in
        for i in 0..self.input_size {
            n_sum += input[i] * self.w_in[row_off_i + i];
        }

        // r * (h * W_hn)
        let mut h_sum = 0.0;
        for h in 0..self.hidden_size {
            h_sum += self.h_t[h] * self.w_hn[row_off_h + h];
        }

        (n_sum + r * h_sum).tanh()
    }

    /// Resets the internal state of the component.
    pub fn reset_state(&mut self) {
        for h in self.h_t.iter_mut() {
            *h = 0.0;
        }
    }
}
