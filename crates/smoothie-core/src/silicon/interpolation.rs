//! Predictive Touch Interpolation
//! Uses a simple linear regression to "guess" where the user's finger will be in 8ms.
//! Masks the physical latency of the touch digitizer.

/// A simple predictor for UI input coordinates.
pub struct TouchPredictor {
    prev_pos: [f32; 2],
    velocity: [f32; 2],
    last_time: u64,
}

impl TouchPredictor {
    pub const fn new() -> Self {
        Self {
            prev_pos: [0.0, 0.0],
            velocity: [0.0, 0.0],
            last_time: 0,
        }
    }

    /// Update the predictor with a new raw position.
    pub fn update(&mut self, x: f32, y: f32, time_ms: u64) {
        if self.last_time != 0 {
            let dt = (time_ms - self.last_time) as f32;
            if dt > 0.0 {
                self.velocity[0] = (x - self.prev_pos[0]) / dt;
                self.velocity[1] = (y - self.prev_pos[1]) / dt;
            }
        }
        self.prev_pos = [x, y];
        self.last_time = time_ms;
    }

    /// Predict the position at a future time (e.g., in 8ms).
    pub fn predict(&self, future_ms: f32) -> [f32; 2] {
        [
            self.prev_pos[0] + self.velocity[0] * future_ms,
            self.prev_pos[1] + self.velocity[1] * future_ms,
        ]
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
