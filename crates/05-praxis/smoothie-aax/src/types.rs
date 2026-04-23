/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7dcc9bae | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-aax/src/types.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[allow(non_camel_case_types)]
pub type AAX_Result = i32;

pub const AAX_SUCCESS: AAX_Result = 0;
pub const AAX_ERROR_INVALID_PARAMETER_ID: AAX_Result = -1;

#[repr(C)]
/// Technical implementation of the AAX_Component structure.
pub struct AAX_Component<T> {
    pub instance: *mut T,
}
