//! Chorus — multi-voice LFO-modulated delay with stereo spread.

use smoothie_math::trig::Phasor;
use smoothie_math::interp::hermite4;

pub struct Chorus {
    buf:       Vec<f32>,
    write_ptr: usize,
    max_len:   usize,
    lfo:       [Phasor; 4],
    /// Base delay time in samples.
    pub delay_base:  f32,
    /// Modulation depth in samples.
    pub depth:       f32,
    /// Number of active voices [1..4].
    pub voices:      usize,
    /// Voice spread for stereo width.
    pub spread:      f32,
    /// Wet/dry [0, 1].
    pub mix:         f32,
    sample_rate:     f32,
}

impl Chorus {
    pub fn new(sample_rate: f32) -> Self {
        let max_len = (0.05 * sample_rate) as usize + 4;
        let lfo_rates = [0.8, 0.9, 1.1, 1.3];
        let lfo = std::array::from_fn(|i| {
            let mut p = Phasor::new(lfo_rates[i], sample_rate);
            p.phase = i as f32 * 0.25;
            p
        });
        Self {
            buf: vec![0.0; max_len],
            write_ptr: 0,
            max_len,
            lfo,
            delay_base: sample_rate * 0.007,
            depth:      sample_rate * 0.003,
            voices:     2,
            spread:     0.5,
            mix:        0.5,
            sample_rate,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        for (i, lfo) in self.lfo.iter_mut().enumerate() {
            lfo.set_freq(hz * (1.0 + i as f32 * 0.05), self.sample_rate);
        }
    }

    #[inline]
    fn read_interp(&self, delay_samples: f32) -> f32 {
        let d = delay_samples.max(1.0);
        let di = d as usize;
        let frac = d - di as f32;
        let idx = |off: usize| -> f32 {
            self.buf[(self.write_ptr + self.max_len - off) % self.max_len]
        };
        hermite4(idx(di.saturating_sub(1)), idx(di), idx(di+1), idx(di+2), frac)
    }

    /// Returns (left, right) wet signal.
    pub fn process(&mut self, x: f32) -> (f32, f32) {
        self.buf[self.write_ptr] = x;
        self.write_ptr = (self.write_ptr + 1) % self.max_len;

        let voices = self.voices.min(4);
        let mut l = 0.0f32;
        let mut r = 0.0f32;

        for i in 0..voices {
            let lfo_out = self.lfo[i].tick();
            let mod_val = (lfo_out * std::f32::consts::TAU).sin();
            let delay = self.delay_base + mod_val * self.depth;
            let s = self.read_interp(delay);
            let pan_l = 1.0 - (i as f32 / voices as f32) * self.spread;
            let pan_r = 1.0 - ((voices - 1 - i) as f32 / voices as f32) * self.spread;
            l += s * pan_l;
            r += s * pan_r;
        }

        let scale = 1.0 / voices as f32;
        (x * (1.0 - self.mix) + l * scale * self.mix,
         x * (1.0 - self.mix) + r * scale * self.mix)
    }

    pub fn reset(&mut self) {
        self.buf.fill(0.0);
    }
}
