/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x63ab105e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/geometry.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the Rect structure.
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Creates a zero-initialized Rect instance.
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }

    /// Technical implementation of the contains logic.
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }

    /// Technical implementation of the split_vertical logic.
    pub fn split_vertical(&self, ratio: f32) -> (Rect, Rect) {
        let top_h = self.height * ratio.clamp(0.0, 1.0);
        let bottom_h = self.height - top_h;
        (
            Rect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: top_h,
            },
            Rect {
                x: self.x,
                y: self.y + top_h,
                width: self.width,
                height: bottom_h,
            },
        )
    }

    /// Technical implementation of the split_horizontal logic.
    pub fn split_horizontal(&self, ratio: f32) -> (Rect, Rect) {
        let left_w = self.width * ratio.clamp(0.0, 1.0);
        let right_w = self.width - left_w;
        (
            Rect {
                x: self.x,
                y: self.y,
                width: left_w,
                height: self.height,
            },
            Rect {
                x: self.x + left_w,
                y: self.y,
                width: right_w,
                height: self.height,
            },
        )
    }
}

impl Default for Rect {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::zero()
    }
}
