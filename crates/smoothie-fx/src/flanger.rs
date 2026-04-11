//! Flanger — very short LFO-modulated delay creating comb-filter sweeps.

use smoothie_math::trig::Phasor;
use smoothie_math::interp::hermite4;

pub struct Flanger {
    buf:       Vec<f32>,
    write_ptr: usize,
    max_len:   usize,
    lfo:       Phasor,
    fb_state:  f32,
    /// Max delay in samples (short: 1–10 ms).
    pub max_delay:  f32,
    /// LFO rate Hz.
    pub rate:       f32,
    /// Depth [0, 1].
    pub depth:      f32,
    /// Feedback [-0.99, 0.99].
    pub feedback:   f32,
    /// Wet/dry [0, 1].
    pub mix:        f32,
    /// Invert the wet signal (negative feedback character).
    pub invert:     bool,
    sample_rate: f32,
}

impl Flanger {
    pub fn new(sample_rate: f32) -> Self {
        let max_len = (0.02 * sample_rate) as usize + 4;
        Self {
            buf: vec![0.0; max_len],
            write_ptr: 0,
            max_len,
            lfo: Phasor::new(0.3, sample_rate),
            fb_state: 0.0,
            max_delay: sample_rate * 0.005,
            rate: 0.3,
            depth: 0.7,
            feedback: 0.5,
            mix: 0.7,
            invert: false,
            sample_rate,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.rate = hz;
        self.lfo.set_freq(hz, self.sample_rate);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // Write input + feedback
        self.buf[self.write_ptr] = x + self.fb_state * self.feedback;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        // LFO → delay time
        let lfo_val = self.lfo.tick();
        let mod_val = (lfo_val * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        let delay = 1.0 + mod_val * self.depth * self.max_delay;

        // Hermite interpolated read
        let di = delay as usize;
        let frac = delay - di as f32;
        let idx = |d: usize| self.buf[(self.write_ptr + self.max_len - d) % self.max_len];
        let wet = hermite4(idx(di.saturating_sub(1)), idx(di), idx(di+1), idx(di+2), frac);
        self.fb_state = wet;

        let wet_sig = if self.invert { -wet } else { wet };
        x * (1.0 - self.mix) + wet_sig * self.mix
    }

    pub fn reset(&mut self) {
        self.buf.fill(0.0);
        self.fb_state = 0.0;
    }
}
