/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0491990c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-modulation/src/matrix/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;

/// A modulation source identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Technical implementation of the ModSource enumeration.
pub enum ModSource {
    Lfo(u8),
    Envelope(u8),
    MidiCc(u8),
    Velocity,
    Aftertouch,
    PitchBend,
    MacroKnob(u8),
    Random,
    Constant(u16),
}

/// A modulation destination identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Technical implementation of the ModDestination enumeration.
pub enum ModDestination {
    Pitch,
    Volume,
    Pan,
    FilterCutoff,
    FilterResonance,
    EnvelopeAttack(u8),
    EnvelopeDecay(u8),
    EnvelopeSustain(u8),
    EnvelopeRelease(u8),
    LfoRate(u8),
    LfoDepth(u8),
    ReverbWet,
    ReverbSize,
    DelayFeedback,
    Custom(u32),
}

/// Curve applied to the modulation signal before scaling by depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ModCurve enumeration.
pub enum ModCurve {
    /// Pass the source value straight through (bipolar or unipolar).
    Linear,
    /// Squared — emphasises values near zero.
    Squared,
    /// Cubed — heavily emphasises values near zero.
    Cubed,
    /// Absolute — treats negative values as positive.
    Absolute,
    /// Inverted — `1.0 − source`.
    Inverted,
}

/// A single modulation connection.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ModRoute structure.
pub struct ModRoute {
    pub source: ModSource,
    pub destination: ModDestination,
    /// Modulation depth in normalised units [−1.0, 1.0].
    pub depth: f32,
    pub curve: ModCurve,
    pub enabled: bool,
}

impl ModRoute {
    /// Initializes a new instance of the associated type.
    pub fn new(source: ModSource, destination: ModDestination, depth: f32) -> Self {
        Self {
            source,
            destination,
            depth,
            curve: ModCurve::Linear,
            enabled: true,
        }
    }

    /// Apply curve shaping to a source value.
    #[inline(always)]
    /// Technical implementation of the apply_curve logic.
    pub fn apply_curve(&self, v: f32) -> f32 {
        let shaped = match self.curve {
            ModCurve::Linear => v,
            ModCurve::Squared => v * v * v.signum(),
            ModCurve::Cubed => v * v * v,
            ModCurve::Absolute => v.abs(),
            ModCurve::Inverted => 1.0 - v,
        };
        shaped * self.depth
    }
}

///
/// are silently dropped — the UI layer should enforce this constraint.
pub const MAX_ROUTES: usize = 128;

/// Technical implementation of the ModMatrix structure.
pub struct ModMatrix {
    routes: Vec<ModRoute>,
    /// Source value cache: updated once per block by `update_sources()`.
    source_values: alloc::collections::BTreeMap<ModSourceKey, f32>,
    /// Accumulated destination deltas — cleared and rebuilt each tick.
    destination_deltas: Vec<(ModDestination, f32)>,
}

/// Flat key for BTreeMap storage of source values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ModSourceKey(u32);

impl ModSourceKey {
    /// Technical implementation of the from_source logic.
    fn from_source(s: ModSource) -> Self {
        let v = match s {
            ModSource::Lfo(n) => 0x0100 | n as u32,
            ModSource::Envelope(n) => 0x0200 | n as u32,
            ModSource::MidiCc(n) => 0x0300 | n as u32,
            ModSource::Velocity => 0x0400,
            ModSource::Aftertouch => 0x0401,
            ModSource::PitchBend => 0x0402,
            ModSource::MacroKnob(n) => 0x0500 | n as u32,
            ModSource::Random => 0x0600,
            ModSource::Constant(v) => 0x0700 | v as u32,
        };
        Self(v)
    }
}

impl ModMatrix {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            routes: Vec::with_capacity(MAX_ROUTES),
            source_values: alloc::collections::BTreeMap::new(),
            destination_deltas: Vec::with_capacity(MAX_ROUTES),
        }
    }

    /// Add a modulation route. Returns false if `MAX_ROUTES` would be exceeded.
    pub fn add_route(&mut self, route: ModRoute) -> bool {
        if self.routes.len() >= MAX_ROUTES {
            return false;
        }
        self.routes.push(route);
        true
    }

    /// Remove all routes matching the given source and destination pair.
    pub fn remove_route(&mut self, source: ModSource, destination: ModDestination) {
        self.routes
            .retain(|r| !(r.source == source && r.destination == destination));
    }

    /// Update the cached value for a source (call from audio thread once per block).
    pub fn set_source_value(&mut self, source: ModSource, value: f32) {
        let key = ModSourceKey::from_source(source);
        self.source_values.insert(key, value);
    }

    /// Process all active routes and return accumulated destination deltas.
    ///
    /// The returned slice is valid until the next call to `process()`.
    pub fn process(&mut self) -> &[(ModDestination, f32)] {
        self.destination_deltas.clear();

        for route in self.routes.iter().filter(|r| r.enabled) {
            let key = ModSourceKey::from_source(route.source);
            if let Some(&src_val) = self.source_values.get(&key) {
                let mod_val = route.apply_curve(src_val);
                self.destination_deltas.push((route.destination, mod_val));
            }
        }

        &self.destination_deltas
    }

    /// Returns the total modulation amount applied to a destination (summed).
    pub fn modulation_for(&self, dest: ModDestination) -> f32 {
        self.destination_deltas
            .iter()
            .filter(|(d, _)| *d == dest)
            .map(|(_, v)| v)
            .sum()
    }

    /// Technical implementation of the route_count logic.
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }
}
