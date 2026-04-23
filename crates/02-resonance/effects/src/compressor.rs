/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8bd13366 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/compressor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::{amplitude_to_db, db_to_amplitude, exp_approx};
use smoothie_core::primitives::Sample;

/// Technical implementation of the Compressor structure.
pub struct Compressor {
    threshold: f32, // dB
    ratio: f32,     // Compression ratio
    attack_coeff: f32,
    release_coeff: f32,
    makeup_gain: f32, // dB
    sample_rate: f32,
    envelope: f32, // Current envelope level (dB)
}

impl Compressor {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut comp = Self {
            threshold: -20.0,
            ratio: 4.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            makeup_gain: 0.0,
            sample_rate,
            envelope: -60.0, // Start quiet
        };
        comp.update_coeffs(0.005, 0.1); // 5ms, 100ms
        comp
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self, attack: f32, release: f32) {
        // Coeff = 1 - e^(-1 / (time * sr))
        self.attack_coeff = 1.0 - exp_approx(-1.0 / (attack * self.sample_rate));
        self.release_coeff = 1.0 - exp_approx(-1.0 / (release * self.sample_rate));
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
    /// Technical implementation of the set_ratio logic.
    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.max(1.0);
    }
    /// Technical implementation of the set_makeup_gain logic.
    pub fn set_makeup_gain(&mut self, gain: f32) {
        self.makeup_gain = gain;
    }

    /// Technical implementation of the set_attack logic.
    pub fn set_attack(&mut self, attack: f32) {
        let a = attack.max(0.0001);
        self.attack_coeff = 1.0 - exp_approx(-1.0 / (a * self.sample_rate));
    }

    /// Technical implementation of the set_release logic.
    pub fn set_release(&mut self, release: f32) {
        let r = release.max(0.001);
        self.release_coeff = 1.0 - exp_approx(-1.0 / (r * self.sample_rate));
    }

    /// Process sample through high-fidelity log-domain compression
    pub fn process(&mut self, input: Sample) -> Sample {
        // 1. Level Detection (RMS or Peak) - Using Peak for simplicity
        let abs_input = input.abs();
        let input_db = amplitude_to_db(abs_input);

        // 2. Ballistics (Attack/Release in DB domain for transparency)
        let diff = input_db - self.envelope;
        if diff > 0.0 {
            self.envelope += self.attack_coeff * diff;
        } else {
            self.envelope += self.release_coeff * diff;
        }

        // 3. Gain Computer
        let mut gain_reducer_db = 0.0;
        if self.envelope > self.threshold {
            // Simple hard knee
            gain_reducer_db = (self.threshold - self.envelope) * (1.0 - 1.0 / self.ratio);
        }

        // 4. Conversion and Apply
        let total_gain = db_to_amplitude(gain_reducer_db + self.makeup_gain);
        input * total_gain
    }

    /// Primary real-time signal processing execution block.
    pub fn process_into(&mut self, input: &[Sample], output: &mut [Sample]) {
        for i in 0..input.len().min(output.len()) {
            output[i] = self.process(input[i]);
        }
    }
}

/// Technical implementation of the Limiter structure.
pub struct Limiter {
    comp: Compressor,
}

impl Limiter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut l = Self {
            comp: Compressor::new(sample_rate),
        };
        l.comp.set_ratio(50.0);
        l.comp.set_attack(0.0001); // 0.1ms
        l.comp.set_release(0.05); // 50ms
        l
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.comp.set_threshold(threshold_db);
    }
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        self.comp.process(input)
    }
}

impl Default for Compressor {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for Limiter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
