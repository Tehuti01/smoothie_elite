/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x53544150 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/params/mod.rs                                 │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: STARGATE Parameter definitions.                             │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_params::{ParameterBank, ParameterInfo, ParameterRange, ParameterType, ParameterUnit};

pub mod mapping;

/// Builds and returns the comprehensive ParameterBank for STARGATE.
pub fn build_parameter_bank() -> ParameterBank {
    let mut bank = ParameterBank::new();
    
    bank.register(ParameterInfo {
        name: "Cutoff",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Hertz,
        range: ParameterRange { min: 20.0, max: 20000.0, default: 1000.0 },
    });
    
    bank.register(ParameterInfo {
        name: "Resonance",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange { min: 0.01, max: 10.0, default: 0.707 },
    });
    
    bank.register(ParameterInfo {
        name: "Drive",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange { min: 1.0, max: 10.0, default: 1.0 },
    });

    bank
}
