/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x105189e4 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/examples/compressor/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::{SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus};
use smoothie_effects::Compressor;
use smoothie_params::ParameterSmoother;

/// Technical implementation of the CompressorPlugin structure.
pub struct CompressorPlugin {
    comp_l: Compressor,
    comp_r: Compressor,
    threshold_smoother: ParameterSmoother,
    ratio_smoother: ParameterSmoother,
    makeup_smoother: ParameterSmoother,
    mix_smoother: ParameterSmoother,
    threshold_db: f32,
    ratio: f32,
    makeup_db: f32,
    mix: f32,
    sample_rate: f32,
}

// Parameter indices
const P_THRESHOLD: usize = 0;
const P_RATIO: usize = 1;
const P_ATTACK: usize = 2;
const P_RELEASE: usize = 3;
const P_MAKEUP: usize = 4;
const P_MIX: usize = 5;

impl SmoothiePlugin for CompressorPlugin {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "Smoothie Compressor",
            vendor: "Smoothie Audio",
            version: "1.0.0",
            category: PluginCategory::Effect,
            input_channels: 2,
            output_channels: 2,
        }
    }

    fn new(sample_rate: f32) -> Self {
        Self {
            comp_l: Compressor::default(),
            comp_r: Compressor::default(),
            threshold_smoother: ParameterSmoother::new(-12.0, 5.0, sample_rate),
            ratio_smoother: ParameterSmoother::new(4.0, 5.0, sample_rate),
            makeup_smoother: ParameterSmoother::new(0.0, 5.0, sample_rate),
            mix_smoother: ParameterSmoother::new(1.0, 5.0, sample_rate),
            threshold_db: -12.0,
            ratio: 4.0,
            makeup_db: 0.0,
            mix: 1.0,
            sample_rate,
        }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        if buffer.len() < 2 { return ProcessStatus::Error; }

        let block_len = buffer[0].len();
        for i in 0..block_len {
            let _threshold = self.threshold_smoother.process();
            let _ratio = self.ratio_smoother.process();
            let makeup = self.makeup_smoother.process();
            let mix = self.mix_smoother.process();

            let dry_l = buffer[0][i];
            let dry_r = buffer[1][i];

            let wet_l = self.comp_l.process(dry_l);
            let wet_r = self.comp_r.process(dry_r);

            // Apply makeup gain
            let makeup_linear = db_to_linear(makeup);
            let wet_l = wet_l * makeup_linear;
            let wet_r = wet_r * makeup_linear;

            // Dry/wet mix
            buffer[0][i] = dry_l * (1.0 - mix) + wet_l * mix;
            buffer[1][i] = dry_r * (1.0 - mix) + wet_r * mix;
        }

        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.threshold_smoother.set_sample_rate(sr);
        self.ratio_smoother.set_sample_rate(sr);
        self.makeup_smoother.set_sample_rate(sr);
        self.mix_smoother.set_sample_rate(sr);
    }

    fn reset(&mut self) {
        self.threshold_smoother.snap();
        self.ratio_smoother.snap();
        self.makeup_smoother.snap();
        self.mix_smoother.snap();
    }

    fn param_count(&self) -> usize { 6 }

    fn get_param(&self, index: usize) -> f32 {
        match index {
            P_THRESHOLD => self.threshold_db,
            P_RATIO => self.ratio,
            P_ATTACK => 10.0,
            P_RELEASE => 100.0,
            P_MAKEUP => self.makeup_db,
            P_MIX => self.mix,
            _ => 0.0,
        }
    }

    fn set_param(&mut self, index: usize, value: f32) {
        match index {
            P_THRESHOLD => {
                self.threshold_db = value.clamp(-60.0, 0.0);
                self.threshold_smoother.set_target(self.threshold_db);
            }
            P_RATIO => {
                self.ratio = value.clamp(1.0, 20.0);
                self.ratio_smoother.set_target(self.ratio);
            }
            P_ATTACK => {} // Compressor manages internally
            P_RELEASE => {}
            P_MAKEUP => {
                self.makeup_db = value.clamp(-12.0, 24.0);
                self.makeup_smoother.set_target(self.makeup_db);
            }
            P_MIX => {
                self.mix = value.clamp(0.0, 1.0);
                self.mix_smoother.set_target(self.mix);
            }
            _ => {}
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            P_THRESHOLD => "Threshold (dB)",
            P_RATIO => "Ratio",
            P_ATTACK => "Attack (ms)",
            P_RELEASE => "Release (ms)",
            P_MAKEUP => "Makeup (dB)",
            P_MIX => "Mix",
            _ => "",
        }
    }
}

fn db_to_linear(db: f32) -> f32 {
    smoothie_core::math::db_to_amplitude(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressor_plugin_stereo() {
        let mut plugin = CompressorPlugin::new(44100.0);
        let mut left = [0.5f32; 64];
        let mut right = [0.3f32; 64];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        let status = plugin.process(&mut channels);
        assert_eq!(status, ProcessStatus::Ok);
    }

    #[test]
    fn test_compressor_param_names() {
        let plugin = CompressorPlugin::new(44100.0);
        assert_eq!(plugin.param_count(), 6);
        assert_eq!(plugin.get_param_name(0), "Threshold (dB)");
        assert_eq!(plugin.get_param_name(5), "Mix");
    }
}
