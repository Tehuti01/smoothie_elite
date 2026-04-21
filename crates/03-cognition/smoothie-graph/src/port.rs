/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x60dd0afb | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/port.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the PortDirection enumeration.
pub enum PortDirection {
    /// Audio flows **into** the node.
    Input,
    /// Audio flows **out of** the node.
    Output,
}

/// A single stereo or mono audio port on a graph node.
#[derive(Debug, Clone)]
/// Technical implementation of the AudioPort structure.
pub struct AudioPort {
    pub name: &'static str,
    pub direction: PortDirection,
    pub channel_count: usize,
}

impl AudioPort {
    /// Technical implementation of the mono_in logic.
    pub const fn mono_in(name: &'static str) -> Self {
        Self {
            name,
            direction: PortDirection::Input,
            channel_count: 1,
        }
    }

    /// Technical implementation of the mono_out logic.
    pub const fn mono_out(name: &'static str) -> Self {
        Self {
            name,
            direction: PortDirection::Output,
            channel_count: 1,
        }
    }

    /// Technical implementation of the stereo_in logic.
    pub const fn stereo_in(name: &'static str) -> Self {
        Self {
            name,
            direction: PortDirection::Input,
            channel_count: 2,
        }
    }

    /// Technical implementation of the stereo_out logic.
    pub const fn stereo_out(name: &'static str) -> Self {
        Self {
            name,
            direction: PortDirection::Output,
            channel_count: 2,
        }
    }
}
