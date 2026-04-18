use std::f32::consts::SQRT_2;

/// A utility for positioning signals within the stereo field.
///
/// The Panner uses a linear-taper gain distribution with a sqrt(2) 
/// compensation to maintain perceived loudness across the stereo panorama.
pub struct Panner {
    /// Panning position from -1.0 (hard left) to 1.0 (hard right).
    pan: f32,
}

impl Panner {
    /// Creates a new Panner centered (0.0).
    pub fn new() -> Self {
        Self { pan: 0.0 }
    }

    /// Sets the panning position.
    ///
    /// # Arguments
    /// * `pan` - Panning value between -1.0 (left) and 1.0 (right).
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    /// Processes a mono signal into a stereo pair based on the current pan setting.
    pub fn process(&self, input: f32) -> (f32, f32) {
        let pan_norm = (self.pan + 1.0) / 2.0;

        let left_gain = if self.pan <= 0.0 { 1.0 } else { 1.0 - pan_norm };

        let right_gain = if self.pan >= 0.0 { 1.0 } else { pan_norm };

        let gain_reduction = SQRT_2;
        (
            input * left_gain / gain_reduction,
            input * right_gain / gain_reduction,
        )
    }

    /// Pans a stereo signal. Currently sums and pans, though normally stereo panners 
    /// would handle channel balancing.
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let (l_out, r_out) = self.process(left + right);
        (left + l_out, right + r_out)
    }
}

impl Default for Panner {
    fn default() -> Self {
        Self::new()
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
