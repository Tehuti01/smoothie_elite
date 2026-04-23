use egui::{Color32, Response, Sense, Shape, Ui, Vec2};

pub struct FractalVisualizer {
    pub audio_buffer: Vec<f32>,
    pub intensity: f32,
    pub complexity: usize,
}

impl Default for FractalVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl FractalVisualizer {
    pub fn new() -> Self {
        Self {
            audio_buffer: vec![0.0; 256],
            intensity: 1.0,
            complexity: 50,
        }
    }

    pub fn with_audio(mut self, buffer: &[f32]) -> Self {
        self.audio_buffer = buffer.to_vec();
        self
    }

    pub fn draw(&self, ui: &mut Ui, size: Vec2) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let time = ui.input(|i| i.time);

            // In a true hyper-realistic setup, this would dispatch to a WGPU compute shader.
            // For now, we simulate the fractal geometry using egui shapes driven by the audio buffer.

            let center = rect.center();
            let mut shapes = vec![];

            let points = self.audio_buffer.len();
            let angle_step = std::f32::consts::TAU / points as f32;

            for i in 0..points {
                let sample = self.audio_buffer[i] * self.intensity;
                let angle = i as f32 * angle_step + (time as f32 * 0.5);

                // Simulate Mandelbrot orbit expansion based on audio
                let radius = 50.0 + (sample * 100.0) + (self.complexity as f32 * 0.5);

                let x = center.x + angle.cos() * radius;
                let y = center.y + angle.sin() * radius;

                let color: Color32 = egui::epaint::Hsva::new(
                    (i as f32 / points as f32 + time as f32 * 0.1).fract(),
                    0.8,
                    0.9 + sample.abs() * 0.1,
                    0.8,
                )
                .into();

                shapes.push(Shape::circle_filled(
                    egui::pos2(x, y),
                    2.0 + sample.abs() * 5.0,
                    color,
                ));
            }

            painter.extend(shapes);
        }

        response
    }
}
