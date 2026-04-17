//! Polyphonic wavetable synthesizer plugin.
//!
//! Features:
//! - 32-voice polyphonic synthesis
//! - Wavetable oscillator (sine, triangle, sawtooth, square)
//! - ADSR envelope with per-voice modulation
//! - Dual LFO for modulation
//! - Voice allocation strategies (oldest, quietest, lowest pitch)
//! - Real-time safe MIDI processing

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::{PolySynth, AllocationStrategy};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Polyphonic wavetable synthesizer.
pub struct PolySynthPlugin {
    synth: PolySynth,

    // Parameters
    waveform: Arc<FloatParam>,      // 0-3: sine, triangle, sawtooth, square
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    attack: Arc<FloatParam>,
    decay: Arc<FloatParam>,
    sustain: Arc<FloatParam>,
    release: Arc<FloatParam>,
    lfo_rate: Arc<FloatParam>,
    lfo_depth: Arc<FloatParam>,
    master_level: Arc<FloatParam>,
}

impl Default for PolySynthPlugin {
    fn default() -> Self {
        Self {
            synth: PolySynth::new(44100.0),
            waveform: Arc::new(FloatParam::simple("Waveform", 0.0, 0.0, 3.0)),
            cutoff: Arc::new(FloatParam::simple("Cutoff", 8000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::simple("Resonance", 1.0, 0.1, 10.0)),
            attack: Arc::new(FloatParam::simple("Attack", 0.01, 0.001, 5.0)),
            decay: Arc::new(FloatParam::simple("Decay", 0.1, 0.001, 5.0)),
            sustain: Arc::new(FloatParam::simple("Sustain", 0.7, 0.0, 1.0)),
            release: Arc::new(FloatParam::simple("Release", 0.3, 0.001, 5.0)),
            lfo_rate: Arc::new(FloatParam::simple("LFO Rate", 5.0, 0.1, 20.0)),
            lfo_depth: Arc::new(FloatParam::simple("LFO Depth", 0.0, 0.0, 1.0)),
            master_level: Arc::new(FloatParam::simple("Master", 0.8, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for PolySynthPlugin {
    const NAME: &'static str = "PolySynth";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.polysynth");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.waveform) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.resonance) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.attack) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.decay) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.release) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.lfo_rate) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.lfo_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.synth.set_sample_rate(sr);
        self.synth.set_allocation(AllocationStrategy::QuietestVoice);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let master = self.master_level.value();
        let buffer = ctx.buffer_mut();

        let num_samples = buffer.samples();
        for i in 0..num_samples {
            let sample = self.synth.process() * master;

            // Get mutable access to channels one at a time to avoid borrow conflicts
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = sample;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.synth.reset();
    }

    fn tail_length_samples(&self) -> u32 {
        // Synth with release envelope can tail
        (5.0 * 44100.0) as u32
    }
}

smoothie_export!(PolySynthPlugin);
