/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6d3c8dc9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-preset/src/bank.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::snapshot::PresetSnapshot;
use alloc::string::String;
///
/// Supports search by name, tag filtering, and A/B comparison slots.
use alloc::vec::Vec;

/// Metadata for a single preset in the bank.
#[derive(Clone, Debug)]
/// Technical implementation of the PresetEntry structure.
pub struct PresetEntry {
    pub name: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub snapshot: PresetSnapshot,
    pub is_modified: bool,
}

impl PresetEntry {
    /// Initializes a new instance of the associated type.
    pub fn new(name: impl Into<String>, snapshot: PresetSnapshot) -> Self {
        Self {
            name: name.into(),
            author: String::new(),
            category: String::new(),
            tags: Vec::new(),
            snapshot,
            is_modified: false,
        }
    }
}

/// Technical implementation of the PresetBank structure.
pub struct PresetBank {
    entries: Vec<PresetEntry>,
    /// Currently selected preset index.
    pub active_index: usize,
    /// A-slot for A/B comparison.
    pub slot_a: Option<PresetSnapshot>,
    /// B-slot for A/B comparison.
    pub slot_b: Option<PresetSnapshot>,
    /// Interpolation amount between slot_a and slot_b [0.0, 1.0].
    pub ab_morph: f32,
}

impl PresetBank {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            active_index: 0,
            slot_a: None,
            slot_b: None,
            ab_morph: 0.0,
        }
    }

    /// Add a preset to the bank and return its index.
    pub fn add(&mut self, entry: PresetEntry) -> usize {
        let idx = self.entries.len();
        self.entries.push(entry);
        idx
    }

    /// Get a reference to a preset entry by index.
    pub fn get(&self, index: usize) -> Option<&PresetEntry> {
        self.entries.get(index)
    }

    /// Get a mutable reference to a preset entry by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PresetEntry> {
        self.entries.get_mut(index)
    }

    /// Select a preset by index and return its snapshot.
    pub fn select(&mut self, index: usize) -> Option<&PresetSnapshot> {
        self.active_index = index;
        self.entries.get(index).map(|e| &e.snapshot)
    }

    /// Search presets by name substring (case-insensitive).
    pub fn search_by_name(&self, query: &str) -> Vec<usize> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.name.to_lowercase().contains(&query_lower))
            .map(|(i, _)| i)
            .collect()
    }

    /// Filter presets by category.
    pub fn filter_by_category<'a>(&'a self, category: &'a str) -> Vec<(usize, &'a PresetEntry)> {
        self.entries
            .iter()
            .enumerate()
            .filter(move |(_, e)| e.category.as_str() == category)
            .collect()
    }

    /// Capture the current state into A-slot.
    pub fn capture_slot_a(&mut self, snapshot: PresetSnapshot) {
        self.slot_a = Some(snapshot);
    }

    /// Capture the current state into B-slot.
    pub fn capture_slot_b(&mut self, snapshot: PresetSnapshot) {
        self.slot_b = Some(snapshot);
    }

    /// Compute the A/B morphed snapshot at the current `ab_morph` amount.
    pub fn morphed_snapshot(&self) -> Option<PresetSnapshot> {
        let a = self.slot_a.as_ref()?;
        let b = self.slot_b.as_ref()?;
        let mut out = PresetSnapshot::new(a.active_count);
        a.interpolate(b, self.ab_morph, &mut out);
        Some(out)
    }

    /// Technical implementation of the count logic.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Technical implementation of the active_preset logic.
    pub fn active_preset(&self) -> Option<&PresetEntry> {
        self.entries.get(self.active_index)
    }
}
