/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9a3c2b1d | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui/src/ironstack_hologram.rs         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: UI Manifest and Bridge for the IRONSTACK-100 instrument.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Premium aesthetics and PHI-aligned control mapping.     │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::widgets::{HyperKnob, SeraphicUi};
use egui::Ui;
use smoothie_params::bank::ParameterBank;
use smoothie_ui_vfx::fractal::FractalVisualizer;
use smoothie_ui_vfx::particles::ParticleEmitter;

pub struct IronStackHologram {
    pub drive_val: f32,
    pub bias_val: f32,
    pub out_val: f32,
    pub neural_drive_val: f32,
    pub neural_mix_val: f32,
    pub reverb_mix_val: f32,
    pub reverb_time_val: f32,
    pub reverb_size_val: f32,
    pub fractal: FractalVisualizer,
    pub emitter: ParticleEmitter,
}

impl IronStackHologram {
    pub fn new() -> Self {
        Self {
            drive_val: 1.0,
            bias_val: -5.0,
            out_val: 0.8,
            neural_drive_val: 1.0,
            neural_mix_val: 0.5,
            reverb_mix_val: 0.2,
            reverb_time_val: 2.0,
            reverb_size_val: 1.5,
            fractal: FractalVisualizer::new(),
            emitter: ParticleEmitter::new(1000),
        }
    }

    pub fn push_audio(&mut self, audio: &[f32]) {
        self.fractal.audio_buffer.clear();
        let max_len = 256;
        let take_len = audio.len().min(max_len);
        self.fractal
            .audio_buffer
            .extend_from_slice(&audio[..take_len]);
    }

    pub fn sync_from_bank(&mut self, bank: &ParameterBank) {
        if let Some(p) = bank.get_by_name("Tube Drive") {
            self.drive_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Plate Bias") {
            self.bias_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Master Out") {
            self.out_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Neural Drive") {
            self.neural_drive_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Neural Mix") {
            self.neural_mix_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Reverb Mix") {
            self.reverb_mix_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Reverb Time") {
            self.reverb_time_val = p.atomic.load();
        }
        if let Some(p) = bank.get_by_name("Reverb Size") {
            self.reverb_size_val = p.atomic.load();
        }
    }

    pub fn sync_to_bank(&self, bank: &ParameterBank) {
        if let Some(p) = bank.get_by_name("Tube Drive") {
            p.atomic.store(self.drive_val);
        }
        if let Some(p) = bank.get_by_name("Plate Bias") {
            p.atomic.store(self.bias_val);
        }
        if let Some(p) = bank.get_by_name("Master Out") {
            p.atomic.store(self.out_val);
        }
        if let Some(p) = bank.get_by_name("Neural Drive") {
            p.atomic.store(self.neural_drive_val);
        }
        if let Some(p) = bank.get_by_name("Neural Mix") {
            p.atomic.store(self.neural_mix_val);
        }
        if let Some(p) = bank.get_by_name("Reverb Mix") {
            p.atomic.store(self.reverb_mix_val);
        }
        if let Some(p) = bank.get_by_name("Reverb Time") {
            p.atomic.store(self.reverb_time_val);
        }
        if let Some(p) = bank.get_by_name("Reverb Size") {
            p.atomic.store(self.reverb_size_val);
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let screen_rect = ui.ctx().screen_rect();

        egui::Area::new(egui::Id::new("fractal_bg"))
            .order(egui::Order::Background)
            .fixed_pos(screen_rect.min)
            .show(ui.ctx(), |ui| {
                self.fractal.draw(ui, screen_rect.size());
            });

        // Update and draw particles
        self.emitter.update_and_draw(ui, ui.input(|i| i.stable_dt));

        // Use the new Seraphic Declarative UI Builder
        let mut s_ui = SeraphicUi::new(ui);

        s_ui.glass(|ui| {
            ui.col(|ui| {
                ui.egui().heading("IRONSTACK-100 HOLOGRAPHIC SURFACE");
                ui.space(20.0);

                ui.row(|ui| {
                    ui.egui().group(|ui| {
                        ui.label("Core Synthesis");
                        ui.horizontal(|ui| {
                            ui.add(HyperKnob::new(&mut self.drive_val).with_range(0.0, 2.0));
                            ui.add(HyperKnob::new(&mut self.bias_val).with_range(-10.0, 0.0));
                            ui.add(HyperKnob::new(&mut self.out_val).with_range(0.0, 1.0));
                        });
                    });

                    ui.egui().group(|ui| {
                        ui.label("Neural Link");
                        ui.horizontal(|ui| {
                            ui.add(HyperKnob::new(&mut self.neural_drive_val).with_range(0.0, 2.0));
                            ui.add(HyperKnob::new(&mut self.neural_mix_val).with_range(0.0, 1.0));
                        });
                    });
                });

                ui.space(20.0);

                ui.row(|ui| {
                    ui.egui().group(|ui| {
                        ui.label("Quantum Reverb");
                        ui.horizontal(|ui| {
                            ui.add(HyperKnob::new(&mut self.reverb_mix_val).with_range(0.0, 1.0));
                            ui.add(HyperKnob::new(&mut self.reverb_time_val).with_range(0.1, 20.0));
                            ui.add(HyperKnob::new(&mut self.reverb_size_val).with_range(0.5, 5.0));
                        });
                    });
                });
            });
        });
    }
}

impl Default for IronStackHologram {
    fn default() -> Self {
        Self::new()
    }
}
