/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4b3a2c9d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/ironstack/src/params.rs                                                       │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: IronStack Parameter Registration and Mapping.               │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_params::bank::ParameterBank;
use smoothie_params::info::{ParameterInfo, ParameterType, ParameterUnit, ParameterRange};

/// Initializes the parameter bank for the IronStack engine.
pub fn init_ironstack_params() -> ParameterBank {
    let mut bank = ParameterBank::new();
    
    // PHI-aligned default values (1.618...)
    
    bank.register(ParameterInfo {
        name: "Tube Drive",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.0,
            max: 2.0,
            default: 0.5,
        },
    });

    bank.register(ParameterInfo {
        name: "Plate Bias",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: -10.0,
            max: 0.0,
            default: -2.0,
        },
    });

    bank.register(ParameterInfo {
        name: "Master Out",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.0,
            max: 1.0,
            default: 0.8,
        },
    });

    // Phase X: Neural Synthesis Macros
    bank.register(ParameterInfo {
        name: "Neural Drive",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.0,
            max: 2.0,
            default: 1.0,
        },
    });

    bank.register(ParameterInfo {
        name: "Neural Mix",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Percent,
        range: ParameterRange {
            min: 0.0,
            max: 1.0,
            default: 0.0, // Default to dry for technical stabilization
        },
    });

    // Phase XI: Spatial Resonance Macros
    bank.register(ParameterInfo {
        name: "Reverb Mix",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Percent,
        range: ParameterRange {
            min: 0.0,
            max: 1.0,
            default: 0.0,
        },
    });

    bank.register(ParameterInfo {
        name: "Reverb Time",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.1,
            max: 20.0,
            default: 2.0,
        },
    });

    bank.register(ParameterInfo {
        name: "Reverb Size",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.5,
            max: 5.0,
            default: 1.0,
        },
    });

    // Phase XII: Cognitive Pitch Macros
    bank.register(ParameterInfo {
        name: "Pitch Snap",
        param_type: ParameterType::Float,
        unit: ParameterUnit::Percent,
        range: ParameterRange {
            min: 0.0,
            max: 1.0,
            default: 1.0, 
        },
    });

    bank.register(ParameterInfo {
        name: "Scale Mask",
        param_type: ParameterType::Int,
        unit: ParameterUnit::Generic,
        range: ParameterRange {
            min: 0.0,
            max: 4095.0, // 12-bit mask
            default: 2741.0, // C Major: 0b101010110101 -> Wait, C Major is 0, 2, 4, 5, 7, 9, 11
        },
    });
    
    bank
}
