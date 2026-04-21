/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xeda9c8ab | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/multiply.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::matrix::{Mat4, Vec3};

#[inline(always)]
/// Technical implementation of the mul_mat4_vec4 logic.
pub fn mul_mat4_vec4(m: &Mat4, v: &[f32; 4]) -> [f32; 4] {
    let mut result = [0.0; 4];
    for i in 0..4 {
        let mut sum = 0.0;
        for j in 0..4 {
            sum += m.m[i][j] * v[j];
        }
        result[i] = sum;
    }
    result
}

#[inline(always)]
/// Technical implementation of the mul_mat4_mat4 logic.
pub fn mul_mat4_mat4(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut result = Mat4::default();
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a.m[i][k] * b.m[k][j];
            }
            result.m[i][j] = sum;
        }
    }
    result
}

#[inline(always)]
/// Technical implementation of the outer_product logic.
pub fn outer_product(a: &[f32; 4], b: &[f32; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] = a[i] * b[j];
        }
    }
    result
}

#[inline(always)]
/// Technical implementation of the transpose_4x4 logic.
pub fn transpose_4x4(m: &Mat4) -> Mat4 {
    let mut result = Mat4::default();
    for i in 0..4 {
        for j in 0..4 {
            result.m[i][j] = m.m[j][i];
        }
    }
    result
}
