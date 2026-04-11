//! Tape saturation and wow/flutter emulation.

use smoothie_math::random::Xorshift32;

/// Tape saturation — combines soft-clipping, HF roll-off, and bias distortion.
pub struct TapeSaturation {
    /// Drive amount [0, 1].
    pub drive:   f32,
    /// Tape bias [0, 1] — higher = more even-harmonic character.
    pub bias:    f32,
    /// HF roll-off coefficient (one-pole lowpass).
    lp_state: f32,
    lp_coeff: f32,
    /// Noise level [0, 1].
    pub noise:   f32,
    rng: Xorshift32,
}

impl TapeSaturation {
    pub fn new(sample_rate: f32) -> Self {
        let cutoff = 16000.0_f32;
        let coeff = (-2.0 * std::f32::consts::PI * cutoff / sample_rate).exp();
        Self {
            drive:    0.5,
            bias:     0.1,
            lp_state: 0.0,
            lp_coeff: coeff,
            noise:    0.001,
            rng:      Xorshift32::new(54321),
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // HF roll-off (tape head response)
        self.lp_state += (1.0 - self.lp_coeff) * (x - self.lp_state);
        let x = self.lp_state;

        // Bias distortion
        let biased = x + self.bias * 0.1;

        // Drive and soft-clip
        let driven = biased * (1.0 + self.drive * 8.0);
        let clipped = driven.tanh();

        // Tape noise
        let noise_sig = self.rng.next_f32() * self.noise;

        clipped + noise_sig
    }
}

/// Wow and flutter — slow (wow) and fast (flutter) pitch modulation.
pub struct WowFlutter {
    wow_phase:     f32,
    flutter_phase: f32,
    buf:           Vec<f32>,
    write_ptr:     usize,
    max_len:       usize,
    rng:           Xorshift32,
    /// Wow rate Hz (typically 0.5–2).
    pub wow_rate:    f32,
    /// Wow depth in samples.
    pub wow_depth:   f32,
    /// Flutter rate Hz (typically 5–15).
    pub flutter_rate: f32,
    /// Flutter depth in samples.
    pub flutter_depth: f32,
    /// Random flutter component [0, 1].
    pub random_flutter: f32,
    sample_rate: f32,
}

impl WowFlutter {
    pub fn new(sample_rate: f32) -> Self {
        let max_len = (0.05 * sample_rate) as usize + 4;
        Self {
            wow_phase:      0.0,
            flutter_phase:  0.0,
            buf:            vec![0.0; max_len],
            write_ptr:      0,
            max_len,
            rng:            Xorshift32::new(11111),
            wow_rate:       0.5,
            wow_depth:      sample_rate * 0.001,
            flutter_rate:   8.0,
            flutter_depth:  sample_rate * 0.0002,
            random_flutter: 0.3,
            sample_rate,
        }
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.buf[self.write_ptr] = x;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        let wow_mod = (self.wow_phase * std::f32::consts::TAU).sin();
        let flutter_mod = (self.flutter_phase * std::f32::consts::TAU).sin()
            + self.rng.next_f32() * self.random_flutter;

        let delay = (self.max_len / 2) as f32
            + wow_mod * self.wow_depth
            + flutter_mod * self.flutter_depth;
        let delay = delay.clamp(1.0, self.max_len as f32 - 2.0);

        let di = delay as usize;
        let frac = delay - di as f32;
        let a = self.buf[(self.write_ptr + self.max_len - di) % self.max_len];
        let b = self.buf[(self.write_ptr + self.max_len - di - 1) % self.max_len];

        self.wow_phase += self.wow_rate / self.sample_rate;
        if self.wow_phase >= 1.0 { self.wow_phase -= 1.0; }
        self.flutter_phase += self.flutter_rate / self.sample_rate;
        if self.flutter_phase >= 1.0 { self.flutter_phase -= 1.0; }

        a + frac * (b - a)
    }
}
