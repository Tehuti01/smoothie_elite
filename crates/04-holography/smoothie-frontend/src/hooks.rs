/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc38dbc4e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-frontend/src/hooks.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the HookParamDef structure.
pub struct HookParamDef {
    /// The unique integer mapping natively bound to the DSP matrix.
    pub param_index: u32,
    pub min_value: f32,
    pub max_value: f32,
    pub default_value: f32,
    /// Non-linear skew factor (e.g., 0.5 for a log curve on a filter frequency).
    pub curve_skew: f32,
    /// Identifier indicating string mapping format (e.g., Enum(0)="Hz", Enum(1)="dB").
    pub format_id: u8,
}

impl HookParamDef {
    /// Deserializes a normalized GUI `[0.0, 1.0]` scalar into the true world value.
    #[inline(always)]
    /// Technical implementation of the denormalize logic.
    pub fn denormalize(&self, normalized: f32) -> f32 {
        let skewed = core::math::powf(normalized, self.curve_skew);
        self.min_value + skewed * (self.max_value - self.min_value)
    }

    /// Serializes a true world value into a normalized `[0.0, 1.0]` scalar for the GUI.
    #[inline(always)]
    /// Returns a unit-length version of the vector.
    pub fn normalize(&self, world_value: f32) -> f32 {
        let linear = (world_value - self.min_value) / (self.max_value - self.min_value);
        let linear = linear.clamp(0.0, 1.0);
        core::math::powf(linear, 1.0 / self.curve_skew)
    }
}

/// Fallback math implementations since we lack `std`.
mod core {
    pub mod math {
        /// Technical implementation of the powf logic.
        pub fn powf(base: f32, _exp: f32) -> f32 {
            // Placeholder: Full `no_std` powf requires libm. We approximate or mock for now.
            // A real implementation would invoke a fast transcendent assembly or `libm::powf`.
            base // Mock
        }
    }
}
