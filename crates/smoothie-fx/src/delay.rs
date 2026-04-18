//! Tempo-sync and free delay lines with interpolation.

use smoothie_math::interp::hermite4;

/// Multi-tap delay with hermite interpolation, ping-pong mode, and feedback.
pub struct StereoDelay {
    buf_l:     Vec<f32>,
    buf_r:     Vec<f32>,
    write_ptr: usize,
    max_len:   usize,
    /// Delay time in samples (fractional).
    pub delay_l:   f32,
    pub delay_r:   f32,
    /// Feedback amount [0, 1).
    pub feedback:  f32,
    /// Ping-pong mode swaps L/R on each feedback echo.
    pub ping_pong: bool,
    /// Dry/wet [0, 1].
    pub mix:       f32,
    /// High-cut on feedback path (one-pole coeff).
    pub damping:   f32,
    lp_l: f32,
    lp_r: f32,
}

impl StereoDelay {
    pub fn new(max_seconds: f32, sample_rate: f32) -> Self {
        let max_len = (max_seconds * sample_rate) as usize + 4;
        Self {
            buf_l: vec![0.0; max_len],
            buf_r: vec![0.0; max_len],
            write_ptr: 0,
            max_len,
            delay_l:   (0.25 * sample_rate),
            delay_r:   (0.25 * sample_rate),
            feedback:  0.4,
            ping_pong: false,
            mix:       0.3,
            damping:   0.3,
            lp_l:      0.0,
            lp_r:      0.0,
        }
    }

    #[inline]
    fn read_interp(buf: &[f32], write_ptr: usize, delay: f32, max_len: usize) -> f32 {
        let delay_int = delay as usize;
        let frac = delay - delay_int as f32;
        let idx = |d: usize| -> f32 {
            buf[(write_ptr + max_len - d) % max_len]
        };
        let y0 = idx(delay_int.saturating_sub(1));
        let y1 = idx(delay_int);
        let y2 = idx(delay_int + 1);
        let y3 = idx(delay_int + 2);
        hermite4(y0, y1, y2, y3, frac)
    }

    #[inline]
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let wet_l = Self::read_interp(&self.buf_l, self.write_ptr, self.delay_l, self.max_len);
        let wet_r = Self::read_interp(&self.buf_r, self.write_ptr, self.delay_r, self.max_len);

        // Damping (one-pole lowpass on feedback)
        self.lp_l += (1.0 - self.damping) * (wet_l - self.lp_l);
        self.lp_r += (1.0 - self.damping) * (wet_r - self.lp_r);

        let fb_l = if self.ping_pong { self.lp_r } else { self.lp_l };
        let fb_r = if self.ping_pong { self.lp_l } else { self.lp_r };

        self.buf_l[self.write_ptr] = in_l + fb_l * self.feedback;
        self.buf_r[self.write_ptr] = in_r + fb_r * self.feedback;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        (in_l * (1.0 - self.mix) + wet_l * self.mix,
         in_r * (1.0 - self.mix) + wet_r * self.mix)
    }

    pub fn set_time_ms(&mut self, ms_l: f32, ms_r: f32, sample_rate: f32) {
        self.delay_l = ((ms_l * sample_rate / 1000.0) as usize).min(self.max_len - 4) as f32;
        self.delay_r = ((ms_r * sample_rate / 1000.0) as usize).min(self.max_len - 4) as f32;
    }

    pub fn reset(&mut self) {
        self.buf_l.fill(0.0);
        self.buf_r.fill(0.0);
        self.lp_l = 0.0;
        self.lp_r = 0.0;
    }
}

/// Tempo-sync utility: BPM + note value → milliseconds.
pub fn bpm_to_ms(bpm: f32, note_value: NoteValue) -> f32 {
    let beat_ms = 60000.0 / bpm;
    beat_ms * note_value.multiplier()
}

#[derive(Clone, Copy, Debug)]
pub enum NoteValue {
    Whole, Half, Quarter, Eighth, Sixteenth, ThirtySecond,
    DottedHalf, DottedQuarter, DottedEighth,
    TripletHalf, TripletQuarter, TripletEighth,
}

impl NoteValue {
    pub fn multiplier(self) -> f32 {
        match self {
            Self::Whole         => 4.0,
            Self::Half          => 2.0,
            Self::Quarter       => 1.0,
            Self::Eighth        => 0.5,
            Self::Sixteenth     => 0.25,
            Self::ThirtySecond  => 0.125,
            Self::DottedHalf    => 3.0,
            Self::DottedQuarter => 1.5,
            Self::DottedEighth  => 0.75,
            Self::TripletHalf   => 4.0 / 3.0,
            Self::TripletQuarter => 2.0 / 3.0,
            Self::TripletEighth => 1.0 / 3.0,
        }
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
