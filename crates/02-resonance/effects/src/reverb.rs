/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x52455642 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/reverb.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::primitives::Sample;
use smoothie_core::ring_buffer::DelayLine;

/// Technical implementation of the ReverbEffect structure.
pub struct ReverbEffect {
    comb_buffers: [Vec<Sample>; 8],
    allpass_buffers: [Vec<Sample>; 4],
    comb_indices: [usize; 8],
    allpass_indices: [usize; 4],
    room_size: f32,
    damping: f32,
    wet: f32,
    dry: f32,
}

use smoothie_core::plugin::Reset;

impl Reset for ReverbEffect {
    fn reset(&mut self) {
        // Reset all comb and allpass buffers
        for b in &mut self.comb_buffers {
            for sample in b.iter_mut() { *sample = 0.0; }
        }
        for b in &mut self.allpass_buffers {
             for sample in b.iter_mut() { *sample = 0.0; }
        }
    }
}

impl ReverbEffect {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            comb_buffers: [
                vec![0.0; 1116], vec![0.0; 1188], vec![0.0; 1277], vec![0.0; 1356],
                vec![0.0; 1422], vec![0.0; 1491], vec![0.0; 1557], vec![0.0; 1617],
            ],
            allpass_buffers: [
                vec![0.0; 556], vec![0.0; 441], vec![0.0; 341], vec![0.0; 225],
            ],
            comb_indices: [0; 8],
            allpass_indices: [0; 4],
            room_size: 0.5,
            damping: 0.5,
            wet: 0.3,
            dry: 0.7,
        }
    }

    /// Technical implementation of the set_room_size logic.
    pub fn set_room_size(&mut self, size: f32) {
        self.room_size = size.clamp(0.0, 0.98);
    }

    /// Technical implementation of the set_damping logic.
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut output = 0.0;

        // Parallel comb filters
        for i in 0..8 {
            let idx = self.comb_indices[i];
            let delayed = self.comb_buffers[i][idx];
            let val = input + delayed * self.room_size;
            self.comb_buffers[i][idx] = val;
            output += delayed;
            self.comb_indices[i] = (idx + 1) % self.comb_buffers[i].len();
        }

        // Series allpass filters
        for i in 0..4 {
            let idx = self.allpass_indices[i];
            let delayed = self.allpass_buffers[i][idx];
            let val = output + delayed * 0.5;
            self.allpass_buffers[i][idx] = output - val * 0.5;
            output = val;
            self.allpass_indices[i] = (idx + 1) % self.allpass_buffers[i].len();
        }

        input * self.dry + output * self.wet
    }
}
