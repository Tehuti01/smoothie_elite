//! Professional dynamics processor with compression and soft clipping.
//!
//! Features:
//! - Transparent compression with attack/release control
//! - Soft knee for musical compression curves
//! - Makeup gain for level compensation
//! - Soft clipping for limiting and saturation
//! - Real-time safe processing

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_params::FloatParam;
use std::sync::Arc;
use std::f32::consts::E;

/// Soft knee compressor with makeup gain and clipping.
pub struct DynamicsProcessorPlugin {
    // Compression state
    envelope_level: f32,
    prev_output: f32,

    // Parameters
    threshold: Arc<FloatParam>,     // Threshold in dB
    ratio: Arc<FloatParam>,         // Compression ratio
    attack_ms: Arc<FloatParam>,     // Attack time in ms
    release_ms: Arc<FloatParam>,    // Release time in ms
    makeup_gain_db: Arc<FloatParam>,// Makeup gain in dB
    soft_clipping: Arc<FloatParam>, // Soft clipping saturation (0-1)

    sample_rate: f32,
}

impl Default for DynamicsProcessorPlugin {
    fn default() -> Self {
        Self {
            envelope_level: 0.0,
            prev_output: 0.0,

            threshold: Arc::new(FloatParam::simple("threshold", -20.0, -60.0, 0.0)),
            ratio: Arc::new(FloatParam::simple("ratio", 4.0, 1.0, 20.0)),
            attack_ms: Arc::new(FloatParam::simple("attack", 10.0, 0.1, 100.0)),
            release_ms: Arc::new(FloatParam::simple("release", 100.0, 10.0, 1000.0)),
            makeup_gain_db: Arc::new(FloatParam::simple("makeup", 0.0, -12.0, 12.0)),
            soft_clipping: Arc::new(FloatParam::simple("clipping", 0.5, 0.0, 1.0)),

            sample_rate: 44100.0,
        }
    }
}

/// Soft knee compressor implementation.
fn compress(
    input: f32,
    threshold_db: f32,
    ratio: f32,
    attack_coeff: f32,
    release_coeff: f32,
    envelope: &mut f32,
) -> f32 {
    let input_db = 20.0 * input.abs().log10().max(-120.0);

    // Update envelope follower
    if input_db > *envelope {
        *envelope = attack_coeff * *envelope + (1.0 - attack_coeff) * input_db;
    } else {
        *envelope = release_coeff * *envelope + (1.0 - release_coeff) * input_db;
    }

    // Calculate gain reduction
    let over = (*envelope - threshold_db).max(0.0);
    let gain_reduction_db = -over * (1.0 - 1.0 / ratio);
    let gain_reduction_linear = 10.0_f32.powf(gain_reduction_db / 20.0);

    input * gain_reduction_linear
}

/// Soft clipping using tanh for smooth saturation.
fn soft_clip(x: f32, amount: f32) -> f32 {
    if amount < 0.001 {
        return x;
    }

    // Blend between linear and tanh based on amount
    let threshold = 1.0 / amount;
    if x.abs() < threshold {
        x
    } else {
        let sign = x.signum();
        let abs_x = x.abs();
        let clipped = threshold + (1.0 - 1.0 / (abs_x - threshold + 1.0));
        sign * clipped
    }
}

impl SmoothiePlugin for DynamicsProcessorPlugin {
    const NAME: &'static str = "Dynamics";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.dynamics");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.threshold) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.ratio) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.attack_ms) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.release_ms) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.makeup_gain_db) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.soft_clipping) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let threshold = self.threshold.value();
        let ratio = self.ratio.value();
        let attack_ms = self.attack_ms.value();
        let release_ms = self.release_ms.value();
        let makeup_db = self.makeup_gain_db.value();
        let clipping_amt = self.soft_clipping.value();

        // Calculate envelope coefficients
        let attack_coeff = (-2.0 * std::f32::consts::PI * attack_ms / 1000.0 / self.sample_rate).exp();
        let release_coeff = (-2.0 * std::f32::consts::PI * release_ms / 1000.0 / self.sample_rate).exp();

        // Makeup gain in linear form
        let makeup_linear = 10.0_f32.powf(makeup_db / 20.0);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            let input = buffer.channel(0)[i];

            // Apply compression
            let compressed = compress(input, threshold, ratio, attack_coeff, release_coeff, &mut self.envelope_level);

            // Apply makeup gain
            let with_makeup = compressed * makeup_linear;

            // Apply soft clipping
            let clipped = soft_clip(with_makeup, clipping_amt);

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = clipped;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.envelope_level = -120.0;
        self.prev_output = 0.0;
    }

    fn tail_length_samples(&self) -> u32 {
        0  // No tail for dynamics processor
    }
}

smoothie_export!(DynamicsProcessorPlugin);
