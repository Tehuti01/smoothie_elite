//! Voice abstraction for polyphonic synthesis.

use smoothie_effects::{Envelope, Lfo, LfoShape};

/// Voice state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceState {
    /// Voice is idle and available
    Idle,
    /// Voice is sounding
    Active,
    /// Voice is in release stage
    Releasing,
}

/// A single synthesizer voice with modulation.
#[derive(Clone)]
pub struct Voice {
    pub state: VoiceState,
    pub note: u8,
    pub velocity: f32,

    // Modulation
    pub amp_env: Envelope,
    pub mod_env: Envelope,
    pub lfo1: Lfo,
    pub lfo2: Lfo,

    // Voice-specific state
    pub sample_rate: f32,
    pub phase: f32,
    pub filter_state: [f32; 2],

    // Age in samples (for voice stealing)
    pub age_samples: u64,
}

impl Voice {
    /// Create a new voice.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            state: VoiceState::Idle,
            note: 0,
            velocity: 0.0,
            amp_env: Envelope::new(0.01, 0.1, 0.7, 0.3, sample_rate),
            mod_env: Envelope::new(0.01, 0.2, 0.5, 0.5, sample_rate),
            lfo1: Lfo::new(5.0, LfoShape::Sine, sample_rate),
            lfo2: Lfo::new(7.0, LfoShape::Triangle, sample_rate),
            sample_rate,
            phase: 0.0,
            filter_state: [0.0; 2],
            age_samples: 0,
        }
    }

    /// Trigger this voice with a MIDI note and velocity.
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        self.note = note;
        self.velocity = velocity as f32 / 127.0;
        self.state = VoiceState::Active;
        self.phase = 0.0;
        self.age_samples = 0;
        self.amp_env.trigger();
        self.mod_env.trigger();
        self.lfo1.reset();
        self.lfo2.reset();
    }

    /// Release this voice.
    pub fn note_off(&mut self) {
        if self.state == VoiceState::Active {
            self.state = VoiceState::Releasing;
            self.amp_env.release();
            self.mod_env.release();
        }
    }

    /// Update voice sample rate.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.amp_env.set_sample_rate(sr);
        self.mod_env.set_sample_rate(sr);
        self.lfo1.set_sample_rate(sr);
        self.lfo2.set_sample_rate(sr);
    }

    /// Advance voice state by one sample.
    pub fn update(&mut self) {
        self.amp_env.next_sample();
        self.mod_env.next_sample();
        self.lfo1.next_sample();
        self.lfo2.next_sample();
        self.age_samples += 1;

        // Transition to Idle when fully released
        if self.state == VoiceState::Releasing && self.amp_env.is_idle() {
            self.state = VoiceState::Idle;
        }
    }

    /// Get current amplitude (velocity * envelope).
    pub fn amplitude(&self) -> f32 {
        self.velocity * self.amp_env.value()
    }

    /// Check if this voice is available for allocation.
    pub fn is_available(&self) -> bool {
        self.state == VoiceState::Idle
    }
}

impl Default for Voice {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
