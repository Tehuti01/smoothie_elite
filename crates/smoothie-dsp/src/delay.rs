//! Delay lines — heap-allocated once at init, zero-alloc at runtime.

/// A simple delay line with linear interpolation.
pub struct DelayLine {
    buffer:     Vec<f32>,
    write_pos:  usize,
    max_samples: usize,
}

impl DelayLine {
    /// Allocate a delay line that can hold `max_samples` samples.
    pub fn new(max_samples: usize) -> Self {
        Self {
            buffer: vec![0.0; max_samples + 1],
            write_pos: 0,
            max_samples,
        }
    }

    /// Write a sample and return the delayed output at `delay_samples` ago.
    /// Supports fractional delay via linear interpolation.
    #[inline]
    pub fn process(&mut self, x: f32, delay_samples: f32) -> f32 {
        self.buffer[self.write_pos] = x;
        let len = self.buffer.len();

        let d = delay_samples.min(self.max_samples as f32);
        let d_floor = d as usize;
        let frac    = d - d_floor as f32;

        let pos0 = (self.write_pos + len - d_floor)     % len;
        let pos1 = (self.write_pos + len - d_floor - 1) % len;

        let output = self.buffer[pos0] * (1.0 - frac) + self.buffer[pos1] * frac;

        self.write_pos = (self.write_pos + 1) % len;
        output
    }

    /// Clear all internal state.
    #[inline]
    pub fn reset(&mut self) {
        self.buffer.iter_mut().for_each(|s| *s = 0.0);
        self.write_pos = 0;
    }
}

/// First-order allpass interpolating delay — useful for chorus / phaser.
pub struct AllpassDelay {
    buf:   f32,
    coeff: f32,
}

impl AllpassDelay {
    pub fn new(delay_frac: f32) -> Self {
        let coeff = (1.0 - delay_frac) / (1.0 + delay_frac);
        Self { buf: 0.0, coeff }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.coeff * x + self.buf;
        self.buf = x - self.coeff * y;
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_line() {
        let mut delay = DelayLine::new(10);
        
        // Write a pulse
        let mut out = vec![0.0; 20];
        for i in 0..20 {
            let x = if i == 0 { 1.0 } else { 0.0 };
            out[i] = delay.process(x, 5.0);
        }
        
        // Should appear at index 5
        assert_eq!(out[0], 0.0);
        assert_eq!(out[5], 1.0);
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
