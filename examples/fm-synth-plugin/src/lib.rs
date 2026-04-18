//! Frequency Modulation (FM) synthesis plugin.
//!
//! Features:
//! - 4-operator FM synthesis with Yamaha-style algorithms
//! - Three algorithm types (simple, stack, complex)
//! - Carrier and modulator frequency ratios
//! - Modulation depth and feedback control
//! - Real-time operator envelope control
//! - Feedback path for rich harmonics

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::{FmSynth, FmAlgorithm};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// FM synthesis plugin.
pub struct FmSynthPlugin {
    fm: FmSynth,

    // Parameters
    algorithm: Arc<FloatParam>,        // 0-2: simple, stack, complex
    carrier_freq: Arc<FloatParam>,
    mod1_ratio: Arc<FloatParam>,
    mod2_ratio: Arc<FloatParam>,
    mod3_ratio: Arc<FloatParam>,
    mod1_depth: Arc<FloatParam>,
    mod2_depth: Arc<FloatParam>,
    mod3_depth: Arc<FloatParam>,
    feedback: Arc<FloatParam>,
    attack: Arc<FloatParam>,
    decay: Arc<FloatParam>,
    sustain: Arc<FloatParam>,
    release: Arc<FloatParam>,
    master_level: Arc<FloatParam>,
}

impl Default for FmSynthPlugin {
    fn default() -> Self {
        Self {
            fm: FmSynth::new(44100.0),
            algorithm: Arc::new(FloatParam::simple("Algorithm", 0.0, 0.0, 2.0)),
            carrier_freq: Arc::new(FloatParam::simple("Carrier Freq", 440.0, 20.0, 2000.0)),
            mod1_ratio: Arc::new(FloatParam::simple("Mod1 Ratio", 1.0, 0.5, 8.0)),
            mod2_ratio: Arc::new(FloatParam::simple("Mod2 Ratio", 2.0, 0.5, 8.0)),
            mod3_ratio: Arc::new(FloatParam::simple("Mod3 Ratio", 3.0, 0.5, 8.0)),
            mod1_depth: Arc::new(FloatParam::simple("Mod1 Depth", 100.0, 0.0, 1000.0)),
            mod2_depth: Arc::new(FloatParam::simple("Mod2 Depth", 50.0, 0.0, 1000.0)),
            mod3_depth: Arc::new(FloatParam::simple("Mod3 Depth", 25.0, 0.0, 1000.0)),
            feedback: Arc::new(FloatParam::simple("Feedback", 0.0, 0.0, 1.0)),
            attack: Arc::new(FloatParam::simple("Attack", 0.01, 0.001, 1.0)),
            decay: Arc::new(FloatParam::simple("Decay", 0.1, 0.001, 2.0)),
            sustain: Arc::new(FloatParam::simple("Sustain", 0.8, 0.0, 1.0)),
            release: Arc::new(FloatParam::simple("Release", 0.2, 0.001, 2.0)),
            master_level: Arc::new(FloatParam::simple("Master", 0.8, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for FmSynthPlugin {
    const NAME: &'static str = "FM Synth";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.fmsynth");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.algorithm) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.carrier_freq) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod1_ratio) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod2_ratio) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod3_ratio) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod1_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod2_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod3_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.feedback) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.attack) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.decay) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.release) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.fm.set_sample_rate(sr);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let algo = self.algorithm.value() as u32;
        let algorithm = match algo {
            1 => FmAlgorithm::Stack,
            2 => FmAlgorithm::Complex,
            _ => FmAlgorithm::Simple,
        };

        let carrier = self.carrier_freq.value();
        self.fm.set_algorithm(algorithm);
        self.fm.set_op_ratio(0, 1.0, carrier);  // Carrier
        self.fm.set_op_ratio(1, self.mod1_ratio.value(), carrier);
        self.fm.set_op_ratio(2, self.mod2_ratio.value(), carrier);
        self.fm.set_op_ratio(3, self.mod3_ratio.value(), carrier);

        // Set levels from depth parameters
        let max_level = 1.0;
        self.fm.set_op_level(1, self.mod1_depth.value().min(max_level));
        self.fm.set_op_level(2, self.mod2_depth.value().min(max_level));
        self.fm.set_op_level(3, self.mod3_depth.value().min(max_level));
        self.fm.set_feedback(self.feedback.value());

        let master = self.master_level.value();
        let buffer = ctx.buffer_mut();

        let num_samples = buffer.samples();
        for i in 0..num_samples {
            let sample = self.fm.process() * master;

            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = sample;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.fm.reset();
    }

    fn tail_length_samples(&self) -> u32 {
        (2.0 * 44100.0) as u32
    }
}

smoothie_export!(FmSynthPlugin);


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
