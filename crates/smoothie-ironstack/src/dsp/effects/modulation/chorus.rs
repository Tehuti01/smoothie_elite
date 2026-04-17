use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Chorus {
    sample_rate: f32,
    rate: f32,
    depth: f32,
    mix: f32,
    delay_base: f32,
    lfo_phase: f32,
    lfo_phase2: f32,
    delay_buffer: Vec<f32>,
    write_index: usize,
}

impl Chorus {
    pub fn new(sample_rate: u32) -> Self {
        let max_delay_samples = ((50.0 * sample_rate as f32 / 1000.0) as usize + 100).max(100);

        Self {
            sample_rate: sample_rate as f32,
            rate: 1.5,
            depth: 0.5,
            mix: 0.5,
            delay_base: 7.0,
            lfo_phase: 0.0,
            lfo_phase2: PI / 3.0,
            delay_buffer: vec![0.0; max_delay_samples],
            write_index: 0,
        }
    }

    pub fn set_rate(&mut self, r: f32) {
        self.rate = r.clamp(0.1, 10.0);
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d.clamp(0.0, 1.0);
    }
    pub fn set_mix(&mut self, m: f32) {
        self.mix = m.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: Sample) -> Sample {
        let lfo1 = (self.lfo_phase.sin() * 0.5 + 0.5) * self.depth;
        let lfo2 = (self.lfo_phase2.sin() * 0.5 + 0.5) * self.depth;

        self.lfo_phase += 2.0 * PI * self.rate / self.sample_rate;
        self.lfo_phase2 += 2.0 * PI * self.rate * 1.1 / self.sample_rate;

        if self.lfo_phase > 2.0 * PI {
            self.lfo_phase -= 2.0 * PI;
        }
        if self.lfo_phase2 > 2.0 * PI {
            self.lfo_phase2 -= 2.0 * PI;
        }

        let delay_l = self.delay_base + lfo1 * 20.0;
        let delay_r = self.delay_base + lfo2 * 20.0;

        let delayed_l = self.read_delay(delay_l);
        let delayed_r = self.read_delay(delay_r);

        self.delay_buffer[self.write_index] = (input.left + input.right) * 0.5;
        self.write_index = (self.write_index + 1) % self.delay_buffer.len();

        let wet_l = delayed_l * self.mix;
        let wet_r = delayed_r * self.mix;
        let dry = 1.0 - self.mix * 0.5;

        Sample {
            left: input.left * dry + wet_l,
            right: input.right * dry + wet_r,
        }
    }

    fn read_delay(&self, delay: f32) -> f32 {
        let delay_int = delay as usize;
        let delay_frac = delay - delay_int as f32;

        let idx =
            (self.write_index + self.delay_buffer.len() - delay_int - 1) % self.delay_buffer.len();
        let idx2 = (idx + 1) % self.delay_buffer.len();

        let s1 = self.delay_buffer[idx];
        let s2 = self.delay_buffer[idx2];

        s1 + (s2 - s1) * delay_frac
    }

    pub fn reset(&mut self) {
        self.delay_buffer.fill(0.0);
        self.write_index = 0;
        self.lfo_phase = 0.0;
        self.lfo_phase2 = PI / 3.0;
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let sample = Sample::new(left, right);
        let result = self.process(sample);
        (result.left, result.right)
    }
}
