//! Modulated delay with LFO for chorus, flanger, and tremolo effects.
//!
//! Features:
//! - Variable delay time with LFO modulation
//! - Feedback control for regenerative effects
//! - Multi-tap delay option
//! - Real-time modulation control

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_effects::Lfo;
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Modulated delay processor with LFO time modulation.
pub struct ModulatedDelayPlugin {
    delay_line: Vec<f32>,
    read_pos: usize,
    write_pos: usize,
    modulation_lfo: Lfo,

    // Parameters
    delay_time: Arc<FloatParam>,  // Base delay in ms
    feedback: Arc<FloatParam>,    // Feedback amount (0-1)
    mod_depth: Arc<FloatParam>,   // LFO modulation depth (0-1)
    mod_rate: Arc<FloatParam>,    // LFO rate in Hz
    dry_wet: Arc<FloatParam>,     // Mix between dry and wet signal

    sample_rate: f32,
}

impl Default for ModulatedDelayPlugin {
    fn default() -> Self {
        Self {
            delay_line: vec![0.0; 88200],  // 2 seconds @ 44.1kHz
            read_pos: 0,
            write_pos: 0,
            modulation_lfo: Lfo::new(5.0, smoothie_effects::LfoShape::Sine, 44100.0),

            delay_time: Arc::new(FloatParam::simple("delay_time", 250.0, 10.0, 2000.0)),
            feedback: Arc::new(FloatParam::simple("feedback", 0.4, 0.0, 0.99)),
            mod_depth: Arc::new(FloatParam::simple("mod_depth", 0.5, 0.0, 1.0)),
            mod_rate: Arc::new(FloatParam::simple("mod_rate", 5.0, 0.1, 20.0)),
            dry_wet: Arc::new(FloatParam::simple("dry_wet", 0.5, 0.0, 1.0)),

            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for ModulatedDelayPlugin {
    const NAME: &'static str = "Modulated Delay";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.mod-delay");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.delay_time) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.feedback) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod_rate) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.dry_wet) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        self.delay_line = vec![0.0; (self.sample_rate * 2.0) as usize]; // 2 sec buffer
        self.modulation_lfo = Lfo::new(5.0, smoothie_effects::LfoShape::Sine, self.sample_rate);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let delay_ms = self.delay_time.value();
        let feedback = self.feedback.value().min(0.99);
        let mod_depth = self.mod_depth.value();
        let mod_rate = self.mod_rate.value();
        let dry_wet = self.dry_wet.value();

        // Update LFO
        self.modulation_lfo.set_frequency(mod_rate);

        // Convert delay time to samples
        let delay_samples = (delay_ms * self.sample_rate / 1000.0) as usize;
        let delay_len = self.delay_line.len();

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            // Get input from first channel
            let input = buffer.channel(0)[i];

            // Get LFO modulation value (-1 to 1)
            let lfo_value = self.modulation_lfo.next_sample();

            // Modulate delay time with LFO
            let modulated_delay = ((delay_samples as f32 * (1.0 + mod_depth * lfo_value)) as usize).min(delay_len - 1).max(1);

            // Calculate read position
            let read_pos = (self.write_pos + delay_len - modulated_delay) % delay_len;

            // Read delayed sample
            let delayed = self.delay_line[read_pos];

            // Write new sample with feedback
            let to_write = input + delayed * feedback;
            self.delay_line[self.write_pos] = to_write;

            // Mix dry and wet
            let output = input * (1.0 - dry_wet) + delayed * dry_wet;

            // Write to both channels
            buffer.channel_mut(0)[i] = output;
            if buffer.channels() >= 2 {
                buffer.channel_mut(1)[i] = output;
            }

            // Advance write pointer
            self.write_pos = (self.write_pos + 1) % delay_len;
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.delay_line.fill(0.0);
        self.write_pos = 0;
        self.read_pos = 0;
    }

    fn tail_length_samples(&self) -> u32 {
        (2.0 * self.sample_rate) as u32
    }
}

smoothie_export!(ModulatedDelayPlugin);
