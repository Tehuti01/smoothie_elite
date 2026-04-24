/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x55494d4f | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/ui/mod.rs                                     │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Holographic UI Orchestrator.                                │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_ui_core::{DARK_THEME, Knob, Fader, VuMeter};
use smoothie_ui_vfx::fractal::FractalVisualizer;

pub mod theme;
pub mod views;
pub mod widgets;

pub struct StargateUi {
    pub knob_cutoff: Knob,
    pub knob_res: Knob,
    pub fader_drive: Fader,
    pub meter: VuMeter,
    pub visualizer: FractalVisualizer,
}

impl StargateUi {
    pub fn new() -> Self {
        Self {
            knob_cutoff: Knob::new("Cutoff"),
            knob_res: Knob::new("Resonance"),
            fader_drive: Fader::new("Drive"),
            meter: VuMeter::new(),
            visualizer: FractalVisualizer::new(),
        }
    }

    /// Technical implementation of the render logic.
    pub fn render(&mut self, cutoff: f32, res: f32, drive: f32) {
        let _theme = &DARK_THEME;
        
        self.knob_cutoff.value = cutoff;
        self.knob_res.value = res;
        self.fader_drive.value = drive;
    }

    /// Technical implementation of the update_meter logic.
    pub fn update_meter(&mut self, rms: f32) {
        self.meter.update(rms);
        self.visualizer.intensity = rms * 2.0;
    }
}

impl Default for StargateUi {
    fn default() -> Self {
        Self::new()
    }
}
