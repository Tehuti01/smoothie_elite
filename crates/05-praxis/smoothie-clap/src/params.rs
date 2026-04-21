/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x56005e68 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/params.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the ClapParamFlag enumeration.
pub enum ClapParamFlag {
    /// Parameter value changes require the audio engine to restart.
    RequiresProcess = 1 << 0,
    /// Parameter is hidden from the host automation lane.
    IsHidden = 1 << 1,
    /// Parameter value is read-only; the host cannot write it.
    IsReadOnly = 1 << 2,
    /// Parameter is a bypass toggle (special host treatment).
    IsBypass = 1 << 3,
    /// Values are stepped (discrete), not continuous.
    IsStepped = 1 << 4,
    /// Parameter should be automatable.
    IsAutomatable = 1 << 5,
}

/// Complete descriptor for a single automatable parameter.
#[derive(Debug, Clone)]
/// Technical implementation of the ClapParamInfo structure.
pub struct ClapParamInfo {
    /// Unique stable identifier for this parameter.
    pub id: u32,
    /// Display name shown in the host automation lane.
    pub name: &'static str,
    /// Optional module path for grouping, e.g. `"EQ/High Shelf"`.
    pub module: &'static str,
    /// Minimum value (in plain units, not normalised).
    pub min_value: f64,
    /// Maximum value (in plain units, not normalised).
    pub max_value: f64,
    /// Default value (in plain units, not normalised).
    pub default_value: f64,
    /// Bitfield of `ClapParamFlag` values.
    pub flags: u32,
}

impl ClapParamInfo {
    /// Construct a fully automatable continuous parameter.
    pub const fn continuous(
        id: u32,
        name: &'static str,
        module: &'static str,
        min: f64,
        max: f64,
        default: f64,
    ) -> Self {
        Self {
            id,
            name,
            module,
            min_value: min,
            max_value: max,
            default_value: default,
            flags: ClapParamFlag::IsAutomatable as u32,
        }
    }

    /// Construct a stepped (enumerated / integer) parameter.
    pub const fn stepped(
        id: u32,
        name: &'static str,
        module: &'static str,
        steps: u32,
        default: u32,
    ) -> Self {
        Self {
            id,
            name,
            module,
            min_value: 0.0,
            max_value: steps as f64,
            default_value: default as f64,
            flags: ClapParamFlag::IsStepped as u32 | ClapParamFlag::IsAutomatable as u32,
        }
    }

    /// Convert a plain value into the normalised [0.0, 1.0] range.
    #[inline(always)]
    /// Technical implementation of the normalise logic.
    pub fn normalise(&self, plain: f64) -> f64 {
        (plain - self.min_value) / (self.max_value - self.min_value)
    }

    /// Convert a normalised [0.0, 1.0] value into plain units.
    #[inline(always)]
    /// Technical implementation of the denormalise logic.
    pub fn denormalise(&self, normalised: f64) -> f64 {
        normalised * (self.max_value - self.min_value) + self.min_value
    }
}
