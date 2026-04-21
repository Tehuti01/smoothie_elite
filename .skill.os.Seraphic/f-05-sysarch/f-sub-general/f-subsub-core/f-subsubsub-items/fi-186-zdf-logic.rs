---
id: fi-186-zdf-logic.rs
category: f-05-sysarch
---

pub struct ZdfLpf {
    pub g: f32,
    pub s1: f32,
}
impl ZdfLpf {
    pub fn process(&mut self, x: f32) -> f32 {
        let v = (x - self.s1) * (self.g / (1.0 + self.g));
        let y = v + self.s1;
        self.s1 = y + v;
        y
    }
}
