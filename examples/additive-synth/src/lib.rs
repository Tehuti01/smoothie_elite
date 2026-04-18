//! Additive synthesizer combining harmonic oscillators.
//!
//! Features:
//! - 16 harmonic oscillators per voice
//! - Individual harmonic amplitude control
//! - Harmonic richness parameter for spectral shaping
//! - Real-time harmonic envelope control

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_params::FloatParam;
use std::sync::Arc;
use std::f32::consts::PI;

/// Single additive oscillator voice with 16 harmonics.
struct AdditiveVoice {
    phases: [f32; 16],  // Phase for each harmonic
    frequency: f32,
    amplitude: f32,
    sample_rate: f32,
}

impl AdditiveVoice {
    fn new(sample_rate: f32) -> Self {
        Self {
            phases: [0.0; 16],
            frequency: 440.0,
            amplitude: 0.0,
            sample_rate,
        }
    }

    #[allow(dead_code)]
    fn note_on(&mut self, note: u8, velocity: u8) {
        let freq = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
        self.frequency = freq;
        self.amplitude = velocity as f32 / 127.0;
        self.phases.fill(0.0);
    }

    #[allow(dead_code)]
    fn note_off(&mut self) {
        self.amplitude = 0.0;
    }

    fn process(&mut self, harmonics: &[f32; 16]) -> f32 {
        let mut output = 0.0;

        for h in 0..16 {
            let phase = self.phases[h];
            let harmonic = (h + 1) as f32;
            let harmonic_freq = self.frequency * harmonic;
            let phase_inc = 2.0 * PI * harmonic_freq / self.sample_rate;

            // Sine oscillator with harmonic amplitude
            output += phase.sin() * harmonics[h];

            // Advance phase for next sample
            self.phases[h] = (phase + phase_inc) % (2.0 * PI);
        }

        output * self.amplitude / 16.0  // Normalize
    }
}

/// Additive synthesizer: combining harmonic oscillators for rich timbres.
pub struct AdditiveSynthPlugin {
    voices: [AdditiveVoice; 8],

    // Harmonic controls (16 harmonics)
    harmonic_amps: [Arc<FloatParam>; 8],  // Control first 8 harmonics visibly
    harmonic_richness: Arc<FloatParam>,   // Master richness (controls fade-out of higher harmonics)
    attack: Arc<FloatParam>,
    release: Arc<FloatParam>,
    master_level: Arc<FloatParam>,
}

impl Default for AdditiveSynthPlugin {
    fn default() -> Self {
        Self {
            voices: [
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
                AdditiveVoice::new(44100.0),
            ],
            harmonic_amps: [
                Arc::new(FloatParam::simple("Harmonic 1", 1.0, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 2", 0.7, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 3", 0.5, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 4", 0.3, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 5", 0.2, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 6", 0.15, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 7", 0.1, 0.0, 1.0)),
                Arc::new(FloatParam::simple("Harmonic 8", 0.05, 0.0, 1.0)),
            ],
            harmonic_richness: Arc::new(FloatParam::simple("Richness", 0.8, 0.0, 1.0)),
            attack: Arc::new(FloatParam::simple("Attack", 0.01, 0.001, 1.0)),
            release: Arc::new(FloatParam::simple("Release", 0.1, 0.001, 2.0)),
            master_level: Arc::new(FloatParam::simple("Master", 0.8, 0.0, 1.0)),
        }
    }
}

impl SmoothiePlugin for AdditiveSynthPlugin {
    const NAME: &'static str = "Additive";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.additive");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        let mut params: Vec<Arc<dyn smoothie_params::Param>> = self.harmonic_amps
            .iter()
            .map(|p| Arc::clone(p) as Arc<dyn smoothie_params::Param>)
            .collect();

        params.push(Arc::clone(&self.harmonic_richness) as Arc<dyn smoothie_params::Param>);
        params.push(Arc::clone(&self.attack) as Arc<dyn smoothie_params::Param>);
        params.push(Arc::clone(&self.release) as Arc<dyn smoothie_params::Param>);
        params.push(Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>);

        params
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        for voice in &mut self.voices {
            voice.sample_rate = sr;
        }
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Build harmonic amplitude array with richness modulation
        let richness = self.harmonic_richness.value();
        let mut harmonics = [0.0; 16];

        for i in 0..8 {
            harmonics[i] = self.harmonic_amps[i].value();
        }

        // Higher harmonics fade based on richness (0 = sine, 1 = all harmonics)
        for i in 8..16 {
            let harmonic_idx = i as f32 / 16.0;
            let fade = (1.0 - harmonic_idx).max(0.0) * richness;
            harmonics[i] = fade * 0.5;
        }

        let master = self.master_level.value();
        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            let mut output = 0.0;

            for voice in &mut self.voices {
                output += voice.process(&harmonics);
            }

            output = (output / 8.0) * master;

            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.amplitude = 0.0;
        }
    }

    fn tail_length_samples(&self) -> u32 {
        (2.0 * 44100.0) as u32
    }
}

smoothie_export!(AdditiveSynthPlugin);
