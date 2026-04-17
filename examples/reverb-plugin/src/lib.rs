//! Professional reverb plugin with algorithmic processing.
//!
//! Features:
//! - Freeverb-style algorithmic reverb
//! - Room size control
//! - Damping/absorption control
//! - Stereo width control
//! - Wet/dry balance
//! - Real-time safe processing

use smoothie_core::prelude::*;
use smoothie_effects::{ReverbProcessor, EffectProcessor};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Professional reverb plugin.
pub struct ReverbPlugin {
    reverb_l: ReverbProcessor,
    reverb_r: ReverbProcessor,

    // Parameters
    room_size: Arc<FloatParam>,
    damping: Arc<FloatParam>,
    width: Arc<FloatParam>,
    wet_level: Arc<FloatParam>,
    dry_level: Arc<FloatParam>,
}

impl Default for ReverbPlugin {
    fn default() -> Self {
        Self {
            reverb_l: ReverbProcessor::default(),
            reverb_r: ReverbProcessor::default(),
            room_size: Arc::new(FloatParam::new("Room Size", 0.5, 0.0, 1.0)),
            damping: Arc::new(FloatParam::new("Damping", 0.5, 0.0, 1.0)),
            width: Arc::new(FloatParam::new("Width", 1.0, 0.0, 1.0)),
            wet_level: Arc::new(FloatParam::new("Wet", 0.3, 0.0, 1.0)),
            dry_level: Arc::new(FloatParam::new("Dry", 0.7, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for ReverbPlugin {
    const NAME: &'static str = "Reverb";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid(*b"REVB0001");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        &[AudioLayout::stereo()]
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.room_size) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.damping) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.width) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.wet_level) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.dry_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.reverb_l.set_sample_rate(sr);
        self.reverb_r.set_sample_rate(sr);
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let room = self.room_size.value();
        let damp = self.damping.value();
        let width = self.width.value();
        let wet = self.wet_level.value();
        let dry = self.dry_level.value();

        // Update reverb parameters
        self.reverb_l.set_room_size(room);
        self.reverb_l.set_damp(damp);
        self.reverb_l.set_width(width);

        self.reverb_r.set_room_size(room);
        self.reverb_r.set_damp(damp);
        self.reverb_r.set_width(width);

        // Process stereo
        let buffer = ctx.buffer_mut();
        if buffer.channels() >= 2 {
            let ch_l = buffer.channel_mut(0);
            let ch_r = buffer.channel_mut(1);

            for i in 0..ch_l.len() {
                let in_l = ch_l[i];
                let in_r = ch_r[i];

                let out_l = self.reverb_l.process(in_l);
                let out_r = self.reverb_r.process(in_r);

                ch_l[i] = in_l * dry + out_l * wet;
                ch_r[i] = in_r * dry + out_r * wet;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.reverb_l.reset();
        self.reverb_r.reset();
    }

    fn tail_length_samples(&self) -> u32 {
        // Reverb has a long tail (5 seconds)
        (5.0 * 44100.0) as u32
    }
}

smoothie_export!(ReverbPlugin);
