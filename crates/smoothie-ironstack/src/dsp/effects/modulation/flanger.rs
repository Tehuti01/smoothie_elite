use crate::audio::Sample;
use std::f32::consts::PI;

pub struct Flanger {
    sample_rate: f32,
    rate: f32,
    depth: f32,
    feedback: f32,
    mix: f32,
    lfo_phase: f32,
    delay_buffer: Vec<f32>,
    write_index: usize,
}

impl Flanger {
    pub fn new(sample_rate: u32) -> Self {
        let max_delay_samples = ((30.0 * sample_rate as f32 / 1000.0) as usize + 100).max(100);

        Self {
            sample_rate: sample_rate as f32,
            rate: 0.5,
            depth: 0.7,
            feedback: 0.5,
            mix: 0.5,
            lfo_phase: 0.0,
            delay_buffer: vec![0.0; max_delay_samples],
            write_index: 0,
        }
    }

    pub fn set_rate(&mut self, r: f32) {
        self.rate = r.clamp(0.01, 5.0);
    }
    pub fn set_depth(&mut self, d: f32) {
        self.depth = d.clamp(0.0, 1.0);
    }
    pub fn set_feedback(&mut self, f: f32) {
        self.feedback = f.clamp(-0.95, 0.95);
    }
    pub fn set_mix(&mut self, m: f32) {
        self.mix = m.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: Sample) -> Sample {
        let lfo = (self.lfo_phase.sin() * 0.5 + 0.5) * self.depth;
        self.lfo_phase += 2.0 * PI * self.rate / self.sample_rate;
        if self.lfo_phase > 2.0 * PI {
            self.lfo_phase -= 2.0 * PI;
        }

        let delay = 1.0 + lfo * 15.0;

        let idx = (self.write_index + self.delay_buffer.len() - delay as usize - 1)
            % self.delay_buffer.len();
        let idx2 = (idx + 1) % self.delay_buffer.len();

        let delayed = self.delay_buffer[idx]
            + (self.delay_buffer[idx2] - self.delay_buffer[idx]) * (delay - delay as f32);

        let mono = (input.left + input.right) * 0.5;
        self.delay_buffer[self.write_index] = mono + delayed * self.feedback;
        self.write_index = (self.write_index + 1) % self.delay_buffer.len();

        let wet = delayed * self.mix;
        let dry = 1.0 - self.mix * 0.5;

        Sample {
            left: input.left * dry + wet,
            right: input.right * dry + wet,
        }
    }

    pub fn reset(&mut self) {
        self.delay_buffer.fill(0.0);
        self.write_index = 0;
        self.lfo_phase = 0.0;
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let sample = Sample::new(left, right);
        let result = self.process(sample);
        (result.left, result.right)
    }
}
