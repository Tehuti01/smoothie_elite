/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x41756469 | REVISION: 2026.04.20                           │
 * │ PATH: crates/01-silicon/core/src/audio.rs                                │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core audio structures and primitives.                       │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use crate::primitives::Sample;

/// A single frame of multi-channel interleaved audio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFrame<const CHANNELS: usize> {
    pub samples: [Sample; CHANNELS],
}

impl<const CHANNELS: usize> AudioFrame<CHANNELS> {
    /// Creates a new audio frame from a slice of samples.
    pub fn new(samples: [Sample; CHANNELS]) -> Self {
        Self { samples }
    }

    /// Returns a mono sum of all channels.
    pub fn mono_sum(&self) -> Sample {
        self.samples.iter().sum::<f32>() / CHANNELS as f32
    }
}

impl<const CHANNELS: usize> Default for AudioFrame<CHANNELS> {
    fn default() -> Self {
        Self { samples: [0.0; CHANNELS] }
    }
}
