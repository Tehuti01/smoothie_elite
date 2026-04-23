/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5120b79d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/simd/neon.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_gain_neon(input: &[f32], output: &mut [f32], gain: f32) {
    let len = input.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );

    let v_gain = vdupq_n_f32(gain);

    for i in (0..len).step_by(4) {
        let v_in = vld1q_f32(input.as_ptr().add(i));
        let v_out = vmulq_f32(v_in, v_gain);
        vst1q_f32(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_mix_neon(
    input_a: &[f32],
    input_b: &[f32],
    output: &mut [f32],
    gain_a: f32,
    gain_b: f32,
) {
    let len = input_a.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );
    assert!(
        len == input_b.len() && len == output.len(),
        "Buffer sizes must match"
    );

    let v_gain_a = vdupq_n_f32(gain_a);
    let v_gain_b = vdupq_n_f32(gain_b);

    for i in (0..len).step_by(4) {
        let v_a = vld1q_f32(input_a.as_ptr().add(i));
        let v_b = vld1q_f32(input_b.as_ptr().add(i));
        let v_out = vaddq_f32(vmulq_f32(v_a, v_gain_a), vmulq_f32(v_b, v_gain_b));
        vst1q_f32(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_copy_neon(input: &[f32], output: &mut [f32]) {
    let len = input.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );
    assert!(len == output.len(), "Buffer sizes must match");

    for i in (0..len).step_by(4) {
        let v = vld1q_f32(input.as_ptr().add(i));
        vst1q_f32(output.as_mut_ptr().add(i), v);
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_add_neon(input_a: &[f32], input_b: &[f32], output: &mut [f32]) {
    let len = input_a.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );
    assert!(
        len == input_b.len() && len == output.len(),
        "Buffer sizes must match"
    );

    for i in (0..len).step_by(4) {
        let v_a = vld1q_f32(input_a.as_ptr().add(i));
        let v_b = vld1q_f32(input_b.as_ptr().add(i));
        let v_out = vaddq_f32(v_a, v_b);
        vst1q_f32(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn buffer_sum_neon(input: &[f32]) -> f32 {
    let len = input.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );

    let mut acc = vdupq_n_f32(0.0);

    for i in (0..len).step_by(4) {
        let v = vld1q_f32(input.as_ptr().add(i));
        acc = vaddq_f32(acc, v);
    }

    let arr: [f32; 4] = core::mem::transmute(acc);
    arr.iter().fold(0.0, |sum, &x| sum + x)
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn process_clear_neon(output: &mut [f32]) {
    let len = output.len();
    assert!(
        len.is_multiple_of(4),
        "Buffer length must be a multiple of 4 for NEON"
    );

    let v_zero = vdupq_n_f32(0.0);

    for i in (0..len).step_by(4) {
        vst1q_f32(output.as_mut_ptr().add(i), v_zero);
    }
}
