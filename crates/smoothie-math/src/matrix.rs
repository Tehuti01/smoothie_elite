//! Small fixed-size matrix math used in reverb and spatialization.

/// 4×4 f32 matrix (column-major).
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        let mut m = Self { cols: [[0.0; 4]; 4] };
        m.cols[0][0] = 1.0;
        m.cols[1][1] = 1.0;
        m.cols[2][2] = 1.0;
        m.cols[3][3] = 1.0;
        m
    }

    pub fn zero() -> Self { Self { cols: [[0.0; 4]; 4] } }

    pub fn mul_vec4(&self, v: [f32; 4]) -> [f32; 4] {
        let mut out = [0.0f32; 4];
        for row in 0..4 {
            for col in 0..4 {
                out[row] += self.cols[col][row] * v[col];
            }
        }
        out
    }
}

/// Hadamard matrix H_N for FDN reverb mixing. N must be a power of 2, max 16.
pub fn hadamard_in_place(buf: &mut [f32]) {
    let n = buf.len();
    assert!(n.is_power_of_two() && n <= 16, "Hadamard only supports power-of-2 sizes up to 16");
    let mut step = 1usize;
    while step < n {
        for i in (0..n).step_by(step * 2) {
            for j in 0..step {
                let a = buf[i + j];
                let b = buf[i + j + step];
                buf[i + j]        = a + b;
                buf[i + j + step] = a - b;
            }
        }
        step *= 2;
    }
    let norm = (n as f32).sqrt().recip();
    for x in buf.iter_mut() { *x *= norm; }
}

/// Householder reflection matrix applied in-place: y = x - 2/N * (1^T x) * 1
pub fn householder_in_place(buf: &mut [f32]) {
    let n = buf.len() as f32;
    let sum: f32 = buf.iter().sum();
    let factor = 2.0 / n * sum;
    for x in buf.iter_mut() { *x -= factor; }
}
