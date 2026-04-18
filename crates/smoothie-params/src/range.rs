//! Parameter value ranges with linear, skewed, and logarithmic scaling.

/// Float parameter range with value→normalized mapping.
#[derive(Debug, Clone, Copy)]
pub enum FloatRange {
    /// Simple linear mapping.
    Linear { min: f32, max: f32 },
    /// Skewed range — useful for gain (most resolution near quiet end).
    Skewed { min: f32, max: f32, skew: f32 },
    /// Logarithmic — useful for frequency.
    Logarithmic { min: f32, max: f32 },
}

impl FloatRange {
    pub fn normalize(&self, value: f32) -> f32 {
        match *self {
            Self::Linear { min, max } => ((value - min) / (max - min)).clamp(0.0, 1.0),
            Self::Skewed { min, max, skew } => {
                ((value - min) / (max - min)).clamp(0.0, 1.0).powf(skew)
            }
            Self::Logarithmic { min, max } => {
                ((value.ln() - min.ln()) / (max.ln() - min.ln())).clamp(0.0, 1.0)
            }
        }
    }

    pub fn denormalize(&self, normalized: f32) -> f32 {
        let n = normalized.clamp(0.0, 1.0);
        match *self {
            Self::Linear { min, max } => min + n * (max - min),
            Self::Skewed { min, max, skew } => min + n.powf(1.0 / skew) * (max - min),
            Self::Logarithmic { min, max } => {
                (min.ln() + n * (max.ln() - min.ln())).exp()
            }
        }
    }
}

impl Default for FloatRange {
    fn default() -> Self { Self::Linear { min: 0.0, max: 1.0 } }
}

/// Integer parameter range.
#[derive(Debug, Clone, Copy)]
pub struct IntRange {
    pub min: i32,
    pub max: i32,
}

impl IntRange {
    pub fn normalize(&self, value: i32) -> f32 {
        (value - self.min) as f32 / (self.max - self.min) as f32
    }

    pub fn denormalize(&self, normalized: f32) -> i32 {
        let n = normalized.clamp(0.0, 1.0);
        self.min + (n * (self.max - self.min) as f32).round() as i32
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
