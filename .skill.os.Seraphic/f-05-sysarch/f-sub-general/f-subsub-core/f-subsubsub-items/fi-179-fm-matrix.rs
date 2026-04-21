---
id: fi-179-fm-matrix.rs
category: f-05-sysarch
---

pub struct FmOperator {
    pub frequency: f32,
    pub phase: f32,
    pub amplitude: f32,
}

impl FmOperator {
    pub fn next(&mut self, modulation: f32) -> f32 {
        self.phase += self.frequency + modulation;
        self.phase %= 1.0;
        (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude
    }
}
