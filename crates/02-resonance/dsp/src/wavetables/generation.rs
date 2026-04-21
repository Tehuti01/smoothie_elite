/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5366bc7b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/dsp/src/wavetables/generation.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::constants::{F_233, TAU};
use smoothie_core::math::sine_approx;
use smoothie_core::math::FloatExt;

/// Technical implementation of the WavetableGenerator structure.
pub struct WavetableGenerator {
    table: [f32; F_233],
}

impl WavetableGenerator {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            table: [0.0; F_233],
        }
    }

    /// Technical implementation of the generate_sine logic.
    pub fn generate_sine(&mut self) -> &[f32; F_233] {
        for i in 0..F_233 {
            self.table[i] = sine_approx((i as f32 / F_233 as f32) * TAU);
        }
        &self.table
    }

    /// Technical implementation of the generate_saw logic.
    pub fn generate_saw(&mut self) -> &[f32; F_233] {
        for i in 0..F_233 {
            let phase = i as f32 / F_233 as f32;
            self.table[i] = 2.0 * (1.0 - phase) - 1.0;
        }
        &self.table
    }

    /// Technical implementation of the generate_square logic.
    pub fn generate_square(&mut self, duty: f32) -> &[f32; F_233] {
        let duty = duty.clamp(0.1, 0.9);
        for i in 0..F_233 {
            let phase = i as f32 / F_233 as f32;
            self.table[i] = if phase < duty { 1.0 } else { -1.0 };
        }
        &self.table
    }

    /// Technical implementation of the generate_triangle logic.
    pub fn generate_triangle(&mut self) -> &[f32; F_233] {
        for i in 0..F_233 {
            let phase = i as f32 / F_233 as f32;
            self.table[i] = if phase < 0.5 {
                4.0 * phase - 1.0
            } else {
                3.0 - 4.0 * phase
            };
        }
        &self.table
    }

    /// Technical implementation of the generate_harmonics logic.
    pub fn generate_harmonics(&mut self, harmonics: &[f32]) -> &[f32; F_233] {
        self.table = [0.0; F_233];
        for (h, &amp) in harmonics.iter().enumerate() {
            if amp.abs() > 1e-6 {
                let harmonic = (h + 1) as f32;
                for i in 0..F_233 {
                    let phase = (i as f32 / F_233 as f32) * TAU * harmonic;
                    self.table[i] += amp * sine_approx(phase);
                }
            }
        }
        let max_val = self.table.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        if max_val > 0.0 {
            for sample in &mut self.table {
                *sample /= max_val;
            }
        }
        &self.table
    }

    /// Technical implementation of the generate_noise logic.
    pub fn generate_noise(&mut self) -> &[f32; F_233] {
        for i in 0..F_233 {
            let x = (i as u32).wrapping_mul(1103515245).wrapping_add(12345);
            self.table[i] = (x as f32 / u32::MAX as f32) * 2.0 - 1.0;
        }
        &self.table
    }

    /// Technical implementation of the get_table logic.
    pub fn get_table(&self) -> &[f32; F_233] {
        &self.table
    }
}

impl Default for WavetableGenerator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug)]
/// Technical implementation of the VintageShape enumeration.
pub enum VintageShape {
    Pew,
    Square59,
    Pwm,
}

/// Technical implementation of the generate_from_samples logic.
pub fn generate_from_samples(samples: &[f32], table_size: usize) -> Vec<f32> {
    let mut table = Vec::with_capacity(table_size);
    let step = samples.len() as f32 / table_size as f32;
    for i in 0..table_size {
        let pos = i as f32 * step;
        let i0 = pos as usize;
        let i1 = (i0 + 1).min(samples.len() - 1);
        let frac = pos - (i0 as f32);
        table.push(samples[i0] * (1.0 - frac) + samples[i1] * frac);
    }
    table
}

/// Technical implementation of the normalize logic.
pub fn normalize(table: &mut [f32]) {
    let max_val = table.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
    if max_val > 0.0 {
        for sample in table {
            *sample /= max_val;
        }
    }
}
