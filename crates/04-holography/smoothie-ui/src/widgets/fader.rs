/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x24a6a311 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/fader.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::super::geometry::Rect;
use super::Widget;
use smoothie_params::AtomicParameter;

/// Technical implementation of the Fader structure.
pub struct Fader<'a> {
    pub parameter: &'a AtomicParameter,
    pub vertical: bool,
}

impl<'a> Fader<'a> {
    /// Initializes a new instance of the associated type.
    pub fn new(parameter: &'a AtomicParameter, vertical: bool) -> Self {
        Self {
            parameter,
            vertical,
        }
    }
}

impl<'a> Widget for Fader<'a> {
    /// Technical implementation of the draw logic.
    fn draw(&self, _rect: Rect) {
        // GPU vector drawing logic
    }

    /// Technical implementation of the on_mouse_drag logic.
    fn on_mouse_drag(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.002;
        let mut p = self.parameter.get_normalized();
        if self.vertical {
            p -= dy * sensitivity;
        } else {
            p += dx * sensitivity;
        }
        self.parameter.set_normalized(p.clamp(0.0, 1.0));
    }
}
