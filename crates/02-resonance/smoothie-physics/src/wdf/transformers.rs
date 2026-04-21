/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcbba1df9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/transformers.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the TransformerCore structure.
pub struct TransformerCore<'a, Primary: WdfNode, Secondary: WdfNode> {
    pub primary: &'a mut Primary,
    pub secondary: &'a mut Secondary,

    ratio: f32,
    saturation: f32, // Magnetic flux saturation point
    flux_state: f32,
}

impl<'a, Primary: WdfNode, Secondary: WdfNode> TransformerCore<'a, Primary, Secondary> {
    /// Initializes a new instance of the associated type.
    pub fn new(primary: &'a mut Primary, secondary: &'a mut Secondary, ratio: f32) -> Self {
        Self {
            primary,
            secondary,
            ratio,
            saturation: 1.5,
            flux_state: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let p_up = self.primary.wave_up();
        let s_up = self.secondary.wave_up();

        let rp = self.primary.get_port_resistance();
        let rs = self.secondary.get_port_resistance();

        // Reflected resistance
        let rs_ref = rs / (self.ratio * self.ratio);

        // Core physics (Jiles-Atherton placeholder approach via simplistic arctangent saturation)
        let total_drive = p_up + s_up * self.ratio;

        let flux = self.soft_clip(total_drive);
        self.flux_state = flux;

        // Wave resolutions
        let p_down = flux - (p_up * rs_ref / (rp + rs_ref));
        let s_down = (flux / self.ratio) - (s_up * rp / (rp + rs_ref));

        self.primary.wave_down(p_down);
        self.secondary.wave_down(s_down);
    }

    /// Fast soft clipping representing B-H magnetic curve
    #[inline(always)]
    /// Technical implementation of the soft_clip logic.
    fn soft_clip(&self, x: f32) -> f32 {
        let max = self.saturation;
        if x > max {
            max
        } else if x < -max {
            -max
        } else {
            x - (x * x * x) / (3.0 * max * max)
        }
    }
}
