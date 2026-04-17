use smoothie_core::{
    AudioLayout, ProcessContext, InitContext, ProcessStatus, PluginUid, SmoothiePlugin,
};
use smoothie_params::prelude::*;
use std::sync::Arc;
use crate::audio::{Sample};
use crate::dsp::amplifiers::Amplifier;
use crate::dsp::cabinets::{Cabinet};
use crate::dsp::dynamics::{Compressor, NoiseGate, Limiter};
use crate::dsp::eq::Equalizer;

/// The IronStack High-End Guitar DSP Engine wrapped as a Smoothie Elite Plugin.
pub struct IronStackPlugin {
    // --- DSP Modules ---
    amp: Amplifier,
    cabinet: Cabinet,
    noise_gate: NoiseGate,
    compressor: Compressor,
    eq: Equalizer,
    limiter: Limiter,

    // --- Parameters ---
    pub params: Arc<IronStackParams>,
}

pub struct IronStackParams {
    pub input_gain: FloatParam,
    pub drive: FloatParam,
    pub bass: FloatParam,
    pub middle: FloatParam,
    pub treble: FloatParam,
    pub presence: FloatParam,
    pub gate_threshold: FloatParam,
    pub comp_threshold: FloatParam,
    pub master_volume: FloatParam,
}

impl Default for IronStackPlugin {
    fn default() -> Self {
        let sample_rate = 44100;
        Self {
            amp: Amplifier::new(sample_rate),
            cabinet: Cabinet::new(sample_rate),
            noise_gate: NoiseGate::new(sample_rate as f64),
            compressor: Compressor::new(sample_rate as f64),
            eq: Equalizer::new(sample_rate as f64),
            limiter: Limiter::new(sample_rate as f64),
            params: Arc::new(IronStackParams::default()),
        }
    }
}

impl Default for IronStackParams {
    fn default() -> Self {
        Self {
            input_gain: FloatParam::new("input_gain", "Input Gain", 1.0)
                .range(FloatRange::Linear { min: 0.0, max: 2.0 }),
            drive: FloatParam::new("drive", "Drive", 0.5)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
            bass: FloatParam::new("bass", "Bass", 0.5)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
            middle: FloatParam::new("middle", "Middle", 0.5)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
            treble: FloatParam::new("treble", "Treble", 0.5)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
            presence: FloatParam::new("presence", "Presence", 0.5)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
            gate_threshold: FloatParam::new("gate_threshold", "Gate Threshold", -60.0)
                .range(FloatRange::Linear { min: -100.0, max: 0.0 })
                .unit(" dB"),
            comp_threshold: FloatParam::new("comp_threshold", "Comp Threshold", -20.0)
                .range(FloatRange::Linear { min: -60.0, max: 0.0 })
                .unit(" dB"),
            master_volume: FloatParam::new("master_volume", "Master Volume", 0.7)
                .range(FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl SmoothiePlugin for IronStackPlugin {
    const NAME: &'static str = "IronStack Elite";
    const VENDOR: &'static str = "IronStack Audio";
    const VERSION: &'static str = "1.0.0";
    const UID: PluginUid = smoothie_core::uid!("com.ironstack.smoothie"); 

    fn audio_layouts() -> &'static [AudioLayout] {
        static LAYOUTS: &[AudioLayout] = &[
            AudioLayout::stereo_in_stereo_out(),
            AudioLayout::mono_in_stereo_out(),
        ];
        LAYOUTS
    }

    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as u32;
        self.amp = Amplifier::new(sr);
        self.cabinet = Cabinet::new(sr);
        self.noise_gate = NoiseGate::new(sr as f64);
        self.compressor = Compressor::new(sr as f64);
        self.eq = Equalizer::new(sr as f64);
        self.limiter = Limiter::new(sr as f64);
        true
    }

    fn reset(&mut self) {
        self.amp.reset();
        self.cabinet.reset();
        self.noise_gate.clear();
        // compressor has no explicit clear/reset, but envelope is updated in process
        self.eq.clear();
        self.limiter.reset();
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let num_samples = ctx.block_size() as usize;
        
        // Sync parameters
        self.amp.set_gain(self.params.drive.value());
        self.amp.set_tone(
            self.params.bass.value(),
            self.params.middle.value(),
            self.params.treble.value()
        );
        self.amp.set_presence(self.params.presence.value());
        
        self.noise_gate.set_threshold(self.params.gate_threshold.value());
        self.compressor.set_threshold(self.params.comp_threshold.value());
        
        let input_gain = self.params.input_gain.value();
        let master_volume = self.params.master_volume.value();

        let num_input_channels = ctx.buffer().channels();

        for i in 0..num_samples {
            // Read input
            let l_in = ctx.buffer().channel(0)[i] * input_gain;
            let r_in = if num_input_channels > 1 {
                ctx.buffer().channel(1)[i] * input_gain
            } else {
                l_in
            };

            // Dynamics: Gate
            let (l_gate, r_gate) = self.noise_gate.process_stereo(l_in, r_in);
            
            // Core: Amp + Cab
            let mut sample = Sample::new(l_gate, r_gate);
            sample = self.amp.process(sample);
            sample = self.cabinet.process(sample);
            
            // Filtering: Post-EQ
            let (l_eq, r_eq) = self.eq.process_stereo(sample.left, sample.right);
            
            // Dynamics: Opto-Compressor
            let (l_comp, r_comp) = self.compressor.process_stereo(l_eq, r_eq);
            
            // Safety: Limiter
            let (l_limit, r_limit) = self.limiter.process_stereo(l_comp, r_comp);
            
            // Write to output
            ctx.buffer_mut().channel_mut(0)[i] = l_limit * master_volume;
            if ctx.buffer_mut().channels() > 1 {
                ctx.buffer_mut().channel_mut(1)[i] = r_limit * master_volume;
            }
        }

        ProcessStatus::Normal
    }
}
