//! Vibrato — frequency modulation via pitch-shifted delay.

use smoothie_math::trig::Phasor;
use smoothie_math::interp::hermite4;

pub struct Vibrato {
    buf:       Vec<f32>,
    write_ptr: usize,
    max_len:   usize,
    lfo:       Phasor,
    /// LFO rate Hz.
    pub rate:   f32,
    /// Modulation depth in samples.
    pub depth:  f32,
    sample_rate: f32,
}

impl Vibrato {
    pub fn new(sample_rate: f32) -> Self {
        let max_len = (0.02 * sample_rate) as usize + 4;
        let centre_delay = sample_rate * 0.005;
        Self {
            buf: vec![0.0; max_len],
            write_ptr: 0,
            max_len,
            lfo: Phasor::new(5.0, sample_rate),
            rate: 5.0,
            depth: sample_rate * 0.002,
            sample_rate,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.rate = hz;
        self.lfo.set_freq(hz, self.sample_rate);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        self.buf[self.write_ptr] = x;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        let phase = self.lfo.tick();
        let mod_val = (phase * std::f32::consts::TAU).sin();
        let centre = self.sample_rate * 0.005;
        let delay = centre + mod_val * self.depth;
        let delay = delay.max(1.0);
        let di = delay as usize;
        let frac = delay - di as f32;

        let idx = |d: usize| self.buf[(self.write_ptr + self.max_len - d) % self.max_len];
        hermite4(idx(di.saturating_sub(1)), idx(di), idx(di+1), idx(di+2), frac)
    }

    pub fn reset(&mut self) {
        self.buf.fill(0.0);
    }
}
