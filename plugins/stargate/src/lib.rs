/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x53544152 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/lib.rs                                        │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: STARGATE Flagship Synthesizer.                              │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_core::plugin::{SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus, Reset, Latency, TailTime};
use smoothie_dsp::filters::BiquadFilter;
use smoothie_dsp::oscillators::WavetableOscillator;
use smoothie_dsp::utils::SoftClipper;
use smoothie_effects::{Chorus, DelayEffect, ReverbEffect};
use smoothie_params::{ParameterBank, ParameterInfo, ParameterRange, ParameterType, ParameterUnit};
use smoothie_modulation::matrix::ModMatrix;
use smoothie_ui_core::{DARK_THEME, Knob, Fader, VuMeter};
use smoothie_ui_vfx::fractal::FractalVisualizer;

pub struct StargateSynth {
    params: ParameterBank,
    oscillators: [WavetableOscillator; 2],
    filter: BiquadFilter,
    chorus: Chorus,
    delay: DelayEffect,
    reverb: ReverbEffect,
    clipper: SoftClipper,
    sample_rate: f32,
    _mod_matrix: ModMatrix,
    
    // UI State (Holographic Widgets)
    pub ui_knob_cutoff: Knob,
    pub ui_knob_res: Knob,
    pub ui_fader_drive: Fader,
    pub ui_meter: VuMeter,
    pub ui_visualizer: FractalVisualizer,
}

impl StargateSynth {
    pub fn new(sample_rate: f32) -> Self {
        let mut bank = ParameterBank::new();
        
        // Define STARGATE parameters
        bank.register(ParameterInfo {
            name: "Cutoff",
            param_type: ParameterType::Float,
            unit: ParameterUnit::Hertz,
            range: ParameterRange { min: 20.0, max: 20000.0, default: 1000.0 },
        });
        bank.register(ParameterInfo {
            name: "Resonance",
            param_type: ParameterType::Float,
            unit: ParameterUnit::Generic,
            range: ParameterRange { min: 0.01, max: 10.0, default: 0.707 },
        });
        bank.register(ParameterInfo {
            name: "Drive",
            param_type: ParameterType::Float,
            unit: ParameterUnit::Generic,
            range: ParameterRange { min: 1.0, max: 10.0, default: 1.0 },
        });

        Self {
            params: bank,
            oscillators: [
                WavetableOscillator::new(440.0, sample_rate),
                WavetableOscillator::new(440.0 * 1.5, sample_rate),
            ],
            filter: BiquadFilter::new(),
            chorus: Chorus::new(sample_rate),
            delay: DelayEffect::default(),
            reverb: ReverbEffect::new(sample_rate),
            clipper: SoftClipper::new(0.1),
            sample_rate,
            _mod_matrix: ModMatrix::new(),
            
            ui_knob_cutoff: Knob::new("Cutoff"),
            ui_knob_res: Knob::new("Resonance"),
            ui_fader_drive: Fader::new("Drive"),
            ui_meter: VuMeter::new(),
            ui_visualizer: FractalVisualizer::new(),
        }
    }

    pub fn render_ui(&mut self) {
        // Seraphic UI Orchestration
        let _theme = &DARK_THEME;
        
        // Update UI widgets from parameters
        self.ui_knob_cutoff.value = self.params.get_value("Cutoff").unwrap_or(0.5);
        self.ui_knob_res.value = self.params.get_value("Resonance").unwrap_or(0.5);
        self.ui_fader_drive.value = self.params.get_value("Drive").unwrap_or(0.0);
    }
}

impl SmoothiePlugin for StargateSynth {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "STARGATE",
            vendor: "Seraphic Technologies",
            version: "0.1.0",
            category: PluginCategory::Instrument,
            input_channels: 0,
            output_channels: 2,
            description: "Elite Geometric Flagship Synthesizer",
            website: "https://seraphic.tech/stargate",
        }
    }

    fn new(sample_rate: f32) -> Self {
        Self::new(sample_rate)
    }

    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
        let cutoff = self.params.get_value("Cutoff").unwrap_or(1000.0);
        let res = self.params.get_value("Resonance").unwrap_or(0.707);
        let drive = self.params.get_value("Drive").unwrap_or(1.0);

        self.filter.set_lowpass(cutoff, self.sample_rate, res);

        let num_samples = buffer[0].len();
        let mut rms = 0.0;
        
        for i in 0..num_samples {
            // 1. Generate Oscillators
            let osc_out = (self.oscillators[0].process() + self.oscillators[1].process()) * 0.5;

            // 2. Filter
            let filtered = self.filter.process(osc_out);

            // 3. Nonlinear Drive
            let saturated = smoothie_core::math::soft_saturate(filtered, drive);

            // 4. Effects Chain
            let post_chorus = self.chorus.process(saturated, self.sample_rate);
            let post_delay = self.delay.process(post_chorus);
            let post_reverb = self.reverb.process(post_delay);

            // 5. Final Output Safety
            let output = self.clipper.process(post_reverb);
            rms += output * output;

            // Write to all channels (Mono to Stereo)
            for channel in buffer.iter_mut() {
                channel[i] = output;
            }
        }
        
        // Update VU Meter & Visualizer
        let final_rms = (rms / num_samples as f32).sqrt();
        self.ui_meter.update(final_rms);
        self.ui_visualizer.intensity = final_rms * 2.0;

        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }

    fn param_count(&self) -> usize {
        3
    }

    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).map(|p| p.atomic.load()).unwrap_or(0.0)
    }

    fn set_param(&mut self, index: usize, value: f32) {
        if let Some(p) = self.params.get(index) {
            p.atomic.store(value);
        }
    }

    fn get_param_name(&self, index: usize) -> &'static str {
        self.params.get(index).map(|p| p.info.name).unwrap_or("")
    }
}

// Implement required traits for SmoothiePlugin
impl Reset for StargateSynth {
    fn reset(&mut self) {
        for osc in &mut self.oscillators {
            osc.reset();
        }
        self.filter.reset();
        self.reverb.reset();
    }
}

impl Latency for StargateSynth {}
impl TailTime for StargateSynth {}
