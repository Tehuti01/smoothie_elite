/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x53544d41 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/params/mapping.rs                             │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Translating raw parameters to DSP targets.                  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_params::ParameterBank;

/// A structured snapshot of the current parameter state.
pub struct StargateState {
    pub cutoff: f32,
    pub resonance: f32,
    pub drive: f32,
}

impl StargateState {
    /// Extracts the current state from the ParameterBank.
    #[inline(always)]
    pub fn from_bank(bank: &ParameterBank) -> Self {
        Self {
            cutoff: bank.get_value("Cutoff").unwrap_or(1000.0),
            resonance: bank.get_value("Resonance").unwrap_or(0.707),
            drive: bank.get_value("Drive").unwrap_or(1.0),
        }
    }
}
