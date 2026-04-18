//! Pitch shifter using OLA (overlap-add) with a circular buffer.
//! Simple, low-latency pitch shifting suitable for monophonic signals.

pub struct PitchShifter {
    buf:        Vec<f32>,
    write_ptr:  usize,
    read_ptr:   f32,
    max_len:    usize,
    /// Pitch ratio (2.0 = one octave up, 0.5 = one octave down).
    pub ratio:  f32,
    /// Wet/dry [0, 1].
    pub mix:    f32,
}

impl PitchShifter {
    pub fn new(sample_rate: f32) -> Self {
        let max_len = (0.1 * sample_rate) as usize;
        Self {
            buf: vec![0.0; max_len],
            write_ptr: 0,
            read_ptr: 0.0,
            max_len,
            ratio: 1.0,
            mix:   1.0,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.buf[self.write_ptr] = x;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        // Fractional read pointer advancing at `ratio` speed
        let ri = self.read_ptr as usize % self.max_len;
        let frac = self.read_ptr - self.read_ptr.floor();
        let a = self.buf[ri];
        let b = self.buf[(ri + 1) % self.max_len];
        let wet = a + frac * (b - a);

        self.read_ptr = (self.read_ptr + self.ratio) % self.max_len as f32;

        x * (1.0 - self.mix) + wet * self.mix
    }

    pub fn set_semitones(&mut self, semitones: f32) {
        self.ratio = 2.0_f32.powf(semitones / 12.0);
    }

    pub fn reset(&mut self) {
        self.buf.fill(0.0);
        self.read_ptr = 0.0;
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
