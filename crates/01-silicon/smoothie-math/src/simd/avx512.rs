/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5be8bc83 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/simd/avx512.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Process 16 f32 samples simultaneously
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
pub unsafe fn process_gain_avx512(input: &[f32], output: &mut [f32], gain: f32) {
    let len = input.len();
    assert!(
        len % 16 == 0,
        "Buffer length must be a multiple of 16 for AVX-512"
    );

    // Load scalar gain into all 16 lanes of 512-bit register
    let v_gain = _mm512_set1_ps(gain);

    for i in (0..len).step_by(16) {
        let v_in = _mm512_loadu_ps(input.as_ptr().add(i) as *const _);
        let v_out = _mm512_mul_ps(v_in, v_gain);
        _mm512_storeu_ps(output.as_mut_ptr().add(i) as *mut _, v_out);
    }
}
