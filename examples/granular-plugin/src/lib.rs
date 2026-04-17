//! Granular synthesis plugin for time-stretching and effects.
//!
//! Features:
//! - 32 concurrent grains with independent pitch and position
//! - Three envelope shapes (Hann, Gaussian, Triangle)
//! - Grain density control (grains per second)
//! - Pitch shifting and time-stretching
//! - Circular buffer for continuous granulation
//! - Real-time safe processing

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::{GranularEngine, GrainEnvelope};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Granular synthesis processor.
pub struct GranularPlugin {
    engine: GranularEngine,

    // Parameters
    density: Arc<FloatParam>,          // Grains per second
    grain_duration: Arc<FloatParam>,   // Milliseconds
    pitch: Arc<FloatParam>,            // Playback pitch (0.5 = half speed, 2.0 = double)
    envelope_shape: Arc<FloatParam>,   // 0-2: hann, gaussian, triangle
    pitch_randomness: Arc<FloatParam>, // Random pitch variation
    position_randomness: Arc<FloatParam>, // Random start position
    master_level: Arc<FloatParam>,
}

impl Default for GranularPlugin {
    fn default() -> Self {
        Self {
            engine: GranularEngine::new(44100.0),
            density: Arc::new(FloatParam::simple("Density", 10.0, 1.0, 100.0)),
            grain_duration: Arc::new(FloatParam::simple("Grain Duration", 100.0, 10.0, 500.0)),
            pitch: Arc::new(FloatParam::simple("Pitch", 1.0, 0.25, 4.0)),
            envelope_shape: Arc::new(FloatParam::simple("Envelope", 0.0, 0.0, 2.0)),
            pitch_randomness: Arc::new(FloatParam::simple("Pitch Randomness", 0.0, 0.0, 0.5)),
            position_randomness: Arc::new(FloatParam::simple("Position Randomness", 0.0, 0.0, 1.0)),
            master_level: Arc::new(FloatParam::simple("Master", 0.8, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for GranularPlugin {
    const NAME: &'static str = "Granular";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.granular");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.density) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.grain_duration) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.pitch) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.envelope_shape) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.pitch_randomness) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.position_randomness) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.engine.set_sample_rate(sr);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Update grain parameters
        let duration_ms = self.grain_duration.value();
        self.engine.set_grain_duration(duration_ms);
        self.engine.set_grain_density(self.density.value());
        self.engine.set_grain_pitch(self.pitch.value());

        let shape = match self.envelope_shape.value() as u32 {
            1 => GrainEnvelope::Gaussian,
            2 => GrainEnvelope::Triangle,
            _ => GrainEnvelope::Hann,
        };
        self.engine.set_grain_shape(shape);

        let master = self.master_level.value();
        let buffer = ctx.buffer_mut();

        let num_samples = buffer.samples();
        for i in 0..num_samples {
            // Feed input from first channel into granule buffer
            let input = buffer.channel(0)[i];
            self.engine.feed_sample(input);

            let sample = self.engine.process() * master;

            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = sample;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.engine.reset();
    }

    fn tail_length_samples(&self) -> u32 {
        // Granular can have a tail from grain release
        (2.0 * 44100.0) as u32
    }
}

smoothie_export!(GranularPlugin);
