/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x781a3680 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-preset/src/diff.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::snapshot::PresetSnapshot;
///
/// undo/redo stacks, "what changed?" UI feedback, and incremental state
/// synchronization in collaborative session contexts.
use alloc::vec::Vec;
// use smoothie_core::math::FloatExt;

/// A single parameter change in a preset diff.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the ParamChange structure.
pub struct ParamChange {
    pub param_index: u32,
    pub old_value: f32,
    pub new_value: f32,
}

/// The diff between two preset snapshots.
#[derive(Clone, Debug)]
/// Technical implementation of the PresetDiff structure.
pub struct PresetDiff {
    pub changes: Vec<ParamChange>,
}

impl PresetDiff {
    /// Compute all parameter differences between `before` and `after`.
    ///
    /// Only parameters with an absolute difference exceeding `epsilon`
    /// are recorded in the diff, avoiding floating-point noise.
    pub fn compute(before: &PresetSnapshot, after: &PresetSnapshot, epsilon: f32) -> Self {
        let count = before.active_count.min(after.active_count);
        let mut changes = Vec::new();

        for i in 0..count {
            let old_v = before.values[i];
            let new_v = after.values[i];
            if (old_v - new_v).abs() > epsilon {
                changes.push(ParamChange {
                    param_index: i as u32,
                    old_value: old_v,
                    new_value: new_v,
                });
            }
        }

        Self { changes }
    }

    /// Apply this diff to a snapshot, writing the `new_value`s.
    pub fn apply_to(&self, snapshot: &mut PresetSnapshot) {
        for change in &self.changes {
            let idx = change.param_index as usize;
            if idx < snapshot.active_count {
                snapshot.values[idx] = change.new_value;
            }
        }
    }

    /// Apply the inverse of this diff — effectively an undo operation.
    pub fn apply_inverse_to(&self, snapshot: &mut PresetSnapshot) {
        for change in &self.changes {
            let idx = change.param_index as usize;
            if idx < snapshot.active_count {
                snapshot.values[idx] = change.old_value;
            }
        }
    }

    /// Technical implementation of the change_count logic.
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }
    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

///
/// Capacity: `MAX_UNDO_DEPTH` levels. Oldest entries are discarded when full.
pub const MAX_UNDO_DEPTH: usize = 64;

/// Technical implementation of the UndoStack structure.
pub struct UndoStack {
    diffs: Vec<PresetDiff>,
    cursor: usize,
    /// Tracks whether there is anything beyond `cursor` to redo.
    max_cursor: usize,
}

impl UndoStack {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            diffs: Vec::with_capacity(MAX_UNDO_DEPTH),
            cursor: 0,
            max_cursor: 0,
        }
    }

    /// Push a new diff onto the stack, discarding any redo history beyond the cursor.
    pub fn push(&mut self, diff: PresetDiff) {
        // Discard redo history
        self.diffs.truncate(self.cursor);

        if self.diffs.len() >= MAX_UNDO_DEPTH {
            self.diffs.remove(0);
        } else {
            self.cursor += 1;
        }

        self.diffs.push(diff);
        self.max_cursor = self.cursor;
    }

    /// Undo: returns the diff to apply in inverse.
    pub fn undo(&mut self) -> Option<&PresetDiff> {
        if self.cursor == 0 {
            return None;
        }
        self.cursor -= 1;
        self.diffs.get(self.cursor)
    }

    /// Redo: returns the diff to re-apply.
    pub fn redo(&mut self) -> Option<&PresetDiff> {
        if self.cursor >= self.max_cursor {
            return None;
        }
        let diff = self.diffs.get(self.cursor);
        self.cursor += 1;
        diff
    }

    /// Technical implementation of the can_undo logic.
    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }
    /// Technical implementation of the can_redo logic.
    pub fn can_redo(&self) -> bool {
        self.cursor < self.max_cursor
    }
    /// Technical implementation of the depth logic.
    pub fn depth(&self) -> usize {
        self.diffs.len()
    }
}
