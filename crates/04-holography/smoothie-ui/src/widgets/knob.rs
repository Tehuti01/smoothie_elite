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

use egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct HyperKnob<'a> {
    pub value: &'a mut f32,
    pub min_value: f32,
    pub max_value: f32,
    pub size: f32,
}

impl<'a> HyperKnob<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self {
            value,
            min_value: 0.0,
            max_value: 1.0,
            size: 60.0,
        }
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }
}

impl<'a> Widget for HyperKnob<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

        if response.dragged() {
            let delta = -response.drag_delta().y * 0.005;
            let range = self.max_value - self.min_value;
            let current_norm = (*self.value - self.min_value) / range;
            let next_norm = (current_norm + delta).clamp(0.0, 1.0);
            *self.value = self.min_value + next_norm * range;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = self.size * 0.45;
            let painter = ui.painter();

            // 1. Draw Drop Shadow (Ambient Occlusion)
            painter.circle_filled(
                center + Vec2::new(0.0, 5.0),
                radius,
                Color32::from_black_alpha(150),
            );

            // 2. Draw Base Body (Gradient simulation using nested circles)
            let is_hovered = response.hovered();
            let base_col = if is_hovered { 60 } else { 40 };
            painter.circle_filled(center, radius, Color32::from_gray(base_col));
            painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::from_gray(80)));

            // 3. Draw Inner Bevel / Reflection
            painter.circle_stroke(
                center,
                radius - 2.0,
                Stroke::new(1.0, Color32::from_white_alpha(30)),
            );

            // 4. Draw Indicator Line (3D indent)
            let range = self.max_value - self.min_value;
            let norm = (*self.value - self.min_value) / range;
            let start_angle = 225.0_f32.to_radians();
            let end_angle = -45.0_f32.to_radians();
            let angle = start_angle + norm * (end_angle - start_angle);

            let indicator_len = radius * 0.6;
            let indicator_start = center + Vec2::new(angle.cos(), angle.sin()) * (radius * 0.2);
            let indicator_end = center + Vec2::new(angle.cos(), angle.sin()) * indicator_len;

            // Indent shadow
            painter.line_segment(
                [
                    indicator_start + Vec2::new(0.0, 1.0),
                    indicator_end + Vec2::new(0.0, 1.0),
                ],
                Stroke::new(3.0, Color32::BLACK),
            );
            // Indent highlight (LED/Metal)
            let accent = if is_hovered {
                Color32::from_rgb(0, 255, 200)
            } else {
                Color32::from_rgb(0, 150, 100)
            };

            // Neon Glow effect for the indicator
            if is_hovered {
                painter.line_segment(
                    [indicator_start, indicator_end],
                    Stroke::new(6.0, Color32::from_rgba_premultiplied(0, 255, 200, 50)),
                );
            }

            painter.line_segment([indicator_start, indicator_end], Stroke::new(2.0, accent));

            // Outer Reactive Glow Ring
            let ring_radius = radius + 6.0;
            let mut points = vec![];
            for i in 0..=30 {
                let t = i as f32 / 30.0;
                let a = start_angle + t * (end_angle - start_angle);
                points.push(center + Vec2::new(a.cos(), a.sin()) * ring_radius);

                if t <= norm {
                    painter.circle_filled(
                        center + Vec2::new(a.cos(), a.sin()) * ring_radius,
                        2.0,
                        if is_hovered {
                            Color32::from_rgb(0, 255, 200)
                        } else {
                            Color32::from_rgb(0, 150, 100)
                        },
                    );
                } else {
                    painter.circle_filled(
                        center + Vec2::new(a.cos(), a.sin()) * ring_radius,
                        1.5,
                        Color32::from_white_alpha(20),
                    );
                }
            }
        }

        response
    }
}

pub struct Knob {
    pub value: f32,
    pub min_value: f32,
    pub max_value: f32,
}

impl Knob {
    pub fn rotary() -> Self {
        Self {
            value: 0.0,
            min_value: 0.0,
            max_value: 1.0,
        }
    }

    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_value = min;
        self.max_value = max;
        self
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn draw(&self, _rect: crate::geometry::Rect) {
        // Non-egui draw
    }
}

impl crate::widgets::Widget for Knob {
    fn draw(&self, rect: crate::geometry::Rect) {
        self.draw(rect);
    }

    fn on_mouse_drag(&mut self, _dx: f32, dy: f32) {
        let delta = -dy * 0.01;
        let range = self.max_value - self.min_value;
        let current_norm = (self.value - self.min_value) / range;
        let next_norm = (current_norm + delta).clamp(0.0, 1.0);
        self.value = self.min_value + next_norm * range;
    }
}
