/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x04dd5bd6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/activations/softmax.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::{exp_approx, fast_log2};

/// Output sums to 1.0, useful for probability distributions.
#[inline(always)]
/// Technical implementation of the softmax logic.
pub fn softmax(x: &[f32], output: &mut [f32]) {
    let len = x.len();
    if len == 0 {
        return;
    }

    let mut max_val = x[0];
    for i in 1..len {
        if x[i] > max_val {
            max_val = x[i];
        }
    }

    let mut sum = 0.0f32;
    for i in 0..len {
        output[i] = exp_approx(x[i] - max_val);
        sum += output[i];
    }

    let inv_sum = 1.0 / sum;
    for i in 0..len {
        output[i] *= inv_sum;
    }
}

/// Log-softmax for numerical stability in cross-entropy computation.
#[inline(always)]
/// Technical implementation of the log_softmax logic.
pub fn log_softmax(x: &[f32], output: &mut [f32]) {
    let len = x.len();
    if len == 0 {
        return;
    }

    let mut max_val = x[0];
    for i in 1..len {
        if x[i] > max_val {
            max_val = x[i];
        }
    }

    let mut sum = 0.0f32;
    for i in 0..len {
        sum += exp_approx(x[i] - max_val);
    }
    let log_sum = fast_log2(sum) * 0.6931471805599453 + max_val;

    for i in 0..len {
        output[i] = x[i] - log_sum;
    }
}

/// Hardmax - returns index of maximum value (for hardware acceleration).
#[inline(always)]
/// Technical implementation of the hardmax logic.
pub fn hardmax(x: &[f32]) -> usize {
    let mut max_idx = 0;
    let mut max_val = x[0];
    for i in 1..x.len() {
        if x[i] > max_val {
            max_val = x[i];
            max_idx = i;
        }
    }
    max_idx
}
