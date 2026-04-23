use egui::{Color32, Frame, Margin, Rounding, Stroke, Ui};

pub struct GlassPanel {
    pub rounding: f32,
    pub blur: f32,
    pub tint: Color32,
}

impl Default for GlassPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl GlassPanel {
    pub fn new() -> Self {
        Self {
            rounding: 12.0,
            blur: 10.0,
            tint: Color32::from_rgba_premultiplied(20, 25, 30, 180), // Deep translucent dark
        }
    }

    pub fn show<R>(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        let frame = Frame {
            inner_margin: Margin::same(24.0),
            rounding: Rounding::same(self.rounding),
            fill: Color32::from_rgba_premultiplied(5, 10, 15, 220), // Hyper-realistic dark glass
            stroke: Stroke::new(1.5, Color32::from_rgba_premultiplied(255, 255, 255, 60)), // Sharp specular top edge
            shadow: egui::epaint::Shadow {
                extrusion: 40.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 180), // Deep ambient drop shadow
            },
            ..Default::default()
        };

        let inner_response = frame.show(ui, add_contents);

        // Post-render "Frost/Shine" overlay
        let rect = inner_response.response.rect;
        let painter = ui.painter();

        // Subtle top-down specular gradient (using simple lines for a mock gradient)
        painter.line_segment(
            [
                rect.left_top() + egui::vec2(self.rounding, 1.0),
                rect.right_top() + egui::vec2(-self.rounding, 1.0),
            ],
            Stroke::new(2.0, Color32::from_rgba_premultiplied(255, 255, 255, 80)),
        );

        // Inner glow reflection
        painter.rect_stroke(
            rect.shrink(1.0),
            self.rounding - 1.0,
            Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 200, 255, 15)),
        );

        inner_response.inner
    }
}
