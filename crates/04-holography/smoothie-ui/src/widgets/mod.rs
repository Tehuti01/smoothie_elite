/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe9691db0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

pub mod builder;
pub mod button;
pub mod fader;
pub mod glass;
pub mod immediate;
pub mod knob;
pub mod layout;
pub mod metering;
pub mod oscilloscope;
pub mod spectrum;
pub mod text;

pub use builder::*;
pub use button::*;
pub use fader::*;
pub use glass::*;
pub use immediate::*;
pub use knob::*;
pub use layout::*;
pub use metering::*;
pub use oscilloscope::*;
pub use spectrum::*;
pub use text::*;

use crate::geometry::Rect;

pub trait Widget {
    /// Technical implementation of the draw logic.
    fn draw(&self, rect: Rect);
    /// Technical implementation of the on_mouse_drag logic.
    fn on_mouse_drag(&mut self, dx: f32, dy: f32);
}
