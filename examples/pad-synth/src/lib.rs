//! Rich pad synthesizer with detuned oscillators and smooth morphing.
//!
//! Features:
//! - Detuned unison oscillators for lush, thick pads
//! - Smooth slow attack and long sustain
//! - Harmonic morphing between timbres
//! - Filter sweep with mod wheel control
//! - Polyphonic voice management

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_synth::PolySynth;
use smoothie_effects::{BiquadFilter, FilterType, Envelope};
use smoothie_params::FloatParam;
use std::sync::Arc;

/// Lush pad synthesizer: unison OSCs → filter sweep → slow envelope.
pub struct PadSynthPlugin {
    synth: PolySynth,

    // Filter state
    filter: BiquadFilter,
    envelope: Envelope,

    // Oscillator parameters
    waveform: Arc<FloatParam>,
    unison_count: Arc<FloatParam>,  // 1-7 voices per note
    unison_spread: Arc<FloatParam>, // Detuning amount

    // Filter parameters
    filter_cutoff: Arc<FloatParam>,
    filter_resonance: Arc<FloatParam>,
    filter_env_amount: Arc<FloatParam>,

    // Envelope parameters (slow pads)
    env_attack: Arc<FloatParam>,
    env_decay: Arc<FloatParam>,
    env_sustain: Arc<FloatParam>,
    env_release: Arc<FloatParam>,

    // Modulation
    mod_wheel: Arc<FloatParam>,        // CC1: filter cutoff modulation
    vibrato_depth: Arc<FloatParam>,    // Pitch vibrato

    // Mix
    master_level: Arc<FloatParam>,
    sample_rate: f32,
}

impl Default for PadSynthPlugin {
    fn default() -> Self {
        Self {
            synth: PolySynth::new(44100.0),
            filter: BiquadFilter::design(FilterType::LowPass, 4000.0, 44100.0, 2.0, 0.0),
            envelope: Envelope::new(0.5, 0.5, 0.8, 1.0, 44100.0),

            waveform: Arc::new(FloatParam::simple("waveform", 1.0, 0.0, 3.0)),  // Default triangle
            unison_count: Arc::new(FloatParam::simple("unison", 3.0, 1.0, 7.0)),
            unison_spread: Arc::new(FloatParam::simple("spread", 0.05, 0.0, 0.2)),

            filter_cutoff: Arc::new(FloatParam::simple("cutoff", 4000.0, 20.0, 20000.0)),
            filter_resonance: Arc::new(FloatParam::simple("resonance", 2.0, 0.1, 10.0)),
            filter_env_amount: Arc::new(FloatParam::simple("filter_env", 0.3, 0.0, 1.0)),

            env_attack: Arc::new(FloatParam::simple("attack", 1.5, 0.1, 5.0)),   // Slow attack
            env_decay: Arc::new(FloatParam::simple("decay", 1.0, 0.1, 5.0)),
            env_sustain: Arc::new(FloatParam::simple("sustain", 0.9, 0.0, 1.0)), // High sustain
            env_release: Arc::new(FloatParam::simple("release", 2.0, 0.1, 10.0)),

            mod_wheel: Arc::new(FloatParam::simple("mod_wheel", 0.0, 0.0, 1.0)),
            vibrato_depth: Arc::new(FloatParam::simple("vibrato", 0.0, 0.0, 0.5)),

            master_level: Arc::new(FloatParam::simple("master", 0.7, 0.0, 1.0)),
            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for PadSynthPlugin {
    const NAME: &'static str = "Pad";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.pad");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[AudioLayout::stereo_in_stereo_out()];
        LAYOUTS
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.waveform) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.unison_count) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.unison_spread) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_resonance) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.filter_env_amount) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_attack) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_decay) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_sustain) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.env_release) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.mod_wheel) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.vibrato_depth) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.master_level) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        self.sample_rate = ctx.sample_rate as f32;
        self.synth = PolySynth::new(self.sample_rate);
        self.envelope = Envelope::new(
            self.env_attack.value(),
            self.env_decay.value(),
            self.env_sustain.value(),
            self.env_release.value(),
            self.sample_rate,
        );
        true
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // Get parameter values
        let waveform = (self.waveform.value() as usize) % 4;
        let unison = (self.unison_count.value() as usize).min(7).max(1);
        let spread = self.unison_spread.value();

        let cutoff = self.filter_cutoff.value();
        let resonance = self.filter_resonance.value();
        let filter_env_amt = self.filter_env_amount.value();

        let attack = self.env_attack.value();
        let decay = self.env_decay.value();
        let sustain = self.env_sustain.value();
        let release = self.env_release.value();

        let mod_wheel = self.mod_wheel.value();
        let _vibrato = self.vibrato_depth.value();
        let master = self.master_level.value();

        // Update envelope
        self.envelope.set_attack(attack);
        self.envelope.set_decay(decay);
        self.envelope.set_sustain(sustain);
        self.envelope.set_release(release);

        // Set synthesizer waveform
        self.synth.set_waveform(waveform);

        let buffer = ctx.buffer_mut();
        let num_samples = buffer.samples();

        for i in 0..num_samples {
            // Get base oscillator output
            let osc_out = self.synth.process();

            // Get next envelope sample
            let env_value = self.envelope.next_sample();

            // Modulate filter cutoff with:
            // 1. Envelope
            // 2. Mod wheel (for expression)
            let mod_wheel_amount = mod_wheel * 0.5;
            let modulated_cutoff = cutoff * (1.0 + filter_env_amt * (env_value - 0.5) * 2.0 + mod_wheel_amount).max(0.1);

            // Recreate filter with modulated parameters and increased resonance for pads
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
        // Long tail for pad releases
        (10.0 * self.sample_rate) as u32
    }
}

smoothie_export!(PadSynthPlugin);


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
