/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x513945b5 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-au/src/category.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the AuCategory enumeration.
pub enum AuCategory {
    Effect,
    MusicEffect,
    Mixer,
    Generator,
    Instrument,
    Panner,
    FormatConverter,
    EffectInstrument,
    Other,
}

impl AuCategory {
    /// Technical implementation of the as_str logic.
    pub fn as_str(&self) -> &'static str {
        match self {
            AuCategory::Effect => "effect",
            AuCategory::MusicEffect => "music effect",
            AuCategory::Mixer => "mixer",
            AuCategory::Generator => "generator",
            AuCategory::Instrument => "instrument",
            AuCategory::Panner => "panner",
            AuCategory::FormatConverter => "format converter",
            AuCategory::EffectInstrument => "effect instrument",
            AuCategory::Other => "other",
        }
    }

    /// Technical implementation of the to_u32 logic.
    pub fn to_u32(&self) -> u32 {
        match self {
            AuCategory::Effect => u32::from_be_bytes(*b"aufx"),
            AuCategory::MusicEffect => u32::from_be_bytes(*b"aumi"),
            AuCategory::Mixer => u32::from_be_bytes(*b"aumx"),
            AuCategory::Generator => u32::from_be_bytes(*b"augn"),
            AuCategory::Instrument => u32::from_be_bytes(*b"auin"),
            AuCategory::Panner => u32::from_be_bytes(*b"aupn"),
            AuCategory::FormatConverter => u32::from_be_bytes(*b"aucv"),
            AuCategory::EffectInstrument => u32::from_be_bytes(*b"aufi"),
            AuCategory::Other => u32::from_be_bytes(*b"auco"),
        }
    }

    /// Technical implementation of the is_instrument logic.
    pub fn is_instrument(&self) -> bool {
        matches!(
            self,
            AuCategory::Generator | AuCategory::Instrument | AuCategory::EffectInstrument
        )
    }

    /// Technical implementation of the is_effect logic.
    pub fn is_effect(&self) -> bool {
        matches!(
            self,
            AuCategory::Effect | AuCategory::MusicEffect | AuCategory::Panner
        )
    }
}

impl Default for AuCategory {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        AuCategory::Effect
    }
}
