//! dB / linear conversion and loudness utilities.

/// Linear amplitude → dB. Returns −∞ for values ≤ 0.
#[inline]
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 { return f32::NEG_INFINITY; }
    20.0 * linear.log10()
}

/// dB → linear amplitude.
#[inline]
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Power ratio → dB (10 * log10).
#[inline]
pub fn power_to_db(power: f32) -> f32 {
    if power <= 0.0 { return f32::NEG_INFINITY; }
    10.0 * power.log10()
}

/// dB → power ratio.
#[inline]
pub fn db_to_power(db: f32) -> f32 {
    10.0_f32.powf(db / 10.0)
}

/// Clamp a dB value to the audible range [−144, +24].
#[inline]
pub fn clamp_db(db: f32) -> f32 {
    db.clamp(-144.0, 24.0)
}

/// Gain matching: find the gain (dB) needed to match `target_db` from `current_db`.
#[inline]
pub fn gain_to_match_db(current_db: f32, target_db: f32) -> f32 {
    target_db - current_db
}

/// Mixing law: equal-power pan at `pan` in [-1, 1] → (left_gain, right_gain).
#[inline]
pub fn equal_power_pan(pan: f32) -> (f32, f32) {
    let angle = (pan + 1.0) * 0.25 * std::f32::consts::PI;
    (angle.cos(), angle.sin())
}

/// Simple stereo balance (linear). Pan in [-1, 1].
#[inline]
pub fn linear_pan(pan: f32) -> (f32, f32) {
    let r = (pan + 1.0) * 0.5;
    (1.0 - r, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let db = -12.0f32;
        let lin = db_to_linear(db);
        assert!((linear_to_db(lin) - db).abs() < 1e-4);
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
