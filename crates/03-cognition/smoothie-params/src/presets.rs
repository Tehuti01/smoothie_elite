/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9291750c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/presets.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub const MAX_PRESETS: usize = 128;
pub const MAX_PRESET_SIZE: usize = 4096;

#[derive(Debug, Clone, PartialEq)]
/// Technical implementation of the Preset structure.
pub struct Preset {
    pub name: String,
    pub data: Vec<u8>,
}

impl Preset {
    /// Initializes a new instance of the associated type.
    pub fn new(name: &str, data: Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data,
        }
    }

    /// Technical implementation of the name logic.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Technical implementation of the data logic.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Technical implementation of the data_mut logic.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}

/// Technical implementation of the PresetBank structure.
pub struct PresetBank {
    presets: Vec<Preset>,
    current_index: Option<usize>,
    modified: bool,
}

impl PresetBank {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            presets: Vec::with_capacity(MAX_PRESETS),
            current_index: None,
            modified: false,
        }
    }

    /// Performs vector addition logic.
    pub fn add(&mut self, preset: Preset) -> usize {
        let idx = self.presets.len();
        if idx < MAX_PRESETS {
            self.presets.push(preset);
            idx
        } else {
            MAX_PRESETS
        }
    }

    /// Technical implementation of the get logic.
    pub fn get(&self, index: usize) -> Option<&Preset> {
        self.presets.get(index)
    }

    /// Technical implementation of the set_current logic.
    pub fn set_current(&mut self, index: usize) {
        if index < self.presets.len() {
            self.current_index = Some(index);
        }
    }

    /// Technical implementation of the current logic.
    pub fn current(&self) -> Option<&Preset> {
        self.current_index.and_then(|i| self.presets.get(i))
    }

    /// Technical implementation of the current_index logic.
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// Technical implementation of the has_presets logic.
    pub fn has_presets(&self) -> bool {
        !self.presets.is_empty()
    }

    /// Technical implementation of the preset_count logic.
    pub fn preset_count(&self) -> usize {
        self.presets.len()
    }

    /// Technical implementation of the set_modified logic.
    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }

    /// Technical implementation of the is_modified logic.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Technical implementation of the remove logic.
    pub fn remove(&mut self, index: usize) -> Option<Preset> {
        if index < self.presets.len() {
            let result = self.presets.remove(index);
            if self.current_index == Some(index) {
                self.current_index = None;
            }
            Some(result)
        } else {
            None
        }
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.presets.clear();
        self.current_index = None;
    }
}

impl Default for PresetBank {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_preset_bank logic.
    fn test_preset_bank() {
        let mut bank = PresetBank::new();
        let preset = Preset::new("Init", vec![0u8; 10]);
        let idx = bank.add(preset);
        assert_eq!(idx, 0);
        assert_eq!(bank.preset_count(), 1);
    }
}
