---
id: fi-191-lstm-logic.rs
category: f-05-sysarch
---

pub struct LstmCell {
    pub h: Vec<f32>,
    pub c: Vec<f32>,
}
impl LstmCell {
    pub fn forward(&mut self, x: &[f32]) {
        // Gates logic for industrial cognition...
    }
}
