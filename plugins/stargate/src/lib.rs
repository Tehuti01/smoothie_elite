/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x53544152 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/lib.rs                                        │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: STARGATE Flagship Synthesizer.                              │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

pub mod core;
pub mod dsp;
pub mod params;
pub mod ui;

pub use crate::core::StargateCore as StargateSynth;
use smoothie_core::plugin::{SmoothiePlugin, PluginInfo, PluginCategory, ProcessStatus, Reset, Latency, TailTime};
use crate::params::mapping::StargateState;

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
        let state = StargateState::from_bank(&self.params);
        let num_samples = buffer[0].len();
        let mut rms = 0.0;
        
        for i in 0..num_samples {
            // 1. Core Engine (Osc + Filter)
            let synth_out = self.engine.process(state.cutoff as f64, state.resonance as f64, self.sample_rate as f64);

            // 2. Effects Chain
            let output = self.effects.process(synth_out, self.sample_rate);
            rms += output * output;

            // Write to all channels (Mono to Stereo)
            for channel in buffer.iter_mut() {
                channel[i] = output;
            }
        }
        
        // 3. Update UI state (non-blocking)
        let final_rms = (rms / num_samples as f32).sqrt();
        self.ui.update_meter(final_rms);

        ProcessStatus::Ok
    }

    fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        // In a full implementation, we would also cascade this down to engine and fx
    }

    fn param_count(&self) -> usize {
        3 // Cutoff, Resonance, Drive
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

impl Reset for StargateSynth {
    fn reset(&mut self) {
        self.engine.reset();
        self.effects.reset();
    }
}

impl Latency for StargateSynth {}
impl TailTime for StargateSynth {}
