/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd36f676a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/wavetable_synth.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::voice::{VoiceAllocationMode, VoiceAllocator};
use smoothie_core::primitives::Sample;
use smoothie_dsp::oscillators::WavetableOscillator;
use smoothie_midi::MidiMessage;

/// Real-time safe wavetable synthesizer with morphing and polyphony
#[repr(align(64))]
/// Technical implementation of the WavetableSynth structure.
pub struct WavetableSynth {
    pub allocator: VoiceAllocator<WavetableOscillator, 16>, // 16 voice wavetable
    sample_rate: f32,
}

impl WavetableSynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut allocator = VoiceAllocator::new();
        allocator.set_mode(VoiceAllocationMode::Oldest);
        for voice in allocator.voices_mut() {
            voice.envelope.set_sample_rate(sample_rate);
        }
        Self {
            allocator,
            sample_rate: 44100.0,
        }
    }

    #[inline(always)]
    pub fn process_midi(&mut self, msg: &MidiMessage) {
        self.allocator.process_midi(msg, |voice, freq| {
            voice.oscillator.set_frequency(freq);
            voice.oscillator.reset_phase();
        });
    }

    /// Technical implementation of the next logic.
    pub fn process(&mut self) -> Sample {
        self.allocator.process_mix(|voice| voice.oscillator.process())
    }

    /// Technical implementation of the generate_into logic.
    pub fn generate_into(&mut self, buffer: &mut [Sample]) {
        for i in 0..buffer.len() {
            buffer[i] = self.process();
        }
    }
}

impl Default for WavetableSynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
