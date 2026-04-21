/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0abc4795 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/grain_synth.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::voice::{VoiceAllocationMode, VoiceAllocator};
use smoothie_core::primitives::Sample;

// A dummy grain oscillator for now until we expand the grain DSP
#[derive(Default)]
#[repr(align(64))]
/// Technical implementation of the GrainOscillator structure.
pub struct GrainOscillator {
    freq: f32,
}
impl GrainOscillator {
    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
    /// Technical implementation of the next logic.
    pub fn next(&mut self, _sr: f32) -> Sample {
        0.0
    }
}

/// Advanced Granular Synthesizer
#[repr(align(64))]
/// Technical implementation of the GrainSynth structure.
pub struct GrainSynth {
    pub allocator: VoiceAllocator<GrainOscillator, 8>,
    sample_rate: f32,
}

impl GrainSynth {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut allocator = VoiceAllocator::new();
        allocator.set_mode(VoiceAllocationMode::RoundRobin);
        for voice in allocator.voices_mut() {
            voice.envelope.set_sample_rate(sample_rate);
        }
        Self {
            allocator,
            sample_rate,
        }
    }

    /// Technical implementation of the next logic.
    pub fn next(&mut self) -> Sample {
        let sr = self.sample_rate;
        self.allocator
            .process_mix(|voice| voice.oscillator.next(sr))
    }
}

impl Default for GrainSynth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
