/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x657692c3 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/fm_synth.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::TAU;
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;
use smoothie_dsp::envelope_mod::AdsrEnvelope;

const NUM_OPERATORS: usize = 4;

/// A single FM operator.
#[repr(align(64))]
/// Technical implementation of the FmOperator structure.
pub struct FmOperator {
    phase: f32,
    ratio: f32,
    envelope: AdsrEnvelope,
    level: f32,    // 0.0 to 1.0 (multiplier/gain)
    feedback: f32, // 0.0 to 1.0
    last_out: f32,
}

impl Default for FmOperator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            phase: 0.0,
            ratio: 1.0,
            envelope: AdsrEnvelope::default(),
            level: 1.0,
            feedback: 0.0,
            last_out: 0.0,
        }
    }
}

impl FmOperator {
    #[inline]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, base_freq: f32, phase_modulation: f32, sample_rate: f32) -> f32 {
        let env_val = self.envelope.next();
        let freq = base_freq * self.ratio;

        // Calculate phase increment
        let dt = freq / sample_rate;

        // Apply feedback and external phase modulation
        let total_phase = self.phase + phase_modulation + (self.last_out * self.feedback);

        // Generate sine wave
        let out = sine_approx(total_phase * TAU) * env_val * self.level;

        self.last_out = out;
        self.phase += dt;
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        out
    }

    /// Technical implementation of the trigger logic.
    pub fn trigger(&mut self) {
        self.envelope.trigger();
        self.phase = 0.0;
        self.last_out = 0.0;
    }

    /// Technical implementation of the release logic.
    pub fn release(&mut self) {
        self.envelope.release();
    }
}

/// The core FM Synthesizer handling 4 Operators and 8 Algorithms.
#[repr(align(64))]
/// Technical implementation of the FMSynth structure.
pub struct FMSynth {
    pub operators: [FmOperator; NUM_OPERATORS],
    pub algorithm: u8, // 0 to 7 matching classic DX routing
    sample_rate: f32,
}

impl Default for FMSynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            operators: core::array::from_fn(|_| FmOperator::default()),
            algorithm: 0,
            sample_rate: 44100.0,
        }
    }
}

impl FMSynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut synth = Self::default();
        synth.sample_rate = sample_rate;
        for op in synth.operators.iter_mut() {
            op.envelope.set_sample_rate(sample_rate);
        }
        synth
    }

    /// Process next sample using the selected FM routing algorithm.
    pub fn process(&mut self, frequency: f32) -> Sample {
        let sr = self.sample_rate;
        // Split borrows for operators
        let (op1, rest) = self.operators.split_at_mut(1);
        let (op2, rest2) = rest.split_at_mut(1);
        let (op3, op4_slice) = rest2.split_at_mut(1);
        let op1 = &mut op1[0];
        let op2 = &mut op2[0];
        let op3 = &mut op3[0];
        let op4 = &mut op4_slice[0];

        match self.algorithm {
            // Algorithm 0: 4->3->2->1 (Cascade)
            0 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, out4, sr);
                let out2 = op2.process(frequency, out3, sr);
                op1.process(frequency, out2, sr)
            }
            // Algorithm 1: 4->3 \
            //                    -> 1
            //                 2 /
            1 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, out4, sr);
                let out2 = op2.process(frequency, 0.0, sr);
                op1.process(frequency, out3 + out2, sr)
            }
            // Algorithm 2: 4->3 \
            //                    -> 2 -> 1
            2 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, out4, sr);
                let out2 = op2.process(frequency, out3, sr);
                op1.process(frequency, out2, sr)
            }
            // Algorithm 3: 4 \
            //                 -> 2 -> 1
            //              3 /
            3 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, 0.0, sr);
                let out2 = op2.process(frequency, out4 + out3, sr);
                op1.process(frequency, out2, sr)
            }
            // Algorithm 4: 4 -> 3 (Mix)
            //              2 -> 1 (Mix)
            4 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, out4, sr);
                let out2 = op2.process(frequency, 0.0, sr);
                let out1 = op1.process(frequency, out2, sr);
                (out3 + out1) * 0.5
            }
            // Algorithm 5: 4 \
            //              3 -> 1
            //              2 /
            5 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, 0.0, sr);
                let out2 = op2.process(frequency, 0.0, sr);
                op1.process(frequency, out4 + out3 + out2, sr)
            }
            // Algorithm 6: 4 -> 3
            //                   2 -> Mix
            //                   1 -> Mix
            6 => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, out4, sr);
                let out2 = op2.process(frequency, 0.0, sr);
                let out1 = op1.process(frequency, 0.0, sr);
                (out3 + out2 + out1) / 3.0
            }
            // Algorithm 7: All parallel mix
            _ => {
                let out4 = op4.process(frequency, 0.0, sr);
                let out3 = op3.process(frequency, 0.0, sr);
                let out2 = op2.process(frequency, 0.0, sr);
                let out1 = op1.process(frequency, 0.0, sr);
                (out4 + out3 + out2 + out1) * 0.25
            }
        }
    }
}
