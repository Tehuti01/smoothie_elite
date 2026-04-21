/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5e59bc9a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/examples/gain/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::{
    SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus,
    AudioProcessor,
};

// ═══════════════════════════════════════════════════════════════
// Plugin Definition
// ═══════════════════════════════════════════════════════════════

/// Technical implementation of the GainPlugin structure.
pub struct GainPlugin {
    /// Current gain in linear amplitude (1.0 = unity, 0.0 = silence).
    gain: f32,
    /// Target gain for smooth transitions.
    target_gain: f32,
    /// Smoothing coefficient (derived from sample rate).
    smooth_coeff: f32,
    /// Current sample rate.
    sample_rate: f32,
}

impl SmoothiePlugin for GainPlugin {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "Smoothie Gain",
            vendor: "Smoothie Audio",
            version: "1.0.0",
            category: PluginCategory::Utility,
            input_channels: 2,
            output_channels: 2,
        }
    }

    fn new(sample_rate: f32) -> Self {
        Self {
            gain: 1.0,
            target_gain: 1.0,
            // Smoothing: ~5ms time constant
            smooth_coeff: 1.0 - (-1.0 / (0.005 * sample_rate)).exp_approx_local(),
            sample_rate,
        }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        for channel in buffer.iter_mut() {
            for sample in channel.iter_mut() {
                // Smooth parameter changes to avoid clicks
                self.gain += self.smooth_coeff * (self.target_gain - self.gain);
                *sample *= self.gain;
            }
        }
        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.smooth_coeff = 1.0 - (-1.0 / (0.005 * sr)).exp_approx_local();
    }

    fn reset(&mut self) {
        self.gain = self.target_gain;
    }

    fn param_count(&self) -> usize { 1 }

    fn get_param(&self, index: usize) -> f32 {
        match index {
            0 => self.target_gain,
            _ => 0.0,
        }
    }

    fn set_param(&mut self, index: usize, value: f32) {
        match index {
            0 => self.target_gain = value.clamp(0.0, 2.0),
            _ => {}
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            0 => "Gain",
            _ => "",
        }
    }
}

/// Local exp approximation for no_std.
trait ExpApproxLocal {
    fn exp_approx_local(self) -> f32;
}

impl ExpApproxLocal for f32 {
    fn exp_approx_local(self) -> f32 {
        smoothie_core::math::exp_approx(self)
    }
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unity_gain_passthrough() {
        let mut plugin = GainPlugin::new(44100.0);
        let mut left = [0.5f32; 64];
        let mut right = [0.3f32; 64];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        let status = plugin.process(&mut channels);
        assert_eq!(status, ProcessStatus::Ok);
        // With unity gain, output should approximately equal input
        assert!((left[63] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_zero_gain_silence() {
        let mut plugin = GainPlugin::new(44100.0);
        plugin.set_param(0, 0.0);
        let mut left = [1.0f32; 512];
        let mut right = [1.0f32; 512];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        plugin.process(&mut channels);
        // After enough samples, gain should have smoothed to near zero
        assert!(left[511].abs() < 0.01);
    }

    #[test]
    fn test_param_names() {
        let plugin = GainPlugin::new(44100.0);
        assert_eq!(plugin.get_param_name(0), "Gain");
        assert_eq!(plugin.param_count(), 1);
    }
}
