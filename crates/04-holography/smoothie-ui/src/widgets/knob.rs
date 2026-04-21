/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb0d43554 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/knob.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::geometry::Rect;
use smoothie_core::math::FloatExt;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the KnobStyle enumeration.
pub enum KnobStyle {
    Rotary,
    Linear,
    Horizontal,
    Vertical,
}

/// Technical implementation of the Knob structure.
pub struct Knob {
    value: f32,
    style: KnobStyle,
    size: f32,
    min_value: f32,
    max_value: f32,
    sensitivity: f32,
}

impl Knob {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            value: 0.5,
            style: KnobStyle::Rotary,
            size: 50.0,
            min_value: 0.0,
            max_value: 1.0,
            sensitivity: 0.005,
        }
    }

    /// Technical implementation of the rotary logic.
    pub const fn rotary() -> Self {
        Self {
            style: KnobStyle::Rotary,
            ..Self::new()
        }
    }

    /// Technical implementation of the linear logic.
    pub const fn linear() -> Self {
        Self {
            style: KnobStyle::Linear,
            ..Self::new()
        }
    }

    /// Technical implementation of the horizontal logic.
    pub const fn horizontal() -> Self {
        Self {
            style: KnobStyle::Horizontal,
            ..Self::new()
        }
    }

    /// Technical implementation of the vertical logic.
    pub const fn vertical() -> Self {
        Self {
            style: KnobStyle::Vertical,
            ..Self::new()
        }
    }

    /// Technical implementation of the with_size logic.
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Technical implementation of the with_range logic.
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }

    /// Technical implementation of the with_sensitivity logic.
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    #[inline]
    /// Technical implementation of the value logic.
    pub fn value(&self) -> f32 {
        self.value
    }

    #[inline]
    /// Returns a unit-length version of the vector.
    pub fn normalized_value(&self) -> f32 {
        (self.value - self.min_value) / (self.max_value - self.min_value)
    }

    /// Technical implementation of the set_value logic.
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min_value, self.max_value);
    }

    /// Technical implementation of the set_normalized logic.
    pub fn set_normalized(&mut self, normalized: f32) {
        self.value =
            self.min_value + normalized.clamp(0.0, 1.0) * (self.max_value - self.min_value);
    }

    /// Technical implementation of the on_mouse_drag logic.
    pub fn on_mouse_drag(&mut self, dx: f32, dy: f32) {
        match self.style {
            KnobStyle::Rotary | KnobStyle::Vertical => {
                let delta = -dy * self.sensitivity;
                self.set_normalized(self.normalized_value() + delta);
            }
            KnobStyle::Linear | KnobStyle::Horizontal => {
                let delta = dx * self.sensitivity;
                self.set_normalized(self.normalized_value() + delta);
            }
        }
    }

    /// Technical implementation of the on_mouse_wheel logic.
    pub fn on_mouse_wheel(&mut self, delta: f32) {
        let step = delta.signum() * 0.01;
        self.set_normalized(self.normalized_value() + step);
    }

    /// Technical implementation of the on_double_click logic.
    pub fn on_double_click(&mut self) {
        self.set_value((self.min_value + self.max_value) * 0.5);
    }

    /// Technical implementation of the draw logic.
    pub fn draw(&self, rect: Rect) {
        match self.style {
            KnobStyle::Rotary => self.draw_rotary(rect),
            KnobStyle::Linear => self.draw_linear(rect),
            KnobStyle::Horizontal => self.draw_horizontal(rect),
            KnobStyle::Vertical => self.draw_vertical(rect),
        }
    }

    /// Technical implementation of the draw_rotary logic.
    fn draw_rotary(&self, rect: Rect) {
        let cx = rect.x + rect.width * 0.5;
        let cy = rect.y + rect.height * 0.5;
        let radius = rect.width.min(rect.height) * 0.4;
        let value = self.normalized_value();
        let start_angle = 225.0_f32.to_radians();
        let end_angle = -45.0_f32.to_radians();
        let angle = start_angle + value * (end_angle - start_angle);
        let _ = (cx, cy, radius, angle);
    }

    /// Technical implementation of the draw_linear logic.
    fn draw_linear(&self, rect: Rect) {
        let value = self.normalized_value();
        let track_h = 4.0;
        let track_y = rect.y + (rect.height - track_h) * 0.5;
        let thumb_x = rect.x + value * rect.width;
        let _ = (track_y, thumb_x);
    }

    /// Technical implementation of the draw_horizontal logic.
    fn draw_horizontal(&self, rect: Rect) {
        self.draw_linear(rect);
    }

    /// Technical implementation of the draw_vertical logic.
    fn draw_vertical(&self, rect: Rect) {
        let value = self.normalized_value();
        let track_w = 4.0;
        let track_x = rect.x + (rect.width - track_w) * 0.5;
        let thumb_y = rect.y + (1.0 - value) * rect.height;
        let _ = (track_x, thumb_y);
    }
}

impl Default for Knob {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_knob_creation logic.
    fn test_knob_creation() {
        let _knob = Knob::new();
        let _rotary = Knob::rotary();
        let _linear = Knob::linear();
    }

    #[test]
    /// Technical implementation of the test_knob_drag logic.
    fn test_knob_drag() {
        let mut knob = Knob::new();
        knob.set_normalized(0.5);
        knob.on_mouse_drag(0.0, 100.0);
        assert!(knob.normalized_value() < 0.5);
    }
}
