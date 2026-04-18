//! Bitcrusher — sample-rate and bit-depth reduction.

pub struct Bitcrusher {
    /// Bit depth [1..24].
    pub bits: f32,
    /// Sample-rate divider [1..64].
    pub rate_div: f32,
    held_sample: f32,
    phase:       f32,
    /// Wet/dry [0, 1].
    pub mix: f32,
    /// Dither amount [0, 1].
    pub dither: f32,
    rng: smoothie_math::random::Xorshift32,
}

impl Bitcrusher {
    pub fn new() -> Self {
        Self {
            bits:        16.0,
            rate_div:    1.0,
            held_sample: 0.0,
            phase:       0.0,
            mix:         1.0,
            dither:      0.0,
            rng:         smoothie_math::random::Xorshift32::new(99991),
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // Sample-rate reduction
        self.phase += 1.0;
        if self.phase >= self.rate_div {
            self.phase -= self.rate_div;
            let dither_val = if self.dither > 0.0 {
                self.rng.next_f32() * self.dither
            } else { 0.0 };
            // Bit-depth reduction via quantisation
            let levels = (2.0_f32.powf(self.bits)).round();
            let quantized = ((x + dither_val) * levels * 0.5).round() / (levels * 0.5);
            self.held_sample = quantized.clamp(-1.0, 1.0);
        }
        x * (1.0 - self.mix) + self.held_sample * self.mix
    }
}

impl Default for Bitcrusher {
    fn default() -> Self { Self::new() }
}

/// Decimator — sample-rate reduction only (no quantisation).
pub struct Decimator {
    held_sample: f32,
    phase:       f32,
    /// Division ratio.
    pub rate_div: f32,
}

impl Decimator {
    pub fn new() -> Self { Self { held_sample: 0.0, phase: 0.0, rate_div: 1.0 } }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.phase += 1.0;
        if self.phase >= self.rate_div {
            self.phase -= self.rate_div;
            self.held_sample = x;
        }
        self.held_sample
    }
}

impl Default for Decimator {
    fn default() -> Self { Self::new() }
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
