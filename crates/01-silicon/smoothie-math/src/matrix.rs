/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc3ac0796 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/matrix.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

#[inline]
/// Computes the floor of a 32-bit float.
fn floor_f32(x: f32) -> f32 {
    let bits = x.to_bits();
    let sign = bits & 0x80000000;
    let exp = (bits >> 23) & 0xFF;
    if exp < 127 {
        return 0.0;
    }
    let mantissa = bits & 0x007FFFFF;
    let new_exp = exp - 127;
    f32::from_bits(sign | (new_exp << 23) | mantissa)
}

#[inline]
/// Calculates the square root using an optimized approximation.
fn sqrt_f32(x: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else {
        let bits = x.to_bits();
        let exp = ((bits >> 23) & 0xFF) as i32;
        if exp == 0 {
            return 0.0;
        }
        let new_exp = ((exp - 127) >> 1) + 127;
        let mantissa = (bits & 0x007FFFFF) | 0x3F800000;
        let _half_exp_bits = ((exp - 127 + 127) as u32) << 22;
        let guess = f32::from_bits((new_exp as u32) << 23 | mantissa);
        let mut result = guess;
        for _ in 0..5 {
            result = result * 0.5 + x / result * 0.5;
        }
        result
    }
}

#[inline]
/// Computes the sine of a 32-bit float via polynomial approximation.
fn sin_f32(x: f32) -> f32 {
    let phase = x / (2.0 * core::f32::consts::PI);
    let mut p = phase - floor_f32(phase + 0.25);
    let sign = if p < 0.0 { -1.0 } else { 1.0 };
    p = p.abs();
    if p > 0.5 {
        p = 1.0 - p;
    }
    let p_adj = p - 0.5;
    let p2 = p_adj * p_adj;
    sign * (p_adj * 1.0
        + p_adj * p2 * (-1.769_709_4e-2)
        + p_adj * p2 * p_adj * (-1.313_309_3)
        + p_adj * p2 * p2 * p_adj * 2.331_609_4e-3
        + p_adj * p2 * p2 * p2 * p_adj * 1.318_977_9e-1)
}

#[inline]
/// Computes the cosine of a 32-bit float via polynomial approximation.
fn cos_f32(x: f32) -> f32 {
    sin_f32(x + 0.5 * core::f32::consts::PI)
}

#[derive(Clone, Copy, Default, Debug)]
/// Technical implementation of the Vec3 structure.
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Initializes a new instance of the associated type.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a zero-initialized instance.
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline(always)]
    /// Calculates the dot product between two vectors.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    /// Calculates the cross product (3D) between two vectors.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline(always)]
    /// Calculates the Euclidean norm (magnitude) of the vector.
    pub fn magnitude(&self) -> f32 {
        sqrt_f32(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[inline(always)]
    /// Returns a unit-length version of the vector.
    pub fn normalize(&self) -> Self {
        let m = self.magnitude();
        if m < 1e-10 {
            Self::zero()
        } else {
            Self {
                x: self.x / m,
                y: self.y / m,
                z: self.z / m,
            }
        }
    }

    #[inline(always)]
    /// Performs scalar multiplication.
    pub fn scale(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    #[inline(always)]
    /// Performs vector addition logic.
    pub fn add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    #[inline(always)]
    /// Performs vector subtraction logic.
    pub fn sub(&self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Copy, Default)]
/// Technical implementation of the Mat4 structure.
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4 {
    /// Generates a numerical identity representation.
    pub const fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Performs matrix/vector multiplication.
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut result = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[i][k] * rhs.m[k][j];
                }
                result.m[i][j] = sum;
            }
        }
        result
    }

    /// Generates a Y-axis rotation transformation.
    pub fn rotate_y(angle_radians: f32) -> Self {
        let cos = cos_f32(angle_radians);
        let sin = sin_f32(angle_radians);

        let mut mat = Self::identity();
        mat.m[0][0] = cos;
        mat.m[0][2] = sin;
        mat.m[2][0] = -sin;
        mat.m[2][2] = cos;
        mat
    }

    /// Generates an X-axis rotation transformation.
    pub fn rotate_x(angle_radians: f32) -> Self {
        let cos = cos_f32(angle_radians);
        let sin = sin_f32(angle_radians);

        let mut mat = Self::identity();
        mat.m[1][1] = cos;
        mat.m[1][2] = -sin;
        mat.m[2][1] = sin;
        mat.m[2][2] = cos;
        mat
    }

    /// Generates a Z-axis rotation transformation.
    pub fn rotate_z(angle_radians: f32) -> Self {
        let cos = cos_f32(angle_radians);
        let sin = sin_f32(angle_radians);

        let mut mat = Self::identity();
        mat.m[0][0] = cos;
        mat.m[0][1] = -sin;
        mat.m[1][0] = sin;
        mat.m[1][1] = cos;
        mat
    }

    /// Applies a transformation matrix to a 3D vector.
    pub fn transform_vec3(&self, v: &Vec3) -> Vec3 {
        let w = 1.0;
        Vec3 {
            x: self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * w,
            y: self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * w,
            z: self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * w,
        }
    }

    /// Performs scalar multiplication.
    pub fn scale_xyz(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        mat.m[0][0] = x;
        mat.m[1][1] = y;
        mat.m[2][2] = z;
        mat
    }

    /// Generates a translation transformation.
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut mat = Self::identity();
        mat.m[0][3] = x;
        mat.m[1][3] = y;
        mat.m[2][3] = z;
        mat
    }
}
