//! Polyphonic synthesizer framework.

use crate::voice::Voice;
use crate::wavetable::WavetableEngine;

const MAX_VOICES: usize = 32;

/// Voice allocation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// Oldest voice gets stolen
    OldestVoice,
    /// Quietest (lowest amplitude) voice gets stolen
    QuietestVoice,
    /// Lowest-pitched voice gets stolen
    LowestPitch,
}

/// Polyphonic synthesizer.
pub struct PolySynth {
    voices: [Voice; MAX_VOICES],
    wavetables: WavetableEngine,
    allocation_strategy: AllocationStrategy,
    active_voices: u32,
    sample_rate: f32,

    // Global modulation
    master_tune: f32,
    master_level: f32,
    unison_voices: u8,
    unison_detune: f32,
}

impl PolySynth {
    /// Create a new polyphonic synthesizer.
    pub fn new(sample_rate: f32) -> Self {
        let mut voices = std::array::from_fn(|_| {
            let mut voice = Voice::default();
            voice.set_sample_rate(sample_rate);
            voice
        });

        Self {
            voices,
            wavetables: WavetableEngine::new(sample_rate, 2048),
            allocation_strategy: AllocationStrategy::OldestVoice,
            active_voices: 0,
            sample_rate,
            master_tune: 0.0,
            master_level: 1.0,
            unison_voices: 1,
            unison_detune: 0.0,
        }
    }

    /// Trigger a note on.
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        // Find available voice
        let voice_idx = self.find_available_voice();
        self.voices[voice_idx].note_on(note, velocity);
        self.active_voices = self.active_voices.saturating_add(1);
    }

    /// Trigger a note off.
    pub fn note_off(&mut self, note: u8) {
        for voice in self.voices.iter_mut() {
            if voice.note == note {
                voice.note_off();
                self.active_voices = self.active_voices.saturating_sub(1);
            }
        }
    }

    /// Find an available voice using the allocation strategy.
    fn find_available_voice(&self) -> usize {
        // First, try to find an idle voice
        for (i, voice) in self.voices.iter().enumerate() {
            if voice.is_available() {
                return i;
            }
        }

        // If no idle voices, steal according to strategy
        match self.allocation_strategy {
            AllocationStrategy::OldestVoice => {
                let mut oldest_idx = 0;
                let mut oldest_age = 0u64;
                for (i, voice) in self.voices.iter().enumerate() {
                    if voice.age_samples > oldest_age {
                        oldest_age = voice.age_samples;
                        oldest_idx = i;
                    }
                }
                oldest_idx
            }
            AllocationStrategy::QuietestVoice => {
                let mut quietest_idx = 0;
                let mut quietest_level = f32::MAX;
                for (i, voice) in self.voices.iter().enumerate() {
                    let level = voice.amplitude();
                    if level < quietest_level {
                        quietest_level = level;
                        quietest_idx = i;
                    }
                }
                quietest_idx
            }
            AllocationStrategy::LowestPitch => {
                let mut lowest_idx = 0;
                let mut lowest_note = 127u8;
                for (i, voice) in self.voices.iter().enumerate() {
                    if voice.note < lowest_note {
                        lowest_note = voice.note;
                        lowest_idx = i;
                    }
                }
                lowest_idx
            }
        }
    }

    /// Process one sample of audio.
    pub fn process(&mut self) -> f32 {
        let mut output = 0.0;

        // Update all voices
        for voice in self.voices.iter_mut() {
            voice.update();

            if !voice.is_available() {
                // Synthesize this voice
                let freq = note_to_freq(voice.note) * (1.0 + self.master_tune);
                let phase_inc = freq / self.sample_rate;
                voice.phase = (voice.phase + phase_inc) % 1.0;

                // Sample wavetable
                let sample = self.wavetables.current().sample_interpolated(voice.phase);

                // Apply amplitude envelope
                let amplitude = voice.amplitude();
                output += sample * amplitude;
            }
        }

        // Apply master level
        output * self.master_level / (self.voices.len() as f32).sqrt()
    }

    /// Set voice allocation strategy.
    pub fn set_allocation(&mut self, strategy: AllocationStrategy) {
        self.allocation_strategy = strategy;
    }

    /// Set waveform (0=sine, 1=tri, 2=saw, 3=square).
    pub fn set_waveform(&mut self, waveform: usize) {
        self.wavetables.set_waveform(waveform);
    }

    /// Set master tuning in semitones.
    pub fn set_master_tune(&mut self, semitones: f32) {
        self.master_tune = 2.0_f32.powf(semitones / 12.0) - 1.0;
    }

    /// Set master volume level.
    pub fn set_master_level(&mut self, level: f32) {
        self.master_level = level.clamp(0.0, 2.0);
    }

    /// Set unison (voice doubling) count.
    pub fn set_unison(&mut self, voices: u8, detune: f32) {
        self.unison_voices = voices.max(1);
        self.unison_detune = detune;
    }

    /// Get number of active voices.
    pub fn active_voice_count(&self) -> u32 {
        self.active_voices
    }

    /// Reset all voices.
    pub fn reset(&mut self) {
        for voice in self.voices.iter_mut() {
            voice.state = crate::voice::VoiceState::Idle;
        }
        self.active_voices = 0;
    }

    /// Set sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        for voice in self.voices.iter_mut() {
            voice.set_sample_rate(sr);
        }
    }

    /// Set ADSR envelope parameters (attack, decay, sustain, release in seconds).
    pub fn set_envelope(&mut self, attack: f32, decay: f32, sustain: f32, release: f32) {
        for voice in self.voices.iter_mut() {
            voice.amp_env.set_attack(attack);
            voice.amp_env.set_decay(decay);
            voice.amp_env.set_sustain(sustain);
            voice.amp_env.set_release(release);
        }
    }
}

/// Convert MIDI note number to frequency in Hz.
fn note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

impl Default for PolySynth {
    fn default() -> Self {
        Self::new(44100.0)
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
