/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x018756bc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/synthesis/modal/table_based.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// for bells, plates, membranes, and other resonant structures.

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::constants::TAU;
use smoothie_core::math::{exp_approx, sine_approx};
use smoothie_core::primitives::Sample;

#[repr(align(64))]
/// Technical implementation of the ModeTable structure.
pub struct ModeTable {
    pub frequency: f32,
    pub decay: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub quality_factor: f32,
}

#[repr(align(64))]
/// Technical implementation of the ModalSynthesizer structure.
pub struct ModalSynthesizer {
    modes: Vec<ModeTable>,
    sample_rate: f32,
    max_modes: usize,
    last_sample: f32,
}

impl ModalSynthesizer {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, max_modes: usize) -> Self {
        Self {
            modes: Vec::with_capacity(max_modes),
            sample_rate,
            max_modes,
            last_sample: 0.0,
        }
    }

    /// Technical implementation of the load_bell logic.
    pub fn load_bell(&mut self, fundamental: f32) {
        self.modes.clear();

        let ratios = [1.0, 2.0, 2.4, 3.0, 3.6, 4.5, 5.3, 6.0, 7.5, 8.2];
        let decays = [3.0, 2.5, 2.0, 1.5, 1.2, 0.8, 0.5, 0.3, 0.2, 0.15];
        let amplitudes = [1.0, 0.85, 0.7, 0.55, 0.45, 0.35, 0.25, 0.15, 0.08, 0.05];

        for i in 0..ratios.len().min(self.max_modes) {
            let freq = fundamental * ratios[i];
            let decay = exp_approx(-1.0 / (decays[i] * self.sample_rate));
            self.modes.push(ModeTable {
                frequency: freq,
                decay,
                amplitude: amplitudes[i],
                phase: 0.0,
                quality_factor: TAU * freq * decays[i],
            });
        }
    }

    /// Technical implementation of the load_plate logic.
    pub fn load_plate(&mut self, fundamental: f32) {
        self.modes.clear();

        let ratios = [
            1.0, 1.58, 2.0, 2.24, 2.92, 3.0, 3.45, 4.0, 4.25, 5.04, 5.19, 5.83, 6.0, 6.15, 7.0,
            7.21,
        ];

        for i in 0..ratios.len().min(self.max_modes) {
            let freq = fundamental * ratios[i];
            let decay = exp_approx(-1.0 / (1.5 * self.sample_rate));
            self.modes.push(ModeTable {
                frequency: freq,
                decay,
                amplitude: 1.0 / ratios[i] as f32,
                phase: 0.0,
                quality_factor: 50.0,
            });
        }
    }

    /// Technical implementation of the load_membrane logic.
    pub fn load_membrane(&mut self, fundamental: f32) {
        self.modes.clear();

        let m = 1;
        let n = 0;
        let ratios: Vec<f32> = (1..=4)
            .flat_map(|m| {
                (0..=4).map(move |n| {
                    let j0 = match (m, n) {
                        (1, 0) => 1.0,
                        (1, 1) => 1.593,
                        (2, 0) => 2.296,
                        (1, 2) => 2.653,
                        (2, 1) => 2.918,
                        (3, 0) => 3.5,
                        (1, 3) => 3.659,
                        (2, 2) => 3.832,
                        (3, 1) => 4.058,
                        (4, 0) => 4.231,
                        _ => (m as f32 + n as f32 * 0.5),
                    };
                    j0
                })
            })
            .collect();

        for (i, ratio) in ratios.iter().enumerate().take(self.max_modes) {
            let freq = fundamental * ratio;
            let decay = exp_approx(-1.0 / (0.8 * self.sample_rate));
            let amp = 1.0 / (i + 1) as f32;
            self.modes.push(ModeTable {
                frequency: freq,
                decay,
                amplitude: amp,
                phase: 0.0,
                quality_factor: 30.0,
            });
        }
    }

    /// Technical implementation of the strike logic.
    pub fn strike(&mut self, velocity: f32, position: Option<f32>) {
        for mode in self.modes.iter_mut() {
            let spatial_factor = if let Some(pos) = position {
                (pos * TAU).sin()
            } else {
                1.0
            };
            mode.phase = 0.0;
            mode.decay = mode.decay;
            mode.amplitude = velocity * spatial_factor;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> Sample {
        let sr = self.sample_rate;
        let mut sum = 0.0;

        for mode in self.modes.iter_mut() {
            if mode.amplitude.abs() > 1e-8 {
                sum += sine_approx(mode.phase) * mode.amplitude;
                mode.phase += mode.frequency / sr;
                mode.amplitude *= mode.decay;
            }
        }

        self.last_sample = sum;
        sum
    }

    /// Technical implementation of the get_output logic.
    pub fn get_output(&self) -> Sample {
        self.last_sample
    }

    /// Technical implementation of the num_active_modes logic.
    pub fn num_active_modes(&self) -> usize {
        self.modes
            .iter()
            .filter(|m| m.amplitude.abs() > 1e-8)
            .count()
    }
}

#[repr(align(64))]
/// Technical implementation of the ResonantFilter structure.
pub struct ResonantFilter {
    pub center_freq: f32,
    pub quality_factor: f32,
    pub gain: f32,
    state_x1: f32,
    state_x2: f32,
    state_y1: f32,
    state_y2: f32,
}

impl ResonantFilter {
    /// Initializes a new instance of the associated type.
    pub fn new(center_freq: f32, q: f32, sample_rate: f32) -> Self {
        Self {
            center_freq,
            quality_factor: q,
            gain: 1.0,
            state_x1: 0.0,
            state_x2: 0.0,
            state_y1: 0.0,
            state_y2: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: Sample, sample_rate: f32) -> Sample {
        let w0 = TAU * self.center_freq / sample_rate;
        let alpha = w0.sin() / (2.0 * self.quality_factor);
        let cos_w0 = w0.cos();

        let b0 = 1.0;
        let b1 = 0.0;
        let b2 = -1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        let output = (b0 * input + b1 * self.state_x1 + b2 * self.state_x2
            - a1 * self.state_y1
            - a2 * self.state_y2)
            / a0;

        self.state_x2 = self.state_x1;
        self.state_x1 = input;
        self.state_y2 = self.state_y1;
        self.state_y1 = output;

        output * self.gain
    }
}
