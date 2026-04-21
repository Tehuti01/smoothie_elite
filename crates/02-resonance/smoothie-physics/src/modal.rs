/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5c513f3a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/modal.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::constants::TAU;
use smoothie_core::math::{exp_approx, sine_approx};
use smoothie_core::primitives::Sample;

/// A single resonating mode (essentially an oscillating 2-pole filter with extremely high Q)
#[repr(align(64))]
/// Technical implementation of the ResonanceMode structure.
pub struct ResonanceMode {
    freq: f32,
    decay: f32,
    amplitude: f32,
    phase: f32,
    env: f32,
}

impl ResonanceMode {
    /// Initializes a new instance of the associated type.
    pub fn new(freq: f32, decay: f32, amplitude: f32) -> Self {
        Self {
            freq,
            decay,
            amplitude,
            phase: 0.0,
            env: 0.0,
        }
    }

    /// Excite the mode with an impulse energy
    pub fn strike(&mut self, energy: f32) {
        self.env += energy * self.amplitude;
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample_rate: f32) -> Sample {
        if self.env < 1e-6 {
            return 0.0;
        }

        let out = sine_approx(self.phase * TAU) * self.env;

        self.phase += self.freq / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Exponential decay
        self.env *= self.decay;

        out
    }
}

/// Bank of resonance modes representing a physical body (like a bell or plate)
#[repr(align(64))]
/// Technical implementation of the ModalBody structure.
pub struct ModalBody {
    modes: Vec<ResonanceMode>,
    sample_rate: f32,
}

impl ModalBody {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            modes: Vec::with_capacity(64),
            sample_rate,
        }
    }

    /// Performs vector addition logic.
    pub fn add_mode(&mut self, freq: f32, decay_time_ms: f32, amplitude: f32) {
        // Calculate per-sample multiplier for given decay time
        let decay_samples = decay_time_ms * 0.001 * self.sample_rate;
        let decay_mult = exp_approx(-1.0 / decay_samples);
        self.modes
            .push(ResonanceMode::new(freq, decay_mult, amplitude));
    }

    /// Technical implementation of the generate_bell logic.
    pub fn generate_bell(&mut self, fundamental: f32) {
        self.modes.clear();
        // Classical Bell partials logic
        let ratios = [1.0, 2.0, 2.4, 3.0, 3.6, 4.5, 5.3, 6.0, 7.5];
        let decays = [
            2000.0, 1500.0, 1000.0, 800.0, 600.0, 400.0, 200.0, 150.0, 100.0,
        ];
        let amplitudes = [1.0, 0.8, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.05];

        for i in 0..ratios.len() {
            self.add_mode(fundamental * ratios[i], decays[i], amplitudes[i]);
        }
    }

    /// Technical implementation of the strike logic.
    pub fn strike(&mut self, velocity: f32) {
        for mode in self.modes.iter_mut() {
            mode.strike(velocity);
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> Sample {
        let sr = self.sample_rate;
        let mut sum = 0.0;
        for mode in self.modes.iter_mut() {
            sum += mode.process(sr);
        }
        sum
    }
}
