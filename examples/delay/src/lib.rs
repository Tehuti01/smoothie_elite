/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4f569947 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/examples/delay/src/lib.rs                                                         │
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
    audio::RingBuffer,
};

/// Technical implementation of the DelayPlugin structure.
pub struct DelayPlugin {
    buffer_l: RingBuffer,
    buffer_r: RingBuffer,
    delay_samples: usize,
    feedback: f32,
    mix: f32,
    sample_rate: f32,
}

impl SmoothiePlugin for DelayPlugin {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "Smoothie Delay",
            vendor: "Smoothie Audio",
            version: "1.0.0",
            category: PluginCategory::Effect,
            input_channels: 2,
            output_channels: 2,
        }
    }

    fn new(sample_rate: f32) -> Self {
        // Default: 250ms delay at given sample rate
        let delay_samples = (0.250 * sample_rate) as usize;
        Self {
            buffer_l: RingBuffer::new(),
            buffer_r: RingBuffer::new(),
            delay_samples: delay_samples.min(4095),
            feedback: 0.4,
            mix: 0.5,
            sample_rate,
        }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        if buffer.len() < 2 { return ProcessStatus::Error; }

        let block_len = buffer[0].len();

        for i in 0..block_len {
            // Left channel
            let dry_l = buffer[0][i];
            let delayed_l = self.buffer_l.read(self.delay_samples);
            self.buffer_l.write(dry_l + delayed_l * self.feedback);
            buffer[0][i] = dry_l * (1.0 - self.mix) + delayed_l * self.mix;

            // Right channel
            let dry_r = buffer[1][i];
            let delayed_r = self.buffer_r.read(self.delay_samples);
            self.buffer_r.write(dry_r + delayed_r * self.feedback);
            buffer[1][i] = dry_r * (1.0 - self.mix) + delayed_r * self.mix;
        }

        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.delay_samples = ((self.delay_samples as f32 / self.sample_rate) * sr) as usize;
        self.delay_samples = self.delay_samples.min(4095);
    }

    fn reset(&mut self) {
        self.buffer_l.clear();
        self.buffer_r.clear();
    }

    fn param_count(&self) -> usize { 3 }

    fn get_param(&self, index: usize) -> f32 {
        match index {
            0 => self.delay_samples as f32 / self.sample_rate * 1000.0, // ms
            1 => self.feedback,
            2 => self.mix,
            _ => 0.0,
        }
    }

    fn set_param(&mut self, index: usize, value: f32) {
        match index {
            0 => {
                let ms = value.clamp(1.0, 2000.0);
                self.delay_samples = ((ms / 1000.0) * self.sample_rate) as usize;
                self.delay_samples = self.delay_samples.min(4095);
            }
            1 => self.feedback = value.clamp(0.0, 0.95),
            2 => self.mix = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            0 => "Delay Time (ms)",
            1 => "Feedback",
            2 => "Mix",
            _ => "",
        }
    }

    fn tail_length(&self) -> usize {
        // Tail: delay time * number of audible repeats
        if self.feedback > 0.01 {
            self.delay_samples * 8
        } else {
            self.delay_samples
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_passes_dry_signal() {
        let mut plugin = DelayPlugin::new(44100.0);
        plugin.set_param(2, 0.0); // 100% dry
        let mut left = [0.5f32; 64];
        let mut right = [0.3f32; 64];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        plugin.process(&mut channels);
        assert!((left[0] - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_delay_produces_echo() {
        let mut plugin = DelayPlugin::new(44100.0);
        plugin.set_param(0, 10.0); // 10ms delay
        plugin.set_param(2, 1.0);  // 100% wet

        // Feed an impulse
        let mut left = [0.0f32; 512];
        let mut right = [0.0f32; 512];
        left[0] = 1.0;
        right[0] = 1.0;

        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        plugin.process(&mut channels);

        // The delayed copy should appear later in the buffer
        let delay_pos = (10.0 / 1000.0 * 44100.0) as usize;
        if delay_pos < 512 {
            assert!(left[delay_pos].abs() > 0.01);
        }
    }
}
