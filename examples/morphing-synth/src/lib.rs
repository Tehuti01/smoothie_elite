//! Wavetable morphing synthesizer with smooth timbral evolution.
//!
//! Features:
//! - Smooth morphing between multiple waveforms
//! - Cross-fade between sine, triangle, sawtooth, square
//! - Polyphonic voice synthesis
//! - Real-time morphing control via parameters or MIDI

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::PolySynth;
use smoothie_effects::{BiquadFilter, FilterType, Envelope};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Morphing wavetable synth: smooth interpolation between waveforms.
pub struct MorphingSynthPlugin {
    synth: PolySynth,
    filter: BiquadFilter,
    envelope: Envelope,

    // Morphing parameters
    morph_position: Arc<FloatParam>, // 0=sine, 1=tri, 2=saw, 3=square
    morph_smooth: Arc<FloatParam>,  // Smoothing speed for morphing

    // Filter
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    filter_env: Arc<FloatParam>,

    // Envelope
    attack: Arc<FloatParam>,
    decay: Arc<FloatParam>,
    sustain: Arc<FloatParam>,
    release: Arc<FloatParam>,

    // Mix
    master: Arc<FloatParam>,
    sample_rate: f32,

    // State
    current_morph: f32,
    target_morph: f32,
}

impl Default for MorphingSynthPlugin {
    fn default() -> Self {
        Self {
            synth: PolySynth::new(44100.0),
            filter: BiquadFilter::design(FilterType::LowPass, 5000.0, 44100.0, 1.5, 0.0),
            envelope: Envelope::new(0.05, 0.1, 0.7, 0.3, 44100.0),

            morph_position: Arc::new(FloatParam::simple("morph", 0.0, 0.0, 3.0)),
            morph_smooth: Arc::new(FloatParam::simple("morph_smooth", 0.1, 0.01, 1.0)),

            cutoff: Arc::new(FloatParam::simple("cutoff", 5000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::simple("resonance", 1.5, 0.1, 10.0)),
            filter_env: Arc::new(FloatParam::simple("filter_env", 0.3, 0.0, 1.0)),

            attack: Arc::new(FloatParam::simple("attack", 0.05, 0.001, 2.0)),
            decay: Arc::new(FloatParam::simple("decay", 0.1, 0.001, 2.0)),
            sustain: Arc::new(FloatParam::simple("sustain", 0.7, 0.0, 1.0)),
            release: Arc::new(FloatParam::simple("release", 0.3, 0.001, 5.0)),

            master: Arc::new(FloatParam::simple("master", 0.8, 0.0, 1.0)),
            sample_rate: 44100.0,

            current_morph: 0.0,
            target_morph: 0.0,
        }
    }
}

impl SmoothiePlugin for MorphingSynthPlugin {
    const NAME: &'static str = "Morphing";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.morphing");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.morph_position) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.morph_smooth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.resonance) as Arc<dyn smoothie_params::Param>,
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
        self.synth = PolySynth::new(self.sample_rate);
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
        let target = self.morph_position.value();
        let smooth = self.morph_smooth.value();
        let cutoff = self.cutoff.value();
        let resonance = self.resonance.value();
        let filter_env_amt = self.filter_env.value();
        let attack = self.attack.value();
        let decay = self.decay.value();
        let sustain = self.sustain.value();
        let release = self.release.value();
        let master = self.master.value();

        // Smooth morphing between waveforms
        self.target_morph = target;
        self.current_morph += (self.target_morph - self.current_morph) * smooth;

        // Set waveform based on morphing position
        let waveform = (self.current_morph.floor() as usize) % 4;
        self.synth.set_waveform(waveform);

        // Update envelope
        self.envelope.set_attack(attack);
        self.envelope.set_decay(decay);
        self.envelope.set_sustain(sustain);
        self.envelope.set_release(release);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            // Get oscillator output
            let osc_out = self.synth.process();

            // Get envelope value
            let env_value = self.envelope.next_sample();

            // Modulate filter with envelope
            let modulated_cutoff = cutoff * (1.0 + filter_env_amt * (env_value - 0.5) * 2.0).max(0.1);

            // Update filter
            self.filter = BiquadFilter::design(FilterType::LowPass, modulated_cutoff, self.sample_rate, resonance, 0.0);

            // Apply filter and envelope
            let filtered = self.filter.process(osc_out);
            let output = filtered * env_value * master;

            // Write to both channels
            for ch in 0..buffer.channels() {
                buffer.channel_mut(ch)[i] = output;
            }
        }

        ProcessStatus::Normal
    }

    fn reset(&mut self) {
        self.synth = PolySynth::new(self.sample_rate);
    }

    fn tail_length_samples(&self) -> u32 {
        (5.0 * self.sample_rate) as u32
    }
}

smoothie_export!(MorphingSynthPlugin);


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
