/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7380e080 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/monophonic.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::voice::Voice;
use smoothie_core::math::exp_approx;

use smoothie_core::constants::STANDARD_PITCH;
use smoothie_core::primitives::Sample;
use smoothie_dsp::oscillators::{Oscillator, OscillatorMode};

/// Technical implementation of the MonophonicSynth structure.
pub struct MonophonicSynth {
    pub voice: Voice<Oscillator>,
    sample_rate: f32,
    glide_time: f32,
    glide_current_freq: f32,
    glide_target_freq: f32,
}

impl MonophonicSynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut voice = Voice::default();
        voice.envelope.set_sample_rate(sample_rate);
        Self {
            voice,
            sample_rate,
            glide_time: 0.001,
            glide_current_freq: STANDARD_PITCH,
            glide_target_freq: STANDARD_PITCH,
        }
    }

    /// Technical implementation of the note_on logic.
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        let freq = smoothie_midi::note_to_frequency(note);
        self.glide_target_freq = freq;

        if self.glide_time <= 0.001 {
            self.glide_current_freq = self.glide_target_freq;
        }

        self.voice.note_on(note, velocity, freq, 0);
    }

    /// Technical implementation of the note_off logic.
    pub fn note_off(&mut self) {
        self.voice.note_off();
    }
    /// Technical implementation of the set_glide_time logic.
    pub fn set_glide_time(&mut self, time: f32) {
        self.glide_time = time.max(0.0001);
    }
    /// Technical implementation of the set_waveform logic.
    pub fn set_waveform(&mut self, mode: OscillatorMode) {
        self.voice.oscillator.set_mode(mode);
    }

    /// Generate next sample with exponential pitch glide
    pub fn next(&mut self) -> Sample {
        let delta_time = 1.0 / self.sample_rate;

        // Exponential Glide (Silicon stable pitch warping)
        let alpha = 1.0 - exp_approx(-delta_time / self.glide_time);
        self.glide_current_freq += alpha * (self.glide_target_freq - self.glide_current_freq);

        // Update voice's internal oscillator frequency
        self.voice.oscillator.set_frequency(self.glide_current_freq);

        if self.voice.state != crate::voice::VoiceState::Inactive {
            let env_val = self.voice.envelope.next();
            if self.voice.envelope.is_finished() {
                self.voice.finish();
                return 0.0;
            }
            return self.voice.oscillator.next() * env_val * self.voice.velocity;
        }

        0.0
    }

    /// Technical implementation of the generate_into logic.
    pub fn generate_into(&mut self, buffer: &mut [Sample]) {
        for i in 0..buffer.len() {
            buffer[i] = self.next();
        }
    }
}

impl Default for MonophonicSynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
