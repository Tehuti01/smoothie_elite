/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x57696467 | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-core/src/widgets.rs               │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Premium Holographic UI components.                          │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// Base trait for all Holographic UI widgets.
pub trait HolographicWidget {
    fn render(&self);
    fn handle_input(&mut self, x: f32, y: f32);
}

/// A premium rotational control with SDF-based glow.
pub struct Knob {
    pub value: f32,
    pub label: &'static str,
}

impl Knob {
    pub fn new(label: &'static str) -> Self {
        Self { value: 0.5, label }
    }
}

/// A linear slider with glass-morphic styling.
pub struct Fader {
    pub value: f32,
    pub label: &'static str,
}

impl Fader {
    pub fn new(label: &'static str) -> Self {
        Self { value: 0.0, label }
    }
}

/// High-precision VU meter with peak hold.
pub struct VuMeter {
    pub level: f32,
    pub peak: f32,
}

impl VuMeter {
    pub fn new() -> Self {
        Self { level: 0.0, peak: 0.0 }
    }

    pub fn update(&mut self, new_level: f32) {
        self.level = new_level;
        if new_level > self.peak {
            self.peak = new_level;
        } else {
            self.peak *= 0.999; // Peak decay
        }
    }
}
