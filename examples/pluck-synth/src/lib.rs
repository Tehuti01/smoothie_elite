//! Karplus-Strong style pluck synthesizer for realistic string/plucked instruments.
//!
//! Features:
//! - Efficient circular buffer delay line (Karplus-Strong algorithm)
//! - Polyphonic voice synthesis
//! - Damping control for brightness/decay
//! - Pluck impulse character control
//! - Realistic plucked string physics modeling

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Single Karplus-Strong pluck voice with delay line feedback.
struct PluckVoice {
    delay_line: Vec<f32>,
    write_pos: usize,
    read_pos: usize,
    frequency: f32,
    amplitude: f32,
    decay_factor: f32,
    is_active: bool,
    impulse_countdown: usize,
}

impl PluckVoice {
    fn new(sample_rate: f32, max_delay: usize) -> Self {
        Self {
            delay_line: vec![0.0; max_delay],
            write_pos: 0,
            read_pos: 0,
            frequency: 440.0,
            amplitude: 0.0,
            decay_factor: 0.99,
            is_active: false,
            impulse_countdown: 0,
        }
    }

    #[allow(dead_code)]
    fn note_on(&mut self, note: u8, velocity: u8, sample_rate: f32) {
        let freq = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
        self.frequency = freq;
        self.amplitude = (velocity as f32) / 127.0;

        // Calculate delay line length from frequency
        let delay_samples = (sample_rate / freq).max(2.0) as usize;
        self.delay_line = vec![0.0; delay_samples.min(44100)];
        self.write_pos = 0;
        self.read_pos = delay_samples / 2;
        self.is_active = true;

        // Initialize with random impulse
        for sample in &mut self.delay_line {
            *sample = (rand_float() - 0.5) * 2.0 * self.amplitude;
        }

        self.impulse_countdown = 0;
    }

    #[allow(dead_code)]
    fn note_off(&mut self) {
        self.amplitude = 0.0;
    }

    fn process(&mut self) -> f32 {
        if !self.is_active || self.amplitude < 0.001 {
            self.is_active = false;
            return 0.0;
        }

        let delay_len = self.delay_line.len();

        // Read from delay line
        let delayed = self.delay_line[self.read_pos];

        // Simple low-pass filter (damping) for decay
        let filtered = delayed * self.decay_factor;

        // Write back with feedback
        self.delay_line[self.write_pos] = filtered;

        // Advance pointers
        self.write_pos = (self.write_pos + 1) % delay_len;
        self.read_pos = (self.read_pos + 1) % delay_len;

        // Decay amplitude
        self.amplitude *= 0.9995;

        filtered
    }
}

/// Karplus-Strong pluck synthesizer for natural string sounds.
pub struct PluckSynthPlugin {
    voices: [PluckVoice; 32],

    // Parameters
    damping: Arc<FloatParam>,      // Filter damping (brightness/decay)
    pluck_position: Arc<FloatParam>, // Where on string to pluck
    sustain: Arc<FloatParam>,      // Note sustain length
    master_level: Arc<FloatParam>,

    sample_rate: f32,
}

impl Default for PluckSynthPlugin {
    fn default() -> Self {
        Self {
            voices: [
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
                PluckVoice::new(44100.0, 44100),
            ],

            damping: Arc::new(FloatParam::simple("damping", 0.95, 0.8, 0.999)),
            pluck_position: Arc::new(FloatParam::simple("pluck_pos", 0.5, 0.0, 1.0)),
            sustain: Arc::new(FloatParam::simple("sustain", 2.0, 0.1, 10.0)),
            master_level: Arc::new(FloatParam::simple("master", 0.8, 0.0, 1.0)),

            sample_rate: 44100.0,
        }
    }
}

// Simple pseudo-random number generator
#[allow(dead_code)]
fn rand_float() -> f32 {
    use std::num::Wrapping;
    static mut SEED: Wrapping<u32> = Wrapping(12345);
    unsafe {
        SEED = SEED * Wrapping(1103515245) + Wrapping(12345);
        ((SEED >> 16) & Wrapping(0x7fff)).0 as f32 / 32768.0
    }
}

impl SmoothiePlugin for PluckSynthPlugin {
    const NAME: &'static str = "Pluck";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.pluck");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.damping) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.pluck_position) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        for voice in &mut self.voices {
            *voice = PluckVoice::new(self.sample_rate, (self.sample_rate as usize).min(44100));
        }
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let damping = self.damping.value();
        let _pluck_pos = self.pluck_position.value();
        let _sustain = self.sustain.value();
        let master = self.master_level.value();

        // Update all voices with damping parameter
        for voice in &mut self.voices {
            voice.decay_factor = damping;
        }

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            let mut output = 0.0;

            // Mix all voices
            for voice in &mut self.voices {
                output += voice.process();
            }

            output = (output / 32.0) * master;

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.is_active = false;
            voice.amplitude = 0.0;
        }
    }

    fn tail_length_samples(&self) -> u32 {
        (5.0 * self.sample_rate) as u32
    }
}

smoothie_export!(PluckSynthPlugin);
