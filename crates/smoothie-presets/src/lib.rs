//! smoothie-presets — 'Elite' Patch Distribution.
//! High-performance CRDT-based preset management for the distributed studio.

use serde::{Serialize, Deserialize};
use loro::{LoroDoc, LoroMap};

/// A preset for an 'Elite' synth or effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub name: String,
    pub parameters: Vec<f32>,
    pub metadata: Vec<(String, String)>,
}

/// the 'Elite' Preset Lattice.
pub struct PresetLattice {
    pub doc: LoroDoc,
}

impl PresetLattice {
    /// Initialize a new CRDT lattice for preset synchronization.
    pub fn new() -> Self {
        Self {
            doc: LoroDoc::new(),
        }
    }

    /// Store a patch into the lattice.
    pub fn save_patch(&self, patch: Patch) {
        let lattice: LoroMap = self.doc.get_map("patches");
        // Correct way to insert a nested container in Loro v0.16
        let patch_map = lattice.insert_container(&patch.name, LoroMap::new()).unwrap();
        patch_map.insert("name", patch.name.clone()).unwrap();
    }

    /// Retrieve a list of all stored patch names.
    pub fn list_patches(&self) -> Vec<String> {
        let lattice: LoroMap = self.doc.get_map("patches");
        // In Loro v0.16, we can use the value and then get keys
        let value = lattice.get_value();
        if let Some(map) = value.as_map() {
            map.keys().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
