/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x79d9869c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/convert.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the bipolar_to_unipolar logic.
pub fn bipolar_to_unipolar(v: f32) -> f32 {
    v * 0.5 + 0.5
}

/// Convert unipolar [0, 1] to bipolar [-1, 1].
#[inline(always)]
/// Technical implementation of the unipolar_to_bipolar logic.
pub fn unipolar_to_bipolar(v: f32) -> f32 {
    v * 2.0 - 1.0
}

/// Apply bipolar/unipolar based on flag.
#[inline(always)]
/// Technical implementation of the convert logic.
pub fn convert(v: f32, bipolar: bool) -> f32 {
    if bipolar {
        v
    } else {
        bipolar_to_unipolar(v)
    }
}

/// Scale modulation depth for a destination.
#[inline(always)]
/// Technical implementation of the scale_depth logic.
pub fn scale_depth(value: f32, depth: f32, bipolar: bool) -> f32 {
    let scaled = value * depth;
    if bipolar {
        scaled
    } else {
        bipolar_to_unipolar(scaled)
    }
}
