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
use smoothie_eq::{BandType, ParametricEq, EqBandConfig};

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
        let mut eq_l = ParametricEq::new(sample_rate as f64);
        let mut eq_r = ParametricEq::new(sample_rate as f64);

        // Band 0: Low Shelf @ 100Hz, 0dB, slope=0.7
        eq_l.set_band(0, EqBandConfig {
            band_type: BandType::LowShelf { freq_hz: 100.0, gain_db: 0.0, slope: 0.7 },
            enabled: true,
        });
        eq_r.set_band(0, EqBandConfig {
            band_type: BandType::LowShelf { freq_hz: 100.0, gain_db: 0.0, slope: 0.7 },
            enabled: true,
        });

        // Band 1: Peaking @ 500Hz, 0dB, Q=1.0
        eq_l.set_band(1, EqBandConfig {
            band_type: BandType::Peaking { freq_hz: 500.0, gain_db: 0.0, q: 1.0 },
            enabled: true,
        });
        eq_r.set_band(1, EqBandConfig {
            band_type: BandType::Peaking { freq_hz: 500.0, gain_db: 0.0, q: 1.0 },
            enabled: true,
        });

        Self { eq_l, eq_r, sample_rate }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        if buffer.len() < 2 { return ProcessStatus::Error; }

        let (left, right) = buffer.split_at_mut(1);
        self.eq_l.process_block(left[0], right[0]);
        
        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.eq_l.set_sample_rate(sr as f64);
        self.eq_r.set_sample_rate(sr as f64);
    }

    fn reset(&mut self) {
        self.eq_l.reset();
        self.eq_r.reset();
    }

    fn param_count(&self) -> usize { 12 } // 3 params per band × 4 bands

    fn get_param(&self, _index: usize) -> f32 { 0.0 } // Simplified

    fn set_param(&mut self, index: usize, value: f32) {
        let band_idx = index / 3;
        let param_type = index % 3;

        if band_idx >= 32 { return; }

        let mut config = *self.eq_l.band(band_idx);
        match config.band_type {
            BandType::Peaking { mut freq_hz, mut gain_db, mut q } => {
                match param_type {
                    0 => freq_hz = value as f64,
                    1 => gain_db = value as f64,
                    2 => q = value as f64,
                    _ => {}
                }
                config.band_type = BandType::Peaking { freq_hz, gain_db, q };
            }
            BandType::LowShelf { mut freq_hz, mut gain_db, mut slope } => {
                match param_type {
                    0 => freq_hz = value as f64,
                    1 => gain_db = value as f64,
                    2 => slope = value as f64,
                    _ => {}
                }
                config.band_type = BandType::LowShelf { freq_hz, gain_db, slope };
            }
            // ... other types
            _ => {}
        }
        self.eq_l.set_band(band_idx, config);
        self.eq_r.set_band(band_idx, config);
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            0 => "Low Shelf Freq", 1 => "Low Shelf Gain", 2 => "Low Shelf Slope",
            3 => "Mid-Low Freq", 4 => "Mid-Low Gain", 5 => "Mid-Low Q",
            6 => "Mid-High Freq", 7 => "Mid-High Gain", 8 => "Mid-High Q",
            9 => "High Shelf Freq", 10 => "High Shelf Gain", 11 => "High Shelf Slope",
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
