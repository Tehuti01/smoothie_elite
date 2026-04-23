/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x06217335 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/unison.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// Multiple detuned oscillators stacked for thick, chorus-like sounds.
extern crate alloc;

use smoothie_core::constants::TAU;
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;

/// Unison voice stacking configuration.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the UnisonConfig structure.
pub struct UnisonConfig {
    pub voices: u8,
    pub spread_cents: f32,
    pub stereo_spread: f32,
    pub phase_randomize: bool,
}

impl Default for UnisonConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            voices: 4,
            spread_cents: 10.0,
            stereo_spread: 0.3,
            phase_randomize: true,
        }
    }
}

/// A single unison voice (detuned oscillator pair).
#[derive(Clone, Copy)]
/// Technical implementation of the UnisonVoice structure.
pub struct UnisonVoice {
    pub left_phase: f32,
    pub right_phase: f32,
    pub detune_cents: f32,
    pub velocity: f32,
}

impl UnisonVoice {
    /// Initializes a new instance of the associated type.
    pub fn new(detune_cents: f32, stereo_spread: f32) -> Self {
        Self {
            left_phase: 0.0,
            right_phase: stereo_spread * 0.5,
            detune_cents,
            velocity: 1.0,
        }
    }
}

/// Technical implementation of the UnisonStack structure.
pub struct UnisonStack<const N: usize> {
    voices: [UnisonVoice; N],
    config: UnisonConfig,
    sample_rate: f32,
}

impl<const N: usize> UnisonStack<N> {
    /// Initializes a new instance of the associated type.
    pub fn new(config: UnisonConfig, sample_rate: f32) -> Self {
        let mut voices = [UnisonVoice::new(0.0, 0.0); N];
        let spread = config.spread_cents / (N - 1).max(1) as f32;

        for (i, voice) in voices.iter_mut().enumerate() {
            let detune = if N > 1 {
                -config.spread_cents * 0.5 + i as f32 * spread
            } else {
                0.0
            };
            *voice = UnisonVoice::new(detune, config.stereo_spread);
        }

        Self {
            voices,
            config,
            sample_rate,
        }
    }

    /// Generate next stereo sample from all stacked voices.
    pub fn next_stereo(&mut self, base_freq: f32) -> (Sample, Sample) {
        let mut left = 0.0f32;
        let mut right = 0.0f32;

        let freq_multiplier =
            |cents: f32| -> f32 { smoothie_core::math::exp_approx(cents * 0.0005729955_f32) };

        let inv_sr = 1.0 / self.sample_rate;
        let norm = 1.0 / N as f32;

        for voice in self.voices.iter_mut() {
            let freq = base_freq * freq_multiplier(voice.detune_cents);
            let phase_inc = freq * inv_sr;

            // Left channel
            voice.left_phase += phase_inc;
            if voice.left_phase >= 1.0 {
                voice.left_phase -= 1.0;
            }
            let l = sine_approx(voice.left_phase * TAU);

            // Right channel (with slight phase offset for stereo width)
            voice.right_phase += phase_inc;
            if voice.right_phase >= 1.0 {
                voice.right_phase -= 1.0;
            }
            let r = sine_approx(voice.right_phase * TAU);

            left += l;
            right += r;
        }

        (left * norm, right * norm)
    }

    /// Reset all phases.
    pub fn reset(&mut self) {
        for voice in self.voices.iter_mut() {
            voice.left_phase = 0.0;
            voice.right_phase = if self.config.phase_randomize {
                smoothie_core::math::sine_approx(
                    (voice.detune_cents + 1000.0).to_bits() as f32 * 1e-9,
                )
            } else {
                0.0
            };
        }
    }

    /// Update configuration.
    pub fn set_config(&mut self, config: UnisonConfig) {
        self.config = config;
    }
}
