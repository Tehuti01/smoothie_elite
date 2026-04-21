/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x410d38c8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/aftertouch.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the AftertouchMod structure.
pub struct AftertouchMod {
    value: f32,
}

impl AftertouchMod {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    /// Set aftertouch value from MIDI [0-127].
    #[inline(always)]
    /// Technical implementation of the set logic.
    pub fn set(&mut self, value: u8) {
        self.value = value as f32 / 127.0;
    }

    /// Set aftertouch value normalized [0.0, 1.0].
    #[inline(always)]
    /// Technical implementation of the set_normalized logic.
    pub fn set_normalized(&mut self, value: f32) {
        self.value = value.clamp(0.0, 1.0);
    }

    /// Get current aftertouch value.
    #[inline(always)]
    /// Technical implementation of the value logic.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Reset to no aftertouch.
    #[inline(always)]
    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

/// Polyphonic aftertouch (per-note pressure).
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the PolyAftertouch structure.
pub struct PolyAftertouch {
    values: [f32; 128],
}

impl Default for PolyAftertouch {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self { values: [0.0; 128] }
    }
}

impl PolyAftertouch {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { values: [0.0; 128] }
    }

    /// Set aftertouch for a specific note.
    #[inline(always)]
    /// Technical implementation of the set logic.
    pub fn set(&mut self, note: u8, value: u8) {
        self.values[note as usize] = value as f32 / 127.0;
    }

    /// Get aftertouch for a specific note.
    #[inline(always)]
    /// Technical implementation of the get logic.
    pub fn get(&self, note: u8) -> f32 {
        self.values[note as usize]
    }

    /// Get average aftertouch across all notes.
    #[inline(always)]
    /// Technical implementation of the average logic.
    pub fn average(&self) -> f32 {
        let sum: f32 = self.values.iter().sum();
        sum / 128.0
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for v in self.values.iter_mut() {
            *v = 0.0;
        }
    }
}
