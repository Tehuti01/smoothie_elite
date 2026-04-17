//! A professional multi-mode filter plugin.
//!
//! Demonstrates:
//! - Zero-Delay Feedback (ZDF) filters
//! - Parameter automation
//! - Real-time safe DSP

use smoothie_core::prelude::*;
use smoothie_dsp::filters::ZdfFilter;
use smoothie_params::{FloatParam, EnumParam, Param};
use std::sync::Arc;

/// Professional multi-mode filter plugin using ZDF technology.
pub struct ProFilter {
    // DSP
    filter_l: ZdfFilter,
    filter_r: ZdfFilter,

    // Parameters (real-time safe, atomic updates)
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    mode: Arc<EnumParam>,
}

impl Default for ProFilter {
    fn default() -> Self {
        Self {
            filter_l: ZdfFilter::new(),
            filter_r: ZdfFilter::new(),
            cutoff: Arc::new(FloatParam::new("Cutoff", 2000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::new("Resonance", 1.0, 0.1, 10.0)),
            mode: Arc::new(EnumParam::new("Mode", 0, &["Lowpass", "Highpass", "Bandpass"])),
        }
    }
}

impl SmoothiePlugin for ProFilter {
    // ── Identity ────────────────────────────────────────────────────────────────
    const NAME: &'static str = "Pro Filter";
    const VENDOR: &'static str = "Smoothie Sonic";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid(*b"PFLT0001");
    const URL: &'static str = "https://seraphicsonic.com/smoothie";
    const EMAIL: &'static str = "plugins@seraphicsonic.com";

    // ── Audio I/O ───────────────────────────────────────────────────────────────
    fn audio_layouts() -> &'static [AudioLayout] {
        &[
            AudioLayout::stereo(),
            AudioLayout::mono(),
        ]
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
        self.filter_l.set_params(2000.0, 1.0, sr);
        self.filter_r.set_params(2000.0, 1.0, sr);
        true
    }

    fn reset(&mut self) {
        self.filter_l.reset();
        self.filter_r.reset();
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let cutoff = self.cutoff.value();
        let resonance = self.resonance.value();
        let mode = self.mode.value();

        // Update filter coefficients (cheap operation, safe to do per-frame)
        self.filter_l.set_params(cutoff, resonance, ctx.buffer().sample_rate());
        self.filter_r.set_params(cutoff, resonance, ctx.buffer().sample_rate());

        // Process audio buffer
        let buffer = ctx.buffer_mut();
        let samples = buffer.samples();

        if buffer.channels() >= 2 {
            // Stereo processing
            let ch_l = buffer.channel_mut(0);
            for sample in ch_l.iter_mut() {
                *sample = match mode {
                    0 => self.filter_l.process_lowpass(*sample),
                    1 => self.filter_l.process_highpass(*sample),
                    2 => self.filter_l.process_bandpass(*sample),
                    _ => *sample,
                };
            }

            let ch_r = buffer.channel_mut(1);
            for sample in ch_r.iter_mut() {
                *sample = match mode {
                    0 => self.filter_r.process_lowpass(*sample),
                    1 => self.filter_r.process_highpass(*sample),
                    2 => self.filter_r.process_bandpass(*sample),
                    _ => *sample,
                };
            }
        } else if buffer.channels() == 1 {
            // Mono processing
            let ch = buffer.channel_mut(0);
            for sample in ch.iter_mut() {
                *sample = match mode {
                    0 => self.filter_l.process_lowpass(*sample),
                    1 => self.filter_l.process_highpass(*sample),
                    2 => self.filter_l.process_bandpass(*sample),
                    _ => *sample,
                };
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
                self.mode.set(mode_byte[0] as i32);
            }
        }
    }
}

// Export this plugin to all supported formats (VST3, CLAP, AU, AAX)
smoothie_export!(ProFilter);
