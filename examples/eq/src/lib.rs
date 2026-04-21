/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc6de1181 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/examples/eq/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::{SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus};
use smoothie_dsp::{EqBand, EqBandType, ParametricEq};

/// Technical implementation of the EqPlugin structure.
pub struct EqPlugin {
    eq_l: ParametricEq,
    eq_r: ParametricEq,
    sample_rate: f32,
}

impl SmoothiePlugin for EqPlugin {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "Smoothie EQ",
            vendor: "Smoothie Audio",
            version: "1.0.0",
            category: PluginCategory::Effect,
            input_channels: 2,
            output_channels: 2,
        }
    }

    fn new(sample_rate: f32) -> Self {
        let mut eq_l = ParametricEq::new();
        let mut eq_r = ParametricEq::new();

        // Band 1: Low Shelf @ 100Hz, 0dB, Q=0.7
        eq_l.add_band(EqBand::new(100.0, 0.0, 0.7, EqBandType::LowShelf, sample_rate));
        eq_r.add_band(EqBand::new(100.0, 0.0, 0.7, EqBandType::LowShelf, sample_rate));

        // Band 2: Peaking @ 500Hz, 0dB, Q=1.0
        eq_l.add_band(EqBand::new(500.0, 0.0, 1.0, EqBandType::Peaking, sample_rate));
        eq_r.add_band(EqBand::new(500.0, 0.0, 1.0, EqBandType::Peaking, sample_rate));

        // Band 3: Peaking @ 2000Hz, 0dB, Q=1.0
        eq_l.add_band(EqBand::new(2000.0, 0.0, 1.0, EqBandType::Peaking, sample_rate));
        eq_r.add_band(EqBand::new(2000.0, 0.0, 1.0, EqBandType::Peaking, sample_rate));

        // Band 4: High Shelf @ 8000Hz, 0dB, Q=0.7
        eq_l.add_band(EqBand::new(8000.0, 0.0, 0.7, EqBandType::HighShelf, sample_rate));
        eq_r.add_band(EqBand::new(8000.0, 0.0, 0.7, EqBandType::HighShelf, sample_rate));

        Self { eq_l, eq_r, sample_rate }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        if buffer.len() < 2 { return ProcessStatus::Error; }

        let block_len = buffer[0].len();
        for i in 0..block_len {
            buffer[0][i] = self.eq_l.process(buffer[0][i]);
            buffer[1][i] = self.eq_r.process(buffer[1][i]);
        }
        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        // Rebuild the EQ for the new sample rate
        *self = Self::new(sr);
    }

    fn reset(&mut self) {
        self.eq_l.reset();
        self.eq_r.reset();
    }

    fn param_count(&self) -> usize { 12 } // 3 params per band × 4 bands

    fn get_param(&self, _index: usize) -> f32 { 0.0 } // Simplified

    fn set_param(&mut self, index: usize, value: f32) {
        let band = index / 3;
        let param_type = index % 3;

        if let (Some(bl), Some(br)) = (self.eq_l.band_mut(band), self.eq_r.band_mut(band)) {
            match param_type {
                0 => { bl.set_frequency(value); br.set_frequency(value); }
                1 => { bl.set_gain(value); br.set_gain(value); }
                2 => { bl.set_q(value); br.set_q(value); }
                _ => {}
            }
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            0 => "Low Shelf Freq", 1 => "Low Shelf Gain", 2 => "Low Shelf Q",
            3 => "Mid-Low Freq", 4 => "Mid-Low Gain", 5 => "Mid-Low Q",
            6 => "Mid-High Freq", 7 => "Mid-High Gain", 8 => "Mid-High Q",
            9 => "High Shelf Freq", 10 => "High Shelf Gain", 11 => "High Shelf Q",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_flat_passthrough() {
        let mut plugin = EqPlugin::new(44100.0);
        let mut left = [0.5f32; 512];
        let mut right = [0.3f32; 512];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        let status = plugin.process(&mut channels);
        assert_eq!(status, ProcessStatus::Ok);
        // At 0dB gain on all bands, output should be ~input after transient
        assert!((left[511] - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_eq_param_count() {
        let plugin = EqPlugin::new(44100.0);
        assert_eq!(plugin.param_count(), 12);
    }
}
