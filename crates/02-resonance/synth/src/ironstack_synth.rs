/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x76a083b8 | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/synth/src/ironstack_synth.rs                  │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: High-performance Polyphonic IronStack Synthesizer.          │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Hybrid Voice/Global architecture for WDF modeling.      │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::voice::{VoiceAllocationMode, VoiceAllocator};
use seraphic_multiverse::pitch::auto_pitch_quantizer::PitchQuantizer;
use smoothie_core::primitives::Sample;
use smoothie_dsp::oscillators::{Oscillator, OscillatorMode};
use smoothie_ironstack::IronStackEngine;
use smoothie_midi::MidiMessage;

/// Optimized polyphonic synthesizer leveraging the IRONSTACK-100 modeling hub.
/// uses a Hybrid Voice/Global architecture where oscillators are polyphonic
/// and the high-density WDF stages are applied globally.
pub struct IronStackPolySynth {
    /// Polyphonic voice allocator (16-voice density for industrial stabilization)
    pub allocator: VoiceAllocator<Oscillator, 16>,
    /// Global IronStack high-performance DSP engine
    pub engine: IronStackEngine,
    /// Cognitive pitch quantizer for scale-snapping
    pub quantizer: PitchQuantizer,
    sample_rate: f32,
    mode: OscillatorMode,
}

impl IronStackPolySynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut allocator: VoiceAllocator<Oscillator, 16> = VoiceAllocator::new();
        allocator.set_mode(VoiceAllocationMode::Oldest);

        // Synchronize component sample rates
        for voice in allocator.voices_mut() {
            voice.envelope.set_sample_rate(sample_rate);
            voice.oscillator.sample_rate = sample_rate;
        }

        Self {
            allocator,
            engine: IronStackEngine::new(sample_rate),
            quantizer: PitchQuantizer::new(),
            sample_rate,
            mode: OscillatorMode::Sawtooth,
        }
    }

    /// Technical implementation of the set_waveform logic.
    pub fn set_waveform(&mut self, mode: OscillatorMode) {
        self.mode = mode;
        for voice in self.allocator.voices_mut() {
            voice.oscillator.set_mode(mode);
        }
    }

    /// Primary real-time MIDI processing logic.
    pub fn process_midi(&mut self, msg: &MidiMessage) {
        // Phase XII: Cognitive Pitch Synchronization
        if let Some(snap) = self.engine.params.get_value("Pitch Snap") {
            self.quantizer.intensity = snap;
        }
        if let Some(mask) = self.engine.params.get_value("Scale Mask") {
            self.quantizer.set_scale(0, mask as u16);
        }

        let mode = self.mode;
        let sr = self.sample_rate;
        let quantizer = &mut self.quantizer;

        self.allocator.process_midi(msg, |voice, freq| {
            let q_freq = quantizer.quantize(freq);
            voice.oscillator.set_mode(mode);
            voice.oscillator.set_frequency(q_freq);
            voice.oscillator.sample_rate = sr;
        });
    }

    /// Generate the next sample by mixing voices and passing through the IronStack Hub.
    #[inline(always)]
    pub fn next(&mut self) -> Sample {
        // 1. Silicon-summing of all active polyphonic voices
        let mixed = self.allocator.process_mix(|voice| voice.oscillator.next());

        // 2. High-fidelity global processing via IRONSTACK-100 stages
        self.engine.process(mixed)
    }

    /// Technical implementation of the generate_into logic for buffer processing.
    pub fn generate_into(&mut self, buffer: &mut [Sample]) {
        for sample in buffer.iter_mut() {
            *sample = self.next();
        }
    }

    /// Technical implementation of the reset logic.
    pub fn reset(&mut self) {
        self.allocator.reset();
        // Resetting the engine would clear convolution/WDF states if needed
    }
}

impl Default for IronStackPolySynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
