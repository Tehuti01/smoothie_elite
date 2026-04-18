//! 3-band parametric equalizer with peaking and shelving filters.
//!
//! Features:
//! - Low shelf filter (bass control)
//! - Mid peaking filter (presence control)
//! - High shelf filter (treble control)
//! - Independent frequency and Q control per band
//! - Gain control for each band

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_effects::{BiquadFilter, FilterType};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// 3-band parametric EQ processor.
pub struct ParametricEqPlugin {
    // Three filter stages
    low_filter: BiquadFilter,
    mid_filter: BiquadFilter,
    high_filter: BiquadFilter,

    // Low band controls
    low_freq: Arc<FloatParam>,
    low_gain: Arc<FloatParam>,
    low_q: Arc<FloatParam>,

    // Mid band controls
    mid_freq: Arc<FloatParam>,
    mid_gain: Arc<FloatParam>,
    mid_q: Arc<FloatParam>,

    // High band controls
    high_freq: Arc<FloatParam>,
    high_gain: Arc<FloatParam>,
    high_q: Arc<FloatParam>,

    // Output
    output_gain: Arc<FloatParam>,

    sample_rate: f32,
}

impl Default for ParametricEqPlugin {
    fn default() -> Self {
        Self {
            low_filter: BiquadFilter::design(FilterType::LowPass, 200.0, 44100.0, 1.0, 0.0),
            mid_filter: BiquadFilter::design(FilterType::BandPass, 1000.0, 44100.0, 1.0, 0.0),
            high_filter: BiquadFilter::design(FilterType::HighPass, 5000.0, 44100.0, 1.0, 0.0),

            low_freq: Arc::new(FloatParam::simple("low_freq", 200.0, 20.0, 500.0)),
            low_gain: Arc::new(FloatParam::simple("low_gain", 0.0, -12.0, 12.0)),
            low_q: Arc::new(FloatParam::simple("low_q", 0.7, 0.1, 2.0)),

            mid_freq: Arc::new(FloatParam::simple("mid_freq", 1000.0, 200.0, 5000.0)),
            mid_gain: Arc::new(FloatParam::simple("mid_gain", 0.0, -12.0, 12.0)),
            mid_q: Arc::new(FloatParam::simple("mid_q", 1.0, 0.1, 5.0)),

            high_freq: Arc::new(FloatParam::simple("high_freq", 5000.0, 2000.0, 20000.0)),
            high_gain: Arc::new(FloatParam::simple("high_gain", 0.0, -12.0, 12.0)),
            high_q: Arc::new(FloatParam::simple("high_q", 0.7, 0.1, 2.0)),

            output_gain: Arc::new(FloatParam::simple("output", 0.0, -12.0, 12.0)),

            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for ParametricEqPlugin {
    const NAME: &'static str = "Parametric EQ";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.eq");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.low_freq) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.low_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.low_q) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mid_freq) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mid_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mid_q) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.high_freq) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.high_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.high_q) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.output_gain) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Get all parameter values
        let low_f = self.low_freq.value();
        let low_g = self.low_gain.value();
        let low_q = self.low_q.value();

        let mid_f = self.mid_freq.value();
        let mid_g = self.mid_gain.value();
        let mid_q = self.mid_q.value();

        let high_f = self.high_freq.value();
        let high_g = self.high_gain.value();
        let high_q = self.high_q.value();

        let out_gain = self.output_gain.value();

        // Update filters with current parameters
        self.low_filter = BiquadFilter::design(FilterType::LowPass, low_f, self.sample_rate, low_q, low_g);
        self.mid_filter = BiquadFilter::design(FilterType::BandPass, mid_f, self.sample_rate, mid_q, mid_g);
        self.high_filter = BiquadFilter::design(FilterType::HighPass, high_f, self.sample_rate, high_q, high_g);

        // Output gain linear
        let output_linear = 10.0_f32.powf(out_gain / 20.0);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            let input = buffer.channel(0)[i];

            // Apply cascaded filters
            let low_out = self.low_filter.process(input);
            let mid_out = self.mid_filter.process(low_out);
            let high_out = self.high_filter.process(mid_out);

            // Apply output gain
            let output = high_out * output_linear;

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        // Filters are recreated each process cycle
    }

    fn tail_length_samples(&self) -> u32 {
        0
    }
}

smoothie_export!(ParametricEqPlugin);


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
