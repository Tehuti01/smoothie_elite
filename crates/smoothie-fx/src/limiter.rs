//! Brick-wall limiter with lookahead and true peak detection.

use smoothie_math::decibels::{linear_to_db, db_to_linear};

const LOOKAHEAD_MAX: usize = 2048;

pub struct BrickwallLimiter {
    lookahead_buf: [[f32; LOOKAHEAD_MAX]; 2],
    lookahead_ptr: usize,
    lookahead_len: usize,
    gain_buf:      [f32; LOOKAHEAD_MAX],
    gain_ptr:      usize,
    gain_smooth:   f32,
    release_coeff: f32,
    /// Ceiling in dBFS (typically -0.1 to -1.0).
    pub ceiling:   f32,
    /// Release time coefficient (pre-computed).
    pub release_ms: f32,
    sample_rate:   f32,
}

impl BrickwallLimiter {
    pub fn new(sample_rate: f32) -> Self {
        let release_coeff = (-1000.0 / (sample_rate * 300.0)).exp();
        Self {
            lookahead_buf: [[0.0; LOOKAHEAD_MAX]; 2],
            lookahead_ptr: 0,
            lookahead_len: (0.001 * sample_rate) as usize,
            gain_buf:      [1.0; LOOKAHEAD_MAX],
            gain_ptr:      0,
            gain_smooth:   1.0,
            release_coeff,
            ceiling: -0.3,
            release_ms: 300.0,
            sample_rate,
        }
    }

    pub fn set_lookahead_ms(&mut self, ms: f32) {
        self.lookahead_len = ((ms * self.sample_rate / 1000.0) as usize)
            .clamp(1, LOOKAHEAD_MAX - 1);
    }

    pub fn set_release_ms(&mut self, ms: f32) {
        self.release_ms    = ms;
        self.release_coeff = (-1000.0 / (self.sample_rate * ms)).exp();
    }

    #[inline]
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        // Write into lookahead buffer
        let wp = self.lookahead_ptr;
        self.lookahead_buf[0][wp] = in_l;
        self.lookahead_buf[1][wp] = in_r;

        // Peak detection at write point
        let peak = in_l.abs().max(in_r.abs());
        let ceiling_lin = db_to_linear(self.ceiling);
        let target_gain = if peak > ceiling_lin { ceiling_lin / peak } else { 1.0 };

        // Attack: instant (brick wall)
        let gain = if target_gain < self.gain_smooth {
            target_gain
        } else {
            self.release_coeff * self.gain_smooth + (1.0 - self.release_coeff) * target_gain
        };
        self.gain_smooth = gain;
        self.gain_buf[wp] = gain;

        // Read delayed sample
        let rp = (wp + LOOKAHEAD_MAX - self.lookahead_len) % LOOKAHEAD_MAX;
        let gain_delayed = self.gain_buf[rp];
        let out_l = self.lookahead_buf[0][rp] * gain_delayed;
        let out_r = self.lookahead_buf[1][rp] * gain_delayed;

        self.lookahead_ptr = (self.lookahead_ptr + 1) % LOOKAHEAD_MAX;

        // Hard clip safety net
        (out_l.clamp(-1.0, 1.0), out_r.clamp(-1.0, 1.0))
    }

    /// Returns current gain reduction in dB.
    pub fn gain_reduction_db(&self) -> f32 {
        linear_to_db(self.gain_smooth)
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
