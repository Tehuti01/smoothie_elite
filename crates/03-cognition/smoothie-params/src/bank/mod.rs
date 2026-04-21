/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x177c6f35 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/bank/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::{AtomicParameter, ParameterInfo};
use alloc::vec::Vec;

/// Technical implementation of the ManagedParameter structure.
pub struct ManagedParameter {
    pub info: ParameterInfo,
    pub atomic: AtomicParameter,
}

/// Technical implementation of the ParameterBank structure.
pub struct ParameterBank {
    params: Vec<ManagedParameter>,
}

impl ParameterBank {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// Technical implementation of the register logic.
    pub fn register(&mut self, info: ParameterInfo) {
        let atomic = AtomicParameter::new(info.range.default);
        self.params.push(ManagedParameter { info, atomic });
    }

    /// Technical implementation of the get logic.
    pub fn get(&self, index: usize) -> Option<&ManagedParameter> {
        self.params.get(index)
    }

    /// Technical implementation of the get_by_name logic.
    pub fn get_by_name(&self, name: &str) -> Option<&ManagedParameter> {
        self.params.iter().find(|p| p.info.name == name)
    }

    /// Technical implementation of the get_value logic.
    pub fn get_value(&self, name: &str) -> Option<f32> {
        self.get_by_name(name).map(|p| p.atomic.load())
    }
}
