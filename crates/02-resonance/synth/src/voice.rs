/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe7ce4216 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/voice.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::STANDARD_PITCH;
use smoothie_core::primitives::Sample;
use smoothie_dsp::envelope_mod::AdsrEnvelope;
use smoothie_midi::MidiMessage;

/// State of a single synthesizer voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the VoiceState enumeration.
pub enum VoiceState {
    /// Voice is inactive and available for new notes.
    Inactive,
    /// Voice is currently playing a note.
    Active,
    /// Voice is in its release phase (note off received but sound decaying).
    Releasing,
    /// Voice is being forcibly stolen and quickly faded out.
    Stealing,
}

///
/// Technical implementation of the Voice structure.
pub struct Voice<O> {
    pub oscillator: O,
    pub envelope: AdsrEnvelope,
    pub state: VoiceState,
    pub midi_note: u8,
    pub velocity: f32,
    pub frequency: f32,
    /// Used for voice stealing (oldest active voice).
    pub timestamp: u64,
}

impl<O: Default> Default for Voice<O> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            oscillator: O::default(),
            envelope: AdsrEnvelope::default(),
            state: VoiceState::Inactive,
            midi_note: 0,
            velocity: 0.0,
            frequency: STANDARD_PITCH,
            timestamp: 0,
        }
    }
}

impl<O> Voice<O> {
    /// Create a new voice.
    pub fn new(osc: O, env: AdsrEnvelope) -> Self {
        Self {
            oscillator: osc,
            envelope: env,
            state: VoiceState::Inactive,
            midi_note: 0,
            velocity: 0.0,
            frequency: STANDARD_PITCH,
            timestamp: 0,
        }
    }

    /// Trigger the voice with a MIDI note.
    pub fn note_on(&mut self, note: u8, velocity: f32, frequency: f32, timestamp: u64) {
        self.midi_note = note;
        self.velocity = velocity;
        self.frequency = frequency;
        self.timestamp = timestamp;
        self.state = VoiceState::Active;
        // The specific oscillator must be reset/tuned by the owning synth
        self.envelope.trigger();
    }

    /// Release the voice (note off).
    pub fn note_off(&mut self) {
        if self.state == VoiceState::Active {
            self.state = VoiceState::Releasing;
            self.envelope.release();
        }
    }

    /// Complete the release phase.
    pub fn finish(&mut self) {
        self.state = VoiceState::Inactive;
        self.envelope.reset();
    }

    /// Initiate rapid fade-out for voice stealing.
    pub fn steal(&mut self) {
        self.state = VoiceState::Stealing;
        // Fast release to avoid clicks
        self.envelope.set_release(5.0);
        self.envelope.release();
    }
}

/// Technical implementation of the VoiceAllocationMode enumeration.
pub enum VoiceAllocationMode {
    /// Re-use oldest voice.
    Oldest,
    /// Re-use lowest velocity voice.
    LowestVelocity,
    /// Round-robin allocation.
    RoundRobin,
}

/// Technical implementation of the VoiceAllocator structure.
pub struct VoiceAllocator<O, const N: usize> {
    voices: [Voice<O>; N],
    time: u64,
    mode: VoiceAllocationMode,
    next_robin: usize,
}

impl<O: Default, const N: usize> VoiceAllocator<O, N> {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        let voices: [Voice<O>; N] = core::array::from_fn(|_| Voice::default());
        Self {
            voices,
            time: 0,
            mode: VoiceAllocationMode::Oldest,
            next_robin: 0,
        }
    }

    /// Technical implementation of the set_mode logic.
    pub fn set_mode(&mut self, mode: VoiceAllocationMode) {
        self.mode = mode;
    }

    /// Process a block of samples for all active voices (mixin strategy).
    pub fn process_mix<F>(&mut self, process_fn: F) -> Sample
    where
        F: Fn(&mut Voice<O>) -> Sample,
    {
        let mut mixed = 0.0;
        self.time += 1;

        for voice in self.voices.iter_mut() {
            if voice.state != VoiceState::Inactive {
                let env_val = voice.envelope.process();

                // If envelope has finished decaying
                if voice.envelope.is_finished() {
                    voice.finish();
                    continue;
                }

                mixed += process_fn(voice) * env_val * voice.velocity;
            }
        }
        mixed
    }

    /// Handle a MIDI message, allocating or releasing voices.
    pub fn process_midi<F>(&mut self, msg: &MidiMessage, mut init_fn: F)
    where
        F: FnMut(&mut Voice<O>, f32),
    {
        match msg {
            MidiMessage::NoteOn { note, velocity, .. } => {
                let freq = smoothie_midi::note_to_frequency(*note);
                let vel_lin = smoothie_midi::velocity_to_amplitude(*velocity);

                let idx = self.allocate_voice(*note);
                self.voices[idx].note_on(*note, vel_lin, freq, self.time);
                init_fn(&mut self.voices[idx], freq);
            }
            MidiMessage::NoteOff { note, .. } => {
                for voice in self.voices.iter_mut() {
                    if voice.state == VoiceState::Active && voice.midi_note == *note {
                        voice.note_off();
                    }
                }
            }
            _ => {}
        }
    }

    /// Find an available voice, or steal one if none are available.
    fn allocate_voice(&mut self, _note: u8) -> usize {
        // First pass: look for Inactive voice
        if let Some(idx) = self
            .voices
            .iter()
            .position(|v| v.state == VoiceState::Inactive)
        {
            self.next_robin = (idx + 1) % N;
            return idx;
        }

        // Second pass: look for Releasing voice (preferably quietest)
        let mut best_idx = 0;
        let mut min_env = f32::MAX;

        for (i, voice) in self.voices.iter().enumerate() {
            if voice.state == VoiceState::Releasing {
                let env = voice.envelope.value();
                if env < min_env {
                    min_env = env;
                    best_idx = i;
                }
            }
        }
        if min_env < f32::MAX {
            self.voices[best_idx].steal();
            return best_idx;
        }

        // Third pass: voice stealing based on allocation mode
        match self.mode {
            VoiceAllocationMode::Oldest => {
                let mut oldest_time = u64::MAX;
                for (i, voice) in self.voices.iter().enumerate() {
                    if voice.timestamp < oldest_time {
                        oldest_time = voice.timestamp;
                        best_idx = i;
                    }
                }
            }
            VoiceAllocationMode::LowestVelocity => {
                let mut min_vel = f32::MAX;
                for (i, voice) in self.voices.iter().enumerate() {
                    if voice.velocity < min_vel {
                        min_vel = voice.velocity;
                        best_idx = i;
                    }
                }
            }
            VoiceAllocationMode::RoundRobin => {
                best_idx = self.next_robin;
                self.next_robin = (self.next_robin + 1) % N;
            }
        }

        self.voices[best_idx].steal();
        best_idx
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for voice in self.voices.iter_mut() {
            voice.finish();
        }
        self.time = 0;
    }

    /// Technical implementation of the voices_mut logic.
    pub fn voices_mut(&mut self) -> &mut [Voice<O>] {
        &mut self.voices
    }
}
