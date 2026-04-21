/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xffa0b2bc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/examples/synth_basic/src/lib.rs                                                         │
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
    math::{PhaseAccumulator, OnePoleFilter},
};

/// Technical implementation of the BasicSynth structure.
pub struct BasicSynth {
    oscillator: PhaseAccumulator,
    filter: OnePoleFilter,
    amplitude: f32,
    target_amplitude: f32,
    frequency: f32,
    cutoff: f32,
    sample_rate: f32,
}

impl SmoothiePlugin for BasicSynth {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "Smoothie Synth",
            vendor: "Smoothie Audio",
            version: "1.0.0",
            category: PluginCategory::Instrument,
            input_channels: 0,
            output_channels: 2,
        }
    }

    fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: PhaseAccumulator::new(440.0, sample_rate),
            filter: OnePoleFilter::new(0.3),
            amplitude: 0.0,
            target_amplitude: 0.0,
            frequency: 440.0,
            cutoff: 0.3,
            sample_rate,
        }
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        if buffer.is_empty() { return ProcessStatus::Error; }

        let block_len = buffer[0].len();

        for i in 0..block_len {
            // Simple amplitude envelope (attack/release smoothing)
            self.amplitude += 0.001 * (self.target_amplitude - self.amplitude);

            // Generate saw wave from phase accumulator
            let phase = self.oscillator.next();
            let saw = (phase / core::f32::consts::PI) - 1.0; // Normalize to [-1, 1]

            // Apply filter
            let filtered = self.filter.process(saw);

            // Apply amplitude
            let output = filtered * self.amplitude * 0.5;

            // Write to all output channels
            for ch in buffer.iter_mut() {
                if i < ch.len() {
                    ch[i] = output;
                }
            }
        }

        if self.amplitude < 0.0001 && self.target_amplitude < 0.0001 {
            ProcessStatus::Tail
        } else {
            ProcessStatus::Ok
        }
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.oscillator = PhaseAccumulator::new(self.frequency, sr);
    }

    fn reset(&mut self) {
        self.oscillator.reset();
        self.filter.reset();
        self.amplitude = 0.0;
        self.target_amplitude = 0.0;
    }

    fn param_count(&self) -> usize { 3 }

    fn get_param(&self, index: usize) -> f32 {
        match index {
            0 => self.frequency,
            1 => self.cutoff,
            2 => self.target_amplitude,
            _ => 0.0,
        }
    }

    fn set_param(&mut self, index: usize, value: f32) {
        match index {
            0 => {
                self.frequency = value.clamp(20.0, 20000.0);
                self.oscillator.set_frequency(self.frequency, self.sample_rate);
            }
            1 => {
                self.cutoff = value.clamp(0.01, 0.49);
                self.filter = OnePoleFilter::new(self.cutoff);
            }
            2 => self.target_amplitude = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        match index {
            0 => "Frequency",
            1 => "Filter Cutoff",
            2 => "Amplitude",
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silent_when_off() {
        let mut synth = BasicSynth::new(44100.0);
        let mut left = [0.0f32; 64];
        let mut right = [0.0f32; 64];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        synth.process(&mut channels);
        assert!(left[63].abs() < 0.001);
    }

    #[test]
    fn test_produces_sound() {
        let mut synth = BasicSynth::new(44100.0);
        synth.set_param(2, 0.8); // Turn on
        let mut left = [0.0f32; 4096];
        let mut right = [0.0f32; 4096];
        let mut channels: Vec<&mut [f32]> = vec![&mut left, &mut right];
        synth.process(&mut channels);
        // After several thousand samples, we should have signal
        assert!(left[4000].abs() > 0.001);
    }
}
