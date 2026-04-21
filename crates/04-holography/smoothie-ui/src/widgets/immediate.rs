/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcb148fc1 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/immediate.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use crate::geometry::Rect;
use crate::widgets::{Knob, Widget};
use alloc::vec::Vec;

pub trait ImmediateRenderer {
    /// Technical implementation of the draw_rect logic.
    fn draw_rect(&mut self, rect: Rect, color: u32);
    /// Technical implementation of the draw_text logic.
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32);
    /// Technical implementation of the draw_line logic.
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32, color: u32);
    /// Technical implementation of the draw_circle logic.
    fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: u32);
}

/// Technical implementation of the ImmediateModeContext structure.
pub struct ImmediateModeContext {
    pub rect: Rect,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: bool,
    pub frame_count: u64,
}

impl ImmediateModeContext {
    /// Initializes a new instance of the associated type.
    pub const fn new(rect: Rect) -> Self {
        Self {
            rect,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_down: false,
            frame_count: 0,
        }
    }

    /// Technical implementation of the contains logic.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.rect.contains(x, y)
    }
}

/// Technical implementation of the ImmediateUI structure.
pub struct ImmediateUI<R: ImmediateRenderer> {
    renderer: R,
    pub context: ImmediateModeContext,
}

impl<R: ImmediateRenderer> ImmediateUI<R> {
    /// Initializes a new instance of the associated type.
    pub fn new(renderer: R, rect: Rect) -> Self {
        Self {
            renderer,
            context: ImmediateModeContext::new(rect),
        }
    }

    /// Technical implementation of the update_mouse logic.
    pub fn update_mouse(&mut self, x: f32, y: f32, down: bool) {
        self.context.mouse_x = x;
        self.context.mouse_y = y;
        self.context.mouse_down = down;
    }

    /// Technical implementation of the next_frame logic.
    pub fn next_frame(&mut self) {
        self.context.frame_count += 1;
        self.context.mouse_down = false;
    }

    /// Technical implementation of the button logic.
    pub fn button(&mut self, label: &str) -> bool {
        let is_hovered = self
            .context
            .rect
            .contains(self.context.mouse_x, self.context.mouse_y);
        let color = if is_hovered { 0xFF4444FF } else { 0xFF222244 };
        self.renderer.draw_rect(self.context.rect, color);
        self.renderer.draw_text(
            label,
            self.context.rect.x + 8.0,
            self.context.rect.y + 16.0,
            12.0,
        );
        is_hovered && self.context.mouse_down
    }
}

pub const MAX_RETAINED_WIDGETS: usize = 256;

/// Technical implementation of the RetainedWidget enumeration.
pub enum RetainedWidget {
    Knob(Knob),
}

/// Technical implementation of the RetainedUI structure.
pub struct RetainedUI {
    children: Vec<RetainedWidget>,
    layout_dirty: bool,
}

impl RetainedUI {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout_dirty: true,
        }
    }

    /// Performs vector addition logic.
    pub fn add_knob(&mut self, knob: Knob) {
        self.children.push(RetainedWidget::Knob(knob));
        self.layout_dirty = true;
    }

    /// Technical implementation of the remove logic.
    pub fn remove(&mut self, index: usize) {
        if index < self.children.len() {
            self.children.remove(index);
            self.layout_dirty = true;
        }
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, rect: Rect) {
        if self.layout_dirty {
            self.compute_layout(rect);
            self.layout_dirty = false;
        }
    }

    /// Technical implementation of the compute_layout logic.
    fn compute_layout(&mut self, _rect: Rect) {}

    /// Technical implementation of the draw logic.
    pub fn draw(&self) {
        for child in &self.children {
            match child {
                RetainedWidget::Knob(k) => k.draw(Rect::default()),
            }
        }
    }

    /// Technical implementation of the handle_mouse logic.
    pub fn handle_mouse(&mut self, _x: f32, _y: f32, dx: f32, dy: f32) {
        for child in &mut self.children.iter_mut() {
            match child {
                RetainedWidget::Knob(k) => {
                    k.on_mouse_drag(dx, dy);
                }
            }
        }
    }
}

impl Default for RetainedUI {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_immediate_context logic.
    fn test_immediate_context() {
        let ctx = ImmediateModeContext::new(Rect::default());
        assert!(!ctx.contains(0.0, 0.0));
    }

    #[test]
    /// Technical implementation of the test_retained_ui logic.
    fn test_retained_ui() {
        let ui = RetainedUI::new();
        assert!(ui.children.is_empty());
    }
}
