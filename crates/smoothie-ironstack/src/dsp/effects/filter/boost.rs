pub struct Boost {
    gain: f32,
    level: f32,
}

impl Boost {
    pub fn new() -> Self {
        Self {
            gain: 1.0,
            level: 0.0,
        }
    }

    pub fn set_gain(&mut self, gain_db: f32) {
        self.gain = 10.0_f32.powf(gain_db / 20.0);
    }

    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        input * self.gain * (1.0 - self.level) + input * self.level
    }

    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        (self.process(left), self.process(right))
    }
}

impl Default for Boost {
    fn default() -> Self {
        Self::new()
    }
}
