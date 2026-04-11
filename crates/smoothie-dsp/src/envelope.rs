//! ADSR envelope generator.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvelopeState { Idle, Attack, Decay, Sustain, Release }

pub struct AdsrEnvelope {
    state:         EnvelopeState,
    level:         f32,
    attack_rate:   f32,
    decay_rate:    f32,
    sustain_level: f32,
    release_rate:  f32,
    sample_rate:   f32,
}

impl AdsrEnvelope {
    pub fn new(sample_rate: f32) -> Self {
        let mut e = Self {
            state: EnvelopeState::Idle,
            level: 0.0,
            attack_rate: 0.0,
            decay_rate: 0.0,
            sustain_level: 0.7,
            release_rate: 0.0,
            sample_rate,
        };
        e.set_adsr(10.0, 100.0, 0.7, 200.0);
        e
    }

    /// Set ADSR times in milliseconds and sustain level in [0, 1].
    pub fn set_adsr(&mut self, attack_ms: f32, decay_ms: f32, sustain: f32, release_ms: f32) {
        let sr = self.sample_rate;
        self.attack_rate   = 1.0 / (attack_ms  * 0.001 * sr).max(1.0);
        self.decay_rate    = 1.0 / (decay_ms   * 0.001 * sr).max(1.0);
        self.sustain_level = sustain.clamp(0.0, 1.0);
        self.release_rate  = 1.0 / (release_ms * 0.001 * sr).max(1.0);
    }

    /// Trigger note-on.
    pub fn note_on(&mut self)  { self.state = EnvelopeState::Attack; }

    /// Trigger note-off.
    pub fn note_off(&mut self) { self.state = EnvelopeState::Release; }

    /// Advance one sample. Returns envelope value in [0, 1].
    #[inline]
    pub fn next_sample(&mut self) -> f32 {
        match self.state {
            EnvelopeState::Idle => {}
            EnvelopeState::Attack => {
                self.level += self.attack_rate;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.state = EnvelopeState::Decay;
                }
            }
            EnvelopeState::Decay => {
                self.level -= self.decay_rate;
                if self.level <= self.sustain_level {
                    self.level = self.sustain_level;
                    self.state = EnvelopeState::Sustain;
                }
            }
            EnvelopeState::Sustain => {}
            EnvelopeState::Release => {
                self.level -= self.release_rate;
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.state = EnvelopeState::Idle;
                }
            }
        }
        self.level
    }

    pub fn state(&self)  -> EnvelopeState { self.state }
    pub fn level(&self)  -> f32           { self.level }
    pub fn is_active(&self) -> bool       { self.state != EnvelopeState::Idle }
}
