/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x547a80a6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/simd/avx2.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_gain_avx2(input: &[f32], output: &mut [f32], gain: f32) {
    let len = input.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );

    let v_gain = _mm256_set1_ps(gain);

    for i in (0..len).step_by(8) {
        let v_in = _mm256_loadu_ps(input.as_ptr().add(i));
        let v_out = _mm256_mul_ps(v_in, v_gain);
        _mm256_storeu_ps(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_mix_avx2(
    input_a: &[f32],
    input_b: &[f32],
    output: &mut [f32],
    gain_a: f32,
    gain_b: f32,
) {
    let len = input_a.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );
    assert!(
        len == input_b.len() && len == output.len(),
        "Buffer sizes must match"
    );

    let v_gain_a = _mm256_set1_ps(gain_a);
    let v_gain_b = _mm256_set1_ps(gain_b);

    for i in (0..len).step_by(8) {
        let v_a = _mm256_loadu_ps(input_a.as_ptr().add(i));
        let v_b = _mm256_loadu_ps(input_b.as_ptr().add(i));
        let v_out = _mm256_add_ps(_mm256_mul_ps(v_a, v_gain_a), _mm256_mul_ps(v_b, v_gain_b));
        _mm256_storeu_ps(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_copy_avx2(input: &[f32], output: &mut [f32]) {
    let len = input.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );
    assert!(len == output.len(), "Buffer sizes must match");

    for i in (0..len).step_by(8) {
        let v = _mm256_loadu_ps(input.as_ptr().add(i));
        _mm256_storeu_ps(output.as_mut_ptr().add(i), v);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_add_avx2(input_a: &[f32], input_b: &[f32], output: &mut [f32]) {
    let len = input_a.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );
    assert!(
        len == input_b.len() && len == output.len(),
        "Buffer sizes must match"
    );

    for i in (0..len).step_by(8) {
        let v_a = _mm256_loadu_ps(input_a.as_ptr().add(i));
        let v_b = _mm256_loadu_ps(input_b.as_ptr().add(i));
        let v_out = _mm256_add_ps(v_a, v_b);
        _mm256_storeu_ps(output.as_mut_ptr().add(i), v_out);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn buffer_sum_avx2(input: &[f32]) -> f32 {
    let len = input.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );

    let mut acc = _mm256_set1_ps(0.0);

    for i in (0..len).step_by(8) {
        let v = _mm256_loadu_ps(input.as_ptr().add(i));
        acc = _mm256_add_ps(acc, v);
    }

    let arr: [f32; 8] = core::mem::transmute(acc);
    arr.iter().fold(0.0, |sum, &x| sum + x)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn process_clear_avx2(output: &mut [f32]) {
    let len = output.len();
    assert!(
        len % 8 == 0,
        "Buffer length must be a multiple of 8 for AVX2"
    );

    let v_zero = _mm256_set1_ps(0.0);

    for i in (0..len).step_by(8) {
        _mm256_storeu_ps(output.as_mut_ptr().add(i), v_zero);
    }
}
