/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc8adbf93 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/lfo/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::sine_approx;

/// All available LFO waveform shapes.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the LfoShape enumeration.
pub enum LfoShape {
    /// Smooth sinusoidal oscillation.
    Sine,
    /// Linear triangle wave — bipolar, ramping up then down.
    Triangle,
    /// Forward sawtooth — rising ramp, instant reset.
    Sawtooth,
    /// Reverse sawtooth — falling ramp, instant reset.
    ReverseSawtooth,
    /// Bipolar square wave (duty cycle = 50%).
    Square,
    /// Pulse wave with configurable duty cycle.
    Pulse { duty_cycle: f32 },
    /// Sample-and-hold random — steps to a new random value each cycle.
    SampleAndHold,
    /// Smooth random — S&H values interpolated with cubic smoothing.
    SmoothRandom,
    /// Exponential sawtooth — rises slowly, snaps back instantly.
    ExponentialSaw,
}

/// LFO rate as absolute Hz or a tempo-synced beat division.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the LfoRate enumeration.
pub enum LfoRate {
    /// Free-running at a fixed frequency in Hz.
    FreqHz(f32),
    /// Tempo-synced: `beats` complete oscillations per `beats` host beats.
    /// e.g. `BeatSync { numerator: 3, denominator: 8 }` = 3/8 note.
    BeatSync { numerator: u32, denominator: u32 },
}

/// Complete LFO parameter bundle.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the LfoParams structure.
pub struct LfoParams {
    pub shape: LfoShape,
    pub rate: LfoRate,
    /// Phase offset applied at LFO start [0.0, 1.0].
    pub phase_offset: f32,
    /// Output depth/amount in normalised units [0.0, 1.0].
    pub depth: f32,
    /// Bipolar output if true, unipolar [0.0, 1.0] if false.
    pub bipolar: bool,
    /// Smoothing amount applied to the raw LFO output [0.0, 0.9999].
    pub smoothing: f32,
}

impl Default for LfoParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            shape: LfoShape::Sine,
            rate: LfoRate::FreqHz(1.0),
            phase_offset: 0.0,
            depth: 1.0,
            bipolar: true,
            smoothing: 0.0,
        }
    }
}

/// Technical implementation of the Lfo structure.
pub struct Lfo {
    params: LfoParams,
    phase: f32,
    phase_increment: f32,
    /// Current output value (after smoothing).
    output: f32,
    /// State for smooth-random interpolation.
    prev_random: f32,
    next_random: f32,
    /// State for sample-and-hold.
    held_value: f32,
    /// Noise seed (XorShift32 PRNG).
    seed: u32,
    sample_rate: f32,
}

impl Lfo {
    /// Initializes a new instance of the associated type.
    pub fn new(params: LfoParams, sample_rate: f32) -> Self {
        let mut lfo = Self {
            phase: params.phase_offset,
            phase_increment: 0.0,
            output: 0.0,
            prev_random: 0.0,
            next_random: 0.0,
            held_value: 0.0,
            seed: 0xDEAD_BEEF,
            params,
            sample_rate,
        };
        lfo.recompute_increment(120.0);
        lfo
    }

    /// Update phase increment from host BPM.
    pub fn recompute_increment(&mut self, host_bpm: f32) {
        self.phase_increment = match self.params.rate {
            LfoRate::FreqHz(hz) => hz / self.sample_rate,
            LfoRate::BeatSync {
                numerator,
                denominator,
            } => {
                let beats_per_cycle = numerator as f32 / denominator as f32;
                let hz = host_bpm / 60.0 / beats_per_cycle;
                hz / self.sample_rate
            }
        };
    }

    /// Advance the LFO by one sample and return the current output value.
    #[inline(always)]
    /// Technical implementation of the tick logic.
    pub fn tick(&mut self) -> f32 {
        let prev_phase = self.phase;
        self.phase += self.phase_increment;

        // Phase wrapping with integer subtraction (avoids FP modulo artefacts)
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            // Trigger sample-and-hold on wrap boundary
            match self.params.shape {
                LfoShape::SampleAndHold | LfoShape::SmoothRandom => {
                    self.prev_random = self.next_random;
                    self.next_random =
                        xorshift32(&mut self.seed) as f32 / u32::MAX as f32 * 2.0 - 1.0;
                    self.held_value = self.prev_random;
                }
                _ => {}
            }
        }

        let raw = self.compute_raw_output(self.phase, prev_phase);

        // Unipolar conversion
        let shaped = if self.params.bipolar {
            raw
        } else {
            raw * 0.5 + 0.5
        };

        // Smoothing (one-pole IIR)
        let s = self.params.smoothing;
        self.output = self.output * s + shaped * (1.0 - s);

        self.output * self.params.depth
    }

    #[inline(always)]
    /// Technical implementation of the compute_raw_output logic.
    fn compute_raw_output(&self, phi: f32, _prev: f32) -> f32 {
        match self.params.shape {
            LfoShape::Sine => sine_approx(phi),

            LfoShape::Triangle => {
                if phi < 0.5 {
                    4.0 * phi - 1.0
                } else {
                    3.0 - 4.0 * phi
                }
            }

            LfoShape::Sawtooth => 2.0 * phi - 1.0,

            LfoShape::ReverseSawtooth => 1.0 - 2.0 * phi,

            LfoShape::Square => {
                if phi < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }

            LfoShape::Pulse { duty_cycle } => {
                if phi < duty_cycle.clamp(0.01, 0.99) {
                    1.0
                } else {
                    -1.0
                }
            }

            LfoShape::SampleAndHold => self.held_value,

            LfoShape::SmoothRandom => {
                // Cubic Hermite interpolation between prev and next random values
                let t = phi;
                let t2 = t * t;
                let t3 = t2 * t;
                let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
                let h10 = t3 - 2.0 * t2 + t;
                let h01 = -2.0 * t3 + 3.0 * t2;
                let h11 = t3 - t2;
                h00 * self.prev_random + h10 * 0.0 + h01 * self.next_random + h11 * 0.0
            }

            LfoShape::ExponentialSaw => {
                // Exponential rise from -1 to 1, then instant reset
                // exp(4·φ - 2) normalised to [-1, 1]
                let e = smoothie_core::math::exp_approx(4.0 * phi - 2.0);
                (e - 0.1353) / (7.389 - 0.1353) * 2.0 - 1.0
            }
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.phase = self.params.phase_offset;
        self.output = 0.0;
        self.held_value = 0.0;
        self.prev_random = 0.0;
        self.next_random = 0.0;
    }
}

/// XorShift32 — minimal, high-quality, zero-allocation PRNG.
#[inline(always)]
/// Technical implementation of the xorshift32 logic.
fn xorshift32(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}
