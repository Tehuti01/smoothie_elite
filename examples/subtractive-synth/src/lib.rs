//! Classic subtractive synthesizer demonstrating oscillator→filter→envelope architecture.
//!
//! Features:
//! - Polyphonic voice synthesis
//! - Multi-mode filter (lowpass, highpass, bandpass)
//! - Dynamic filter cutoff with envelope modulation
//! - Full ADSR envelope control
//! - Waveform selection (sine, triangle, sawtooth, square)
//! - Real-time safe processing

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::PolySynth;
use smoothie_effects::{BiquadFilter, FilterType, Envelope};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Classic subtractive synthesizer: OSC → FILTER → ENVELOPE.
pub struct SubtractiveSynthPlugin {
    synth: PolySynth,

    // Filter state
    filter: BiquadFilter,
    envelope: Envelope,

    // Oscillator parameters
    waveform: Arc<FloatParam>,

    // Filter parameters
    filter_cutoff: Arc<FloatParam>,
    filter_resonance: Arc<FloatParam>,
    filter_type: Arc<FloatParam>,  // 0=LP, 1=HP, 2=BP
    filter_env_amount: Arc<FloatParam>,

    // Envelope parameters (ADSR)
    env_attack: Arc<FloatParam>,
    env_decay: Arc<FloatParam>,
    env_sustain: Arc<FloatParam>,
    env_release: Arc<FloatParam>,

    // Mix
    master_level: Arc<FloatParam>,
    sample_rate: f32,
}

impl Default for SubtractiveSynthPlugin {
    fn default() -> Self {
        Self {
            synth: PolySynth::new(44100.0),
            filter: BiquadFilter::design(FilterType::LowPass, 5000.0, 44100.0, 1.0, 0.0),
            envelope: Envelope::new(0.01, 0.1, 0.7, 0.5, 44100.0),

            waveform: Arc::new(FloatParam::simple("waveform", 0.0, 0.0, 3.0)),
            filter_cutoff: Arc::new(FloatParam::simple("cutoff", 5000.0, 20.0, 20000.0)),
            filter_resonance: Arc::new(FloatParam::simple("resonance", 1.0, 0.1, 10.0)),
            filter_type: Arc::new(FloatParam::simple("filter_type", 0.0, 0.0, 2.0)),
            filter_env_amount: Arc::new(FloatParam::simple("filter_env", 0.5, 0.0, 1.0)),

            env_attack: Arc::new(FloatParam::simple("attack", 0.01, 0.001, 2.0)),
            env_decay: Arc::new(FloatParam::simple("decay", 0.1, 0.001, 2.0)),
            env_sustain: Arc::new(FloatParam::simple("sustain", 0.7, 0.0, 1.0)),
            env_release: Arc::new(FloatParam::simple("release", 0.5, 0.001, 5.0)),

            master_level: Arc::new(FloatParam::simple("master", 0.8, 0.0, 1.0)),
            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for SubtractiveSynthPlugin {
    const NAME: &'static str = "Subtractive";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.subtractive");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.waveform) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_resonance) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_type) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_env_amount) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_attack) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_decay) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_release) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        self.synth = PolySynth::new(self.sample_rate);
        self.envelope = Envelope::new(
            self.env_attack.value(),
            self.env_decay.value(),
            self.env_sustain.value(),
            self.env_release.value(),
            self.sample_rate,
        );
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Get parameter values
        let waveform = (self.waveform.value() as usize) % 4;
        let cutoff = self.filter_cutoff.value();
        let resonance = self.filter_resonance.value();
        let filter_type_idx = (self.filter_type.value() as usize) % 3;
        let filter_env_amt = self.filter_env_amount.value();

        let attack = self.env_attack.value();
        let decay = self.env_decay.value();
        let sustain = self.env_sustain.value();
        let release = self.env_release.value();

        let master = self.master_level.value();

        // Select filter type
        let filter_type = match filter_type_idx {
            0 => FilterType::LowPass,
            1 => FilterType::HighPass,
            2 => FilterType::BandPass,
            _ => FilterType::LowPass,
        };

        // Update envelope parameters
        self.envelope.set_attack(attack);
        self.envelope.set_decay(decay);
        self.envelope.set_sustain(sustain);
        self.envelope.set_release(release);

        // Set synthesizer waveform
        self.synth.set_waveform(waveform);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            // Get oscillator output from synth
            let osc_out = self.synth.process();

            // Get next envelope sample
            let env_value = self.envelope.next_sample();

            // Modulate filter cutoff with envelope
            let modulated_cutoff = cutoff * (1.0 + filter_env_amt * (env_value - 0.5) * 2.0).max(0.1);

            // Recreate filter with modulated parameters
            self.filter = BiquadFilter::design(filter_type, modulated_cutoff, self.sample_rate, resonance, 0.0);

            // Apply filter and envelope to oscillator
            let filtered = self.filter.process(osc_out);
            let output = filtered * env_value * master;

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.synth = PolySynth::new(self.sample_rate);
    }

    fn tail_length_samples(&self) -> u32 {
        // Tail for release envelope (max 5 seconds)
        (5.0 * self.sample_rate) as u32
    }
}

smoothie_export!(SubtractiveSynthPlugin);
