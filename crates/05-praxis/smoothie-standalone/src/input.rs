/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0536e170 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-standalone/src/input.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::collections::BTreeMap;

/// Technical implementation of the AutonomousInput structure.
pub struct AutonomousInput {
    pub midi_map: BTreeMap<u8, usize>, // CC -> Parameter Index
    pub learning: Option<usize>,      // Currently learning parameter index
}

impl AutonomousInput {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            midi_map: BTreeMap::new(),
            learning: None,
        }
    }

    pub fn start_learning(&mut self, param_idx: usize) {
        self.learning = Some(param_idx);
    }

    pub fn handle_midi_cc(&mut self, cc: u8, value: u8) -> Option<(usize, f32)> {
        if let Some(param_idx) = self.learning.take() {
            self.midi_map.insert(cc, param_idx);
            return Some((param_idx, value as f32 / 127.0));
        }

        if let Some(&param_idx) = self.midi_map.get(&cc) {
            return Some((param_idx, value as f32 / 127.0));
        }

        None
    }
}
