/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcf35d5ae | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/polyphonic.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::voice::{VoiceAllocationMode, VoiceAllocator};
use smoothie_core::primitives::Sample;
use smoothie_dsp::oscillators::{Oscillator, OscillatorMode};
use smoothie_midi::MidiMessage;

/// Technical implementation of the PolyphonicSynth structure.
pub struct PolyphonicSynth {
    pub allocator: VoiceAllocator<Oscillator, 32>,
    sample_rate: f32,
    mode: OscillatorMode,
}

impl PolyphonicSynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut allocator = VoiceAllocator::new();
        allocator.set_mode(VoiceAllocationMode::Oldest);
        // Initialize sample rates for all envelopes
        for voice in allocator.voices_mut() {
            voice.envelope.set_sample_rate(sample_rate);
        }
        Self {
            allocator,
            sample_rate,
            mode: OscillatorMode::Sine,
        }
    }

    /// Technical implementation of the set_waveform logic.
    pub fn set_waveform(&mut self, mode: OscillatorMode) {
        self.mode = mode;
        for voice in self.allocator.voices_mut() {
            voice.oscillator.set_mode(mode);
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process_midi(&mut self, msg: &MidiMessage) {
        let mode = self.mode;
        self.allocator.process_midi(msg, |voice, freq| {
            voice.oscillator.set_mode(mode);
            voice.oscillator.set_frequency(freq);
        });
    }

    /// Generate next sample by mixing all active voices via silicon-summing
    pub fn next(&mut self) -> Sample {
        let _sr = self.sample_rate;
        self.allocator.process_mix(|voice| voice.oscillator.next())
    }

    /// Technical implementation of the generate_into logic.
    pub fn generate_into(&mut self, buffer: &mut [Sample]) {
        for i in 0..buffer.len() {
            buffer[i] = self.next();
        }
    }
}

impl Default for PolyphonicSynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
