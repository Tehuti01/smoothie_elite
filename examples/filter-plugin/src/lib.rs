//! A professional multi-mode filter plugin.
//!
//! Demonstrates:
//! - Second-order IIR biquad filters
//! - Parameter automation
//! - Real-time safe DSP

#[macro_use]
extern crate smoothie_core;

use smoothie_core::prelude::*;
use smoothie_effects::BiquadFilter;
use smoothie_effects::FilterType;
use smoothie_params::{FloatParam, Param};
use std::sync::Arc;

/// Professional multi-mode filter plugin using biquad technology.
pub struct ProFilter {
    // DSP
    filter_l: BiquadFilter,
    filter_r: BiquadFilter,

    // Parameters (real-time safe, atomic updates)
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    mode: Arc<FloatParam>,  // 0=Lowpass, 1=Highpass, 2=Bandpass
    sample_rate: f32,
}

impl Default for ProFilter {
    fn default() -> Self {
        Self {
            filter_l: BiquadFilter::design(FilterType::LowPass, 2000.0, 44100.0, 1.0, 0.0),
            filter_r: BiquadFilter::design(FilterType::LowPass, 2000.0, 44100.0, 1.0, 0.0),
            cutoff: Arc::new(FloatParam::simple("cutoff", 2000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::simple("resonance", 1.0, 0.1, 10.0)),
            mode: Arc::new(FloatParam::simple("mode", 0.0, 0.0, 2.0)),  // 3 modes: 0, 1, 2
            sample_rate: 44100.0,
        }
    }
}

impl SmoothiePlugin for ProFilter {
    // ── Identity ────────────────────────────────────────────────────────────────
    const NAME: &'static str = "Pro Filter";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid::new("com.seraphicsonic.profilter");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    // ── Audio I/O ───────────────────────────────────────────────────────────────
    fn audio_layouts() -> &'static [AudioLayout] {
        const LAYOUTS: &[AudioLayout] = &[
            AudioLayout::stereo_in_stereo_out(),
            AudioLayout::mono_in_stereo_out(),
        ];
        LAYOUTS
    }

    // ── Parameters ──────────────────────────────────────────────────────────────
    fn parameters(&self) -> Vec<Arc<dyn Param>> {
        vec![
            Arc::clone(&self.cutoff) as Arc<dyn Param>,
            Arc::clone(&self.resonance) as Arc<dyn Param>,
            Arc::clone(&self.mode) as Arc<dyn Param>,
        ]
    }

    // ── Lifecycle ───────────────────────────────────────────────────────────────
    fn initialize(&mut self, ctx: &mut InitContext) -> bool {
        let sr = ctx.sample_rate as f32;
        self.sample_rate = sr;
        true
    }

    fn reset(&mut self) {
        self.filter_l = BiquadFilter::design(FilterType::LowPass, 2000.0, self.sample_rate, 1.0, 0.0);
        self.filter_r = BiquadFilter::design(FilterType::LowPass, 2000.0, self.sample_rate, 1.0, 0.0);
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let cutoff = self.cutoff.value();
        let q = self.resonance.value(); // Map resonance to Q factor
        let mode_idx = self.mode.value() as usize;

        // Select filter type based on mode
        let filter_type = match mode_idx {
            0 => FilterType::LowPass,
            1 => FilterType::HighPass,
            2 => FilterType::BandPass,
            _ => FilterType::LowPass,
        };

        // Recreate filters with new parameters (coefficients recomputed each frame)
        self.filter_l = BiquadFilter::design(filter_type, cutoff, self.sample_rate, q, 0.0);
        self.filter_r = BiquadFilter::design(filter_type, cutoff, self.sample_rate, q, 0.0);

        // Process audio buffer
        let buffer = ctx.buffer_mut();

        if buffer.channels() >= 2 {
            // Stereo processing
            for i in 0..buffer.samples() {
                let sample_l = buffer.channel(0)[i];
                let sample_r = buffer.channel(1)[i];

                buffer.channel_mut(0)[i] = self.filter_l.process(sample_l);
                buffer.channel_mut(1)[i] = self.filter_r.process(sample_r);
            }
        } else if buffer.channels() == 1 {
            // Mono processing
            for i in 0..buffer.samples() {
                let sample = buffer.channel(0)[i];
                buffer.channel_mut(0)[i] = self.filter_l.process(sample);
            }
        }

        ProcessStatus::Normal
    }

    // ── Optional: Editor (GUI) ──────────────────────────────────────────────────
    fn has_editor() -> bool {
        false  // Example doesn't implement GUI
    }

    // ── Optional: Preset State ──────────────────────────────────────────────────
    fn save_state(&self) -> Vec<u8> {
        // In a real plugin, serialize to bytes:
        // cutoff as f32 (4 bytes) + resonance as f32 (4 bytes) + mode as u8 (1 byte)
        let mut state = Vec::new();
        state.extend_from_slice(&self.cutoff.value().to_le_bytes());
        state.extend_from_slice(&self.resonance.value().to_le_bytes());
        state.push(self.mode.value() as u8);
        state
    }

    fn load_state(&mut self, data: &[u8]) {
        if data.len() >= 9 {
            if let Ok(cutoff_bytes) = TryInto::<[u8; 4]>::try_into(&data[0..4]) {
                self.cutoff.set(f32::from_le_bytes(cutoff_bytes));
            }
            if let Ok(res_bytes) = TryInto::<[u8; 4]>::try_into(&data[4..8]) {
                self.resonance.set(f32::from_le_bytes(res_bytes));
            }
            if let Ok(mode_byte) = TryInto::<[u8; 1]>::try_into(&data[8..9]) {
                self.mode.set(mode_byte[0] as f32);
            }
        }
    }
}

// Export this plugin to all supported formats (VST3, CLAP, AU, AAX)
smoothie_export!(ProFilter);
