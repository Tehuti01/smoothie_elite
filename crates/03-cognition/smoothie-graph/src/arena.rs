/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbb2293be | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/arena.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
use alloc::vec::Vec;

/// Technical implementation of the BufferArena structure.
pub struct BufferArena {
    /// The single backing allocation for the entire session.
    storage: Vec<f32>,
    /// Number of samples per buffer slice.
    block_size: usize,
}

impl BufferArena {
    /// Allocate a new arena capable of holding `node_count` stereo buffers
    /// of `block_size` samples each. This is the **only** allocation that
    /// the graph system ever performs.
    pub fn new(node_count: usize, channel_count: usize, block_size: usize) -> Self {
        let total_samples = node_count * channel_count * block_size;
        Self {
            storage: vec![0.0_f32; total_samples],
            block_size,
        }
    }

    /// Return a mutable slice for a specific node & channel combination.
    ///
    /// # Panics
    /// Panics in debug mode if `node_index` or `channel` are out of bounds.
    pub fn buffer_mut(
        &mut self,
        node_index: usize,
        channel: usize,
        channel_count: usize,
    ) -> &mut [f32] {
        let offset = (node_index * channel_count + channel) * self.block_size;
        &mut self.storage[offset..offset + self.block_size]
    }

    /// Return an immutable view of a specific node's channel buffer.
    pub fn buffer(&self, node_index: usize, channel: usize, channel_count: usize) -> &[f32] {
        let offset = (node_index * channel_count + channel) * self.block_size;
        &self.storage[offset..offset + self.block_size]
    }

    /// Zero out all buffers in one linear pass — no looping overhead.
    pub fn clear_all(&mut self) {
        for sample in self.storage.iter_mut() {
            *sample = 0.0;
        }
    }

    /// Returns the configured block size.
    #[inline(always)]
    /// Technical implementation of the block_size logic.
    pub fn block_size(&self) -> usize {
        self.block_size
    }
}
