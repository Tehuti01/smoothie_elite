/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8c308966 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-preset/src/snapshot.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

// use smoothie_core::math::FloatExt;

/// Maximum number of parameters supported in a single preset snapshot.
pub const MAX_PARAMS: usize = 1024;

///
/// Parameters beyond `active_count` are undefined and should be ignored.
#[derive(Clone, Debug)]
/// Technical implementation of the PresetSnapshot structure.
pub struct PresetSnapshot {
    pub values: [f32; MAX_PARAMS],
    /// Number of parameters actually populated in this snapshot.
    pub active_count: usize,
    /// Plugin-provided name (up to 15 chars + null).
    pub name: [u8; 64],
    /// User-provided tags for search/filter.
    pub tags: [u8; 128],
}

impl Default for PresetSnapshot {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            values: [0.0; MAX_PARAMS],
            active_count: 0,
            name: [0; 64],
            tags: [0; 128],
        }
    }
}

impl PresetSnapshot {
    /// Create a new empty snapshot for `param_count` parameters.
    pub fn new(param_count: usize) -> Self {
        debug_assert!(param_count <= MAX_PARAMS);
        Self {
            active_count: param_count,
            ..Default::default()
        }
    }

    /// Set the preset name from a UTF-8 byte slice.
    pub fn set_name(&mut self, name: &[u8]) {
        let len = name.len().min(63);
        self.name[..len].copy_from_slice(&name[..len]);
        self.name[len] = 0;
    }

    /// Get the preset name as a UTF-8 `&str` (trimmed at null byte).
    pub fn name_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(63);
        core::str::from_utf8(&self.name[..end]).unwrap_or("<invalid>")
    }

    /// Set a parameter value at `index`.
    #[inline(always)]
    /// Technical implementation of the set logic.
    pub fn set(&mut self, index: usize, value: f32) {
        debug_assert!(index < self.active_count);
        self.values[index] = value.clamp(0.0, 1.0);
    }

    /// Get a parameter value at `index`.
    #[inline(always)]
    /// Technical implementation of the get logic.
    pub fn get(&self, index: usize) -> f32 {
        self.values[index]
    }

    /// Linearly interpolate towards `other` by factor `t ∈ [0, 1]`.
    ///
    /// This enables smooth parameter morphing between presets — the
    /// interpolated snapshot can be applied sample-by-sample for seamless
    /// A/B transitions.
    pub fn interpolate(&self, other: &Self, t: f32, out: &mut Self) {
        let count = self.active_count.min(other.active_count).min(MAX_PARAMS);
        out.active_count = count;
        for i in 0..count {
            out.values[i] = self.values[i] * (1.0 - t) + other.values[i] * t;
        }
    }

    /// Compare two snapshots and return true if maximum difference exceeds `tolerance`.
    pub fn differs_from(&self, other: &Self, tolerance: f32) -> bool {
        let count = self.active_count.min(other.active_count);
        for i in 0..count {
            if (self.values[i] - other.values[i]).abs() > tolerance {
                return true;
            }
        }
        false
    }
}
