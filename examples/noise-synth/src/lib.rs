//! Noise texture synthesizer with colored noise and filtering.
//!
//! Features:
//! - White, pink, and brown noise generation
//! - Multi-mode filtering (LP, HP, BP)
//! - Envelope control for impact/texture shaping
//! - Real-time noise color selection

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_effects::{BiquadFilter, FilterType, Envelope};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Simple linear congruential generator for noise.
struct NoiseGenerator {
    seed: u32,
}

impl NoiseGenerator {
    fn new() -> Self {
        Self { seed: 0x12345678 }
    }

    fn next(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.seed >> 16) & 0x7fff) as f32 / 32768.0 - 0.5
    }

    fn white_noise(&mut self) -> f32 {
        self.next() * 2.0
    }

    fn pink_noise(&mut self) -> f32 {
        // Simple one-pole low-pass filter for pink noise approximation
        static mut PREV: f32 = 0.0;
        let white = self.white_noise();
        unsafe {
            PREV = white * 0.049922035 + PREV * 0.950077965;
            PREV
        }
    }

    fn brown_noise(&mut self) -> f32 {
        // Two-pole low-pass for brown noise
        static mut PREV1: f32 = 0.0;
        static mut PREV2: f32 = 0.0;
        let white = self.white_noise();
        unsafe {
            PREV1 = white * 0.049922035 + PREV1 * 0.950077965;
            PREV2 = PREV1 * 0.049922035 + PREV2 * 0.950077965;
            PREV2
        }
    }
}

/// Noise synthesizer with colored noise and spectral shaping.
pub struct NoiseSynthPlugin {
    noise: NoiseGenerator,
    filter: BiquadFilter,
    envelope: Envelope,

    // Noise type: 0=white, 1=pink, 2=brown
    noise_type: Arc<FloatParam>,

    // Filter parameters
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    filter_type: Arc<FloatParam>,
    filter_env: Arc<FloatParam>,

    // Envelope
    attack: Arc<FloatParam>,
    decay: Arc<FloatParam>,
    sustain: Arc<FloatParam>,
    release: Arc<FloatParam>,

    // Mix
    master: Arc<FloatParam>,
    sample_rate: f32,
}

impl Default for NoiseSynthPlugin {
    fn default() -> Self {
        Self {
            noise: NoiseGenerator::new(),
            filter: BiquadFilter::design(FilterType::LowPass, 8000.0, 44100.0, 1.0, 0.0),
            envelope: Envelope::new(0.05, 0.2, 0.6, 0.5, 44100.0),

            noise_type: Arc::new(FloatParam::simple("noise_type", 0.0, 0.0, 2.0)),
            cutoff: Arc::new(FloatParam::simple("cutoff", 8000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::simple("resonance", 1.0, 0.1, 10.0)),
            filter_type: Arc::new(FloatParam::simple("filter_type", 0.0, 0.0, 2.0)),
            filter_env: Arc::new(FloatParam::simple("filter_env", 0.5, 0.0, 1.0)),

            attack: Arc::new(FloatParam::simple("attack", 0.05, 0.001, 2.0)),
            decay: Arc::new(FloatParam::simple("decay", 0.2, 0.001, 2.0)),
            sustain: Arc::new(FloatParam::simple("sustain", 0.6, 0.0, 1.0)),
            release: Arc::new(FloatParam::simple("release", 0.5, 0.001, 5.0)),

            master: Arc::new(FloatParam::simple("master", 0.7, 0.0, 1.0)),
            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for NoiseSynthPlugin {
    const NAME: &'static str = "Noise";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.noise");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.noise_type) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.resonance) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_type) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_env) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.attack) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.decay) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.release) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        self.envelope = Envelope::new(
            self.attack.value(),
            self.decay.value(),
            self.sustain.value(),
            self.release.value(),
            self.sample_rate,
        );
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let noise_type_idx = (self.noise_type.value() as usize) % 3;
        let cutoff = self.cutoff.value();
        let resonance = self.resonance.value();
        let filter_type_idx = (self.filter_type.value() as usize) % 3;
        let filter_env_amt = self.filter_env.value();

        let attack = self.attack.value();
        let decay = self.decay.value();
        let sustain = self.sustain.value();
        let release = self.release.value();
        let master = self.master.value();

        // Select filter type
        let filter_type = match filter_type_idx {
            0 => FilterType::LowPass,
            1 => FilterType::HighPass,
            2 => FilterType::BandPass,
            _ => FilterType::LowPass,
        };

        // Update envelope
        self.envelope.set_attack(attack);
        self.envelope.set_decay(decay);
        self.envelope.set_sustain(sustain);
        self.envelope.set_release(release);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            // Generate noise based on type
            let noise = match noise_type_idx {
                0 => self.noise.white_noise(),
                1 => self.noise.pink_noise(),
                2 => self.noise.brown_noise(),
                _ => self.noise.white_noise(),
            };

            // Get envelope value
            let env_value = self.envelope.next_sample();

            // Modulate filter with envelope
            let modulated_cutoff = cutoff * (1.0 + filter_env_amt * (env_value - 0.5) * 2.0).max(0.1);

            // Update filter
            self.filter = BiquadFilter::design(filter_type, modulated_cutoff, self.sample_rate, resonance, 0.0);

            // Apply filter and envelope
            let filtered = self.filter.process(noise);
            let output = filtered * env_value * master;

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.noise = NoiseGenerator::new();
    }

    fn tail_length_samples(&self) -> u32 {
        (5.0 * self.sample_rate) as u32
    }
}

smoothie_export!(NoiseSynthPlugin);


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
