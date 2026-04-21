/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4ddd0912 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/rnn/lstm.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::activations::{sigmoid, tanh};
use alloc::vec;
use alloc::vec::Vec;

/// Technical implementation of the LSTMLayer structure.
pub struct LSTMLayer {
    input_size: usize,
    hidden_size: usize,

    // Input gate weights
    w_ii: Vec<f32>,
    w_hi: Vec<f32>,
    b_i: Vec<f32>,
    // Forget gate weights
    w_if: Vec<f32>,
    w_hf: Vec<f32>,
    b_f: Vec<f32>,
    // Cell gate weights
    w_ic: Vec<f32>,
    w_hc: Vec<f32>,
    b_c: Vec<f32>,
    // Output gate weights
    w_io: Vec<f32>,
    w_ho: Vec<f32>,
    b_o: Vec<f32>,

    // State
    pub h_t: Vec<f32>,
    pub c_t: Vec<f32>,
}

impl LSTMLayer {
    /// Initializes a new instance of the associated type.
    pub fn new(input_size: usize, hidden_size: usize) -> Self {
        let w_i = input_size * hidden_size;
        let w_h = hidden_size * hidden_size;

        Self {
            input_size,
            hidden_size,
            w_ii: vec![0.1; w_i],
            w_hi: vec![0.1; w_h],
            b_i: vec![0.0; hidden_size],
            w_if: vec![0.1; w_i],
            w_hf: vec![0.1; w_h],
            b_f: vec![0.0; hidden_size],
            w_ic: vec![0.1; w_i],
            w_hc: vec![0.1; w_h],
            b_c: vec![0.0; hidden_size],
            w_io: vec![0.1; w_i],
            w_ho: vec![0.1; w_h],
            b_o: vec![0.0; hidden_size],
            h_t: vec![0.0; hidden_size],
            c_t: vec![0.0; hidden_size],
        }
    }

    /// Technical implementation of the load_weights logic.
    pub fn load_weights(
        &mut self,
        w_ii: &[f32],
        w_hi: &[f32],
        b_i: &[f32],
        w_if: &[f32],
        w_hf: &[f32],
        b_f: &[f32],
        w_ic: &[f32],
        w_hc: &[f32],
        b_c: &[f32],
        w_io: &[f32],
        w_ho: &[f32],
        b_o: &[f32],
    ) -> Result<(), &'static str> {
        if w_ii.len() != self.w_ii.len() {
            return Err("w_ii size mismatch");
        }
        if w_hi.len() != self.w_hi.len() {
            return Err("w_hi size mismatch");
        }
        if b_i.len() != self.b_i.len() {
            return Err("b_i size mismatch");
        }

        self.w_ii.copy_from_slice(w_ii);
        self.w_hi.copy_from_slice(w_hi);
        self.b_i.copy_from_slice(b_i);
        self.w_if.copy_from_slice(w_if);
        self.w_hf.copy_from_slice(w_hf);
        self.b_f.copy_from_slice(b_f);
        self.w_ic.copy_from_slice(w_ic);
        self.w_hc.copy_from_slice(w_hc);
        self.b_c.copy_from_slice(b_c);
        self.w_io.copy_from_slice(w_io);
        self.w_ho.copy_from_slice(w_ho);
        self.b_o.copy_from_slice(b_o);
        Ok(())
    }

    /// Technical implementation of the step logic.
    pub fn step(&mut self, input: &[f32], output: &mut [f32]) {
        for j in 0..self.hidden_size {
            let mut i_sum = self.b_i[j];
            let mut f_sum = self.b_f[j];
            let mut c_sum = self.b_c[j];
            let mut o_sum = self.b_o[j];

            for i in 0..self.input_size {
                let x = input[i];
                i_sum += x * self.w_ii[i * self.hidden_size + j];
                f_sum += x * self.w_if[i * self.hidden_size + j];
                c_sum += x * self.w_ic[i * self.hidden_size + j];
                o_sum += x * self.w_io[i * self.hidden_size + j];
            }

            for h in 0..self.hidden_size {
                let h_prev = self.h_t[h];
                i_sum += h_prev * self.w_hi[h * self.hidden_size + j];
                f_sum += h_prev * self.w_hf[h * self.hidden_size + j];
                c_sum += h_prev * self.w_hc[h * self.hidden_size + j];
                o_sum += h_prev * self.w_ho[h * self.hidden_size + j];
            }

            let i_gate = sigmoid(i_sum);
            let f_gate = sigmoid(f_sum);
            let c_candidate = tanh(c_sum);
            let o_gate = sigmoid(o_sum);

            self.c_t[j] = f_gate * self.c_t[j] + i_gate * c_candidate;
            self.h_t[j] = o_gate * tanh(self.c_t[j]);
            output[j] = self.h_t[j];
        }
    }

    /// Resets the internal state of the component.
    pub fn reset_state(&mut self) {
        for h in self.h_t.iter_mut() {
            *h = 0.0;
        }
        for c in self.c_t.iter_mut() {
            *c = 0.0;
        }
    }
}
