# SKILL UI-001: AUDIO PLUGIN UI DESIGN

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        AUDIO PLUGIN UI DESIGN
                     Plugin Interfaces with JUCE/EG-UI
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## CUSTOM KNOB

```rust
pub struct Knob {
    pub size: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub value: f32,
    pub style: KnobStyle,
}

pub enum KnobStyle {
    Analog,
    Digital,
    Minimal,
    Dial,
}

impl Knob {
    pub fn draw(&self, ctx: &mut Graphics, x: f32, y: f32) {
        let angle = self.value / (self.max_value - self.min_value) * 270.0 - 135.0;
        
        // Draw background
        ctx.set_color(Color::dark_gray());
        ctx.fill_arc(x, y, self.size, angle, 270.0);
        
        // Draw value arc
        ctx.set_color(self.color());
        ctx.stroke_arc(x, y, self.size, -135.0, angle);
        
        // Draw indicator
        let rad = angle.to_radians();
        ctx.draw_line(
            x + self.size * 0.5 * rad.cos(),
            y + self.size * 0.5 * rad.sin(),
            x + self.size * 0.8 * rad.cos(),
            y + self.size * 0.8 * rad.sin()
        );
    }
}
```

---

## WAVEFORM DISPLAY

```rust
pub struct WaveformDisplay {
    pub samples: Vec<f32>,
    pub cursor_position: usize,
    pub selection: Option<(usize, usize)>,
}

impl WaveformDisplay {
    pub fn draw(&self, ctx: &mut Graphics, rect: Rect) {
        ctx.set_background(Color::black());
        ctx.fill_rect(rect);
        
        let width = rect.width;
        let height = rect.height;
        let center_y = rect.y + height / 2.0;
        
        ctx.set_color(Color::green());
        
        for (i, &sample) in self.samples.iter().enumerate() {
            let x = rect.x + (i as f32 / self.samples.len() as f32) * width;
            let y = center_y - sample * height * 0.8;
            ctx.fill_rect(Rect::new(x, y, 2.0, 2.0));
        }
        
        // Draw selection
        if let Some((start, end)) = self.selection {
            ctx.set_color(Color::blue().with_alpha(0.3));
            ctx.fill_rect(Rect::new(
                start as f32 / self.samples.len() as f32 * width,
                rect.y,
                (end - start) as f32 / self.samples.len() as f32 * width,
                height,
            ));
        }
    }
}
```

---

## METER DISPLAY

```rust
pub struct MeterDisplay {
    pub level: f32,
    pub peak: f32,
    pub peak_hold: f32,
    pub clip_indicator: bool,
}

impl MeterDisplay {
    pub fn draw(&mut self, ctx: &mut Graphics, rect: Rect) {
        let width = rect.width;
        let height = rect.height;
        
        // Background
        ctx.set_color(Color::dark_gray());
        ctx.fill_rect(rect);
        
        // Level
        let level_height = self.level * height;
        ctx.set_color(self.level_color());
        ctx.fill_rect(Rect::new(
            rect.x, rect.y + height - level_height,
            width, level_height
        ));
        
        // Peak
        let peak_y = height - self.peak * height;
        ctx.fill_rect(Rect::new(rect.x, peak_y, width, 2.0));
        
        // Clip
        if self.clip_indicator {
            ctx.set_color(Color::red());
            ctx.fill_rect(Rect::new(rect.x, rect.y, width, 4.0));
        }
    }
    
    fn level_color(&self) -> Color {
        if self.level > 0.0 {
            Color::green()
        } else if self.level > -6.0 {
            Color::yellow()
        } else if self.level > -12.0 {
            Color::orange()
        } else {
            Color::red()
        }
    }
}
```

---

*Skill UI-001 | Category: UI | Complexity: Expert*