/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x57729e32 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/info/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the ParameterType enumeration.
pub enum ParameterType {
    Float,
    Int,
    Bool,
}
/// Technical implementation of the ParameterUnit enumeration.
pub enum ParameterUnit {
    Decibels,
    Hertz,
    Percent,
    Generic,
}
/// Technical implementation of the ParameterRange structure.
pub struct ParameterRange {
    pub min: f32,
    pub max: f32,
    pub default: f32,
}
/// Technical implementation of the ParameterInfo structure.
pub struct ParameterInfo {
    pub name: &'static str,
    pub param_type: ParameterType,
    pub unit: ParameterUnit,
    pub range: ParameterRange,
}
