//! A simple tone generator synthesizer.
//!
//! Demonstrates:
//! - Band-limited oscillators with anti-aliasing
//! - Parameter automation
//! - Real-time safe processing
//! - Low CPU overhead

use smoothie_core::prelude::*;
use smoothie_dsp::oscillator::{Oscillator, WaveShape};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Simple tone generator synthesizer.
pub struct SimpleSynth {
    osc_l: Oscillator,
    osc_r: Oscillator,

    // Parameters
    frequency: Arc<FloatParam>,
    level: Arc<FloatParam>,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        Self {
            osc_l: Oscillator::new(WaveShape::Sine, 44100.0),
            osc_r: Oscillator::new(WaveShape::Sine, 44100.0),
            frequency: Arc::new(FloatParam::new("Frequency", 440.0, 20.0, 20000.0)),
            level: Arc::new(FloatParam::new("Level", 0.1, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for SimpleSynth {
    const NAME: &'static str = "Simple Synth";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid(*b"SSYN0001");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        &[AudioLayout::stereo()]
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.frequency) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, _ctx: &mut InitContext) -> bool {
        self.osc_l.set_frequency(440.0);
        self.osc_r.set_frequency(440.0);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let frequency = self.frequency.value();
        let level = self.level.value();

        // Update oscillator frequency (real-time safe)
        self.osc_l.set_frequency(frequency);
        self.osc_r.set_frequency(frequency);

        // Process audio buffer
        let buffer = ctx.buffer_mut();

        if buffer.channels() >= 2 {
            // Stereo processing
            let ch_l = buffer.channel_mut(0);
            for sample in ch_l.iter_mut() {
                *sample = self.osc_l.next_sample() * level;
            }

            let ch_r = buffer.channel_mut(1);
            for sample in ch_r.iter_mut() {
                *sample = self.osc_r.next_sample() * level;
            }
        } else if buffer.channels() == 1 {
            // Mono processing
            let ch = buffer.channel_mut(0);
            for sample in ch.iter_mut() {
                *sample = self.osc_l.next_sample() * level;
            }
        }

        ProcessStatus::Normal
    }
}

smoothie_export!(SimpleSynth);
