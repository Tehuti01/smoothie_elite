//! Multi-effect processor demonstrating effect chains.
//!
//! Features:
//! - Distortion → EQ → Delay → Reverb chain
//! - Individual effect bypass/enable
//! - Per-effect parameter control
//! - Real-time safe signal routing

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_effects::{
    EffectProcessor, DistortionProcessor, EqProcessor, DelayProcessor, ReverbProcessor,
    DistortionType,
};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Effects chain plugin: Distortion → EQ → Delay → Reverb.
pub struct EffectsChainPlugin {
    // Effects (in processing order)
    distortion: DistortionProcessor,
    eq: EqProcessor,
    delay: DelayProcessor,
    reverb: ReverbProcessor,

    // Bypass switches
    distortion_on: bool,
    eq_on: bool,
    delay_on: bool,
    reverb_on: bool,

    // Master parameters
    master_level: Arc<FloatParam>,

    // Effect parameters
    dist_drive: Arc<FloatParam>,
    eq_low_gain: Arc<FloatParam>,
    eq_mid_gain: Arc<FloatParam>,
    eq_high_gain: Arc<FloatParam>,
    delay_time: Arc<FloatParam>,
    reverb_room: Arc<FloatParam>,
}

impl Default for EffectsChainPlugin {
    fn default() -> Self {
        Self {
            distortion: DistortionProcessor::new(DistortionType::SoftClip),
            eq: EqProcessor::new(44100.0),
            delay: DelayProcessor::new(44100.0),
            reverb: ReverbProcessor::new(44100.0),
            distortion_on: false,
            eq_on: true,
            delay_on: false,
            reverb_on: false,
            master_level: Arc::new(FloatParam::simple("Master", 1.0, 0.0, 2.0)),
            dist_drive: Arc::new(FloatParam::simple("Dist Drive", 1.0, 0.0, 10.0)),
            eq_low_gain: Arc::new(FloatParam::simple("EQ Low", 0.0, -24.0, 24.0)),
            eq_mid_gain: Arc::new(FloatParam::simple("EQ Mid", 0.0, -24.0, 24.0)),
            eq_high_gain: Arc::new(FloatParam::simple("EQ High", 0.0, -24.0, 24.0)),
            delay_time: Arc::new(FloatParam::simple("Delay MS", 500.0, 1.0, 4000.0)),
            reverb_room: Arc::new(FloatParam::simple("Reverb Room", 0.5, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for EffectsChainPlugin {
    const NAME: &'static str = "Effects Chain";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.effectschain");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.dist_drive) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.eq_low_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.eq_mid_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.eq_high_gain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.delay_time) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.reverb_room) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.distortion.set_sample_rate(sr);
        self.eq.set_sample_rate(sr);
        self.delay.set_sample_rate(sr);
        self.reverb.set_sample_rate(sr);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Update effect parameters
        self.distortion.set_drive(self.dist_drive.value());
        self.eq.set_low(200.0, self.eq_low_gain.value());
        self.eq.set_mid(1000.0, self.eq_mid_gain.value(), 1.0);
        self.eq.set_high(5000.0, self.eq_high_gain.value());
        self.delay.set_delay(self.delay_time.value());
        self.reverb.set_room_size(self.reverb_room.value());

        let master = self.master_level.value();

        // Process stereo
        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            let mut sig_l = buffer.channel(0)[i];
            let mut sig_r = buffer.channel(1)[i];

            // Process left channel through chain
            if self.distortion_on {
                sig_l = self.distortion.process(sig_l);
            }
            if self.eq_on {
                sig_l = self.eq.process(sig_l);
            }
            if self.delay_on {
                sig_l = self.delay.process(sig_l);
            }
            if self.reverb_on {
                sig_l = self.reverb.process(sig_l);
            }

            // Process right channel independently
            if self.distortion_on {
                sig_r = self.distortion.process(sig_r);
            }
            if self.eq_on {
                sig_r = self.eq.process(sig_r);
            }
            if self.delay_on {
                sig_r = self.delay.process(sig_r);
            }
            if self.reverb_on {
                sig_r = self.reverb.process(sig_r);
            }

            buffer.channel_mut(0)[i] = sig_l * master;
            buffer.channel_mut(1)[i] = sig_r * master;
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.distortion.reset();
        self.eq.reset();
        self.delay.reset();
        self.reverb.reset();
    }

    fn tail_length_samples(&self) -> u32 {
        // Reverb tail dominates
        (5.0 * 44100.0) as u32
    }
}

smoothie_export!(EffectsChainPlugin);
