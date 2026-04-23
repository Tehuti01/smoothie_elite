/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4443424c | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/filters/dc_blocker.rs                 │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: DC Offset removal filter.                                   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// High-pass filter designed to remove DC offset from audio signals.
#[derive(Debug, Clone, Copy)]
pub struct DcBlocker {
    x1: f32,
    y1: f32,
    coeff: f32,
}

impl DcBlocker {
    /// Initializes a new instance with a pole position (typically 0.995).
    pub fn new() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            coeff: 0.995,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: f32) -> f32 {
        let output = input - self.x1 + self.coeff * self.y1;
        self.x1 = input;
        self.y1 = output;
        output
    }

    /// Resets the internal state of the filter.
    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.y1 = 0.0;
    }
}

impl Default for DcBlocker {
    fn default() -> Self {
        Self::new()
    }
}
