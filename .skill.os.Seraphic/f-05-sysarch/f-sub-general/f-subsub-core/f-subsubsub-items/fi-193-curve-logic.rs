---
id: fi-193-curve-logic.rs
category: f-05-sysarch
---

pub fn phi_interpolate(a: f32, b: f32, t: f32) -> f32 {
    let phi = 1.618034;
    let t_phi = t.powf(phi);
    a * (1.0 - t_phi) + b * t_phi
}
