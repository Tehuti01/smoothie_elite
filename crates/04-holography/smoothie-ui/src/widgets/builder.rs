/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4a5d6e7f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/builder.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: High-level declarative UI builder API for Seraphic.         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Inspired by SwiftUI and React, optimized for egui.      │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use crate::widgets::GlassPanel;
use egui::Ui;

pub struct SeraphicUi<'a> {
    ui: &'a mut Ui,
}

impl<'a> SeraphicUi<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        Self { ui }
    }

    /// Creates a glass-morphic container.
    pub fn glass<R>(&mut self, add_contents: impl FnOnce(&mut SeraphicUi) -> R) -> R {
        let glass = GlassPanel::new();
        glass.show(self.ui, |ui| {
            let mut seraphic_ui = SeraphicUi::new(ui);
            add_contents(&mut seraphic_ui)
        })
    }

    /// Horizontal stack (row).
    pub fn row<R>(&mut self, add_contents: impl FnOnce(&mut SeraphicUi) -> R) -> R {
        self.ui
            .horizontal(|ui| {
                let mut seraphic_ui = SeraphicUi::new(ui);
                add_contents(&mut seraphic_ui)
            })
            .inner
    }

    /// Vertical stack (column).
    pub fn col<R>(&mut self, add_contents: impl FnOnce(&mut SeraphicUi) -> R) -> R {
        self.ui
            .vertical(|ui| {
                let mut seraphic_ui = SeraphicUi::new(ui);
                add_contents(&mut seraphic_ui)
            })
            .inner
    }

    /// Adds space.
    pub fn space(&mut self, amount: f32) {
        self.ui.add_space(amount);
    }

    /// Returns the underlying egui Ui.
    pub fn egui(&mut self) -> &mut Ui {
        self.ui
    }
}
