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

use crate::widgets::knob::Knob;
use crate::geometry::Rect;
use smoothie_params::bank::ParameterBank;

/// Technical implementation of the IronStackHologram structure.
/// Orchestrates the visual control surface for the IRONSTACK-100 instrument.
pub struct IronStackHologram {
    pub drive_knob: Knob,
    pub bias_knob: Knob,
    pub out_knob: Knob,
    
    // Phase X: Neural Controls
    pub neural_drive_knob: Knob,
    pub neural_mix_knob: Knob,

    // Phase XI: Reverb Controls
    pub reverb_mix_knob: Knob,
    pub reverb_time_knob: Knob,
    pub reverb_size_knob: Knob,
}

impl IronStackHologram {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            drive_knob: Knob::rotary().with_range(0.0, 2.0),
            bias_knob: Knob::rotary().with_range(-10.0, 0.0),
            out_knob: Knob::rotary().with_range(0.0, 1.0),
            neural_drive_knob: Knob::rotary().with_range(0.0, 2.0),
            neural_mix_knob: Knob::rotary().with_range(0.0, 1.0),
            reverb_mix_knob: Knob::rotary().with_range(0.0, 1.0),
            reverb_time_knob: Knob::rotary().with_range(0.1, 20.0),
            reverb_size_knob: Knob::rotary().with_range(0.5, 5.0),
        }
    }

    /// Synchronizes the UI widgets from the given parameter bank.
    pub fn sync_from_bank(&mut self, bank: &ParameterBank) {
        if let Some(p) = bank.get_by_name("Tube Drive") { self.drive_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Plate Bias") { self.bias_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Master Out") { self.out_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Neural Drive") { self.neural_drive_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Neural Mix") { self.neural_mix_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Reverb Mix") { self.reverb_mix_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Reverb Time") { self.reverb_time_knob.set_value(p.atomic.load()); }
        if let Some(p) = bank.get_by_name("Reverb Size") { self.reverb_size_knob.set_value(p.atomic.load()); }
    }

    /// Synchronizes the parameter bank from the UI widgets.
    pub fn sync_to_bank(&self, bank: &ParameterBank) {
        if let Some(p) = bank.get_by_name("Tube Drive") { p.atomic.store(self.drive_knob.value()); }
        if let Some(p) = bank.get_by_name("Plate Bias") { p.atomic.store(self.bias_knob.value()); }
        if let Some(p) = bank.get_by_name("Master Out") { p.atomic.store(self.out_knob.value()); }
        if let Some(p) = bank.get_by_name("Neural Drive") { p.atomic.store(self.neural_drive_knob.value()); }
        if let Some(p) = bank.get_by_name("Neural Mix") { p.atomic.store(self.neural_mix_knob.value()); }
        if let Some(p) = bank.get_by_name("Reverb Mix") { p.atomic.store(self.reverb_mix_knob.value()); }
        if let Some(p) = bank.get_by_name("Reverb Time") { p.atomic.store(self.reverb_time_knob.value()); }
        if let Some(p) = bank.get_by_name("Reverb Size") { p.atomic.store(self.reverb_size_knob.value()); }
    }

    /// Technical implementation of the draw logic.
    pub fn draw(&self, screen_rect: Rect) {
        let margin = 20.0;
        let knob_size = 60.0; // Compact size for multi-stage instrumentation
        
        // Row 1: Core Synthesis (Tube + Neural)
        let mut x = screen_rect.x + margin;
        let y1 = screen_rect.y + margin;
        
        let row1 = [&self.drive_knob, &self.bias_knob, &self.out_knob, &self.neural_drive_knob, &self.neural_mix_knob];
        for knob in row1 {
            knob.draw(Rect { x, y: y1, width: knob_size, height: knob_size });
            x += knob_size + margin;
        }

        // Row 2: Spatial Resonance (Quantum Reverb)
        let mut x2 = screen_rect.x + margin;
        let y2 = screen_rect.y + margin + knob_size + margin;
        
        let row2 = [&self.reverb_mix_knob, &self.reverb_time_knob, &self.reverb_size_knob];
        for knob in row2 {
            knob.draw(Rect { x: x2, y: y2, width: knob_size, height: knob_size });
            x2 += knob_size + margin;
        }
    }
}

impl Default for IronStackHologram {
    fn default() -> Self {
        Self::new()
    }
}
