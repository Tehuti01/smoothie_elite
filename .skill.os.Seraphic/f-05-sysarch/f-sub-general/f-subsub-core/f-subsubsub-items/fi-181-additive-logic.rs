---
id: fi-181-additive-logic.rs
category: f-05-sysarch
---

pub struct HarmonicLattice {
    pub phases: Vec<f32>,
    pub freqs: Vec<f32>,
}
impl HarmonicLattice {
    pub fn process(&mut self) -> f32 {
        let mut out = 0.0;
        for i in 0..self.phases.len() {
            self.phases[i] = (self.phases[i] + self.freqs[i]) % 1.0;
            out += (self.phases[i] * 6.2831853).sin();
        }
        out
    }
}
