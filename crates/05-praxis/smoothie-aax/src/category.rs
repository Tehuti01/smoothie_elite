/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xad3db939 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-aax/src/category.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the AaxCategory enumeration.
pub enum AaxCategory {
    Effect,
    Synthesizer,
    Instrument,
    Meter,
    Other,
}

impl AaxCategory {
    /// Technical implementation of the as_str logic.
    pub fn as_str(&self) -> &'static str {
        match self {
            AaxCategory::Effect => "Effect",
            AaxCategory::Synthesizer => "Synthesizer",
            AaxCategory::Instrument => "Instrument",
            AaxCategory::Meter => "Meter",
            AaxCategory::Other => "Other",
        }
    }

    /// Technical implementation of the is_effect logic.
    pub fn is_effect(&self) -> bool {
        matches!(self, AaxCategory::Effect)
    }

    /// Technical implementation of the is_instrument logic.
    pub fn is_instrument(&self) -> bool {
        matches!(self, AaxCategory::Synthesizer | AaxCategory::Instrument)
    }
}

impl Default for AaxCategory {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        AaxCategory::Effect
    }
}
