/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3d8578a7 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/reverb.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::{vec, vec::Vec};

///
/// Schroeder/Freeverb-inspired topology with Silicon Hardened stability
use smoothie_core::math::OnePoleFilter;
use smoothie_core::primitives::Sample;

/// Technical implementation of the ReverbEffect structure.
pub struct ReverbEffect {
    // 8 Parallel Comb Filters (Fibonacci-aligned lengths for density)
    comb_buffers: [Vec<f32>; 8],
    comb_positions: [usize; 8],
    comb_filters: [OnePoleFilter; 8],

    // 4 Serial Allpass Filters
    allpass_buffers: [Vec<f32>; 4],
    allpass_positions: [usize; 4],

    room_size: f32,
    damping: f32,
    wet: f32,
    dry: f32,
}

impl ReverbEffect {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            comb_buffers: [
                vec![0.0; 1116],
                vec![0.0; 1188],
                vec![0.0; 1277],
                vec![0.0; 1356],
                vec![0.0; 1422],
                vec![0.0; 1491],
                vec![0.0; 1557],
                vec![0.0; 1617],
            ],
            comb_positions: [0; 8],
            comb_filters: [
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
                OnePoleFilter::with_coefficient(0.5),
            ],
            allpass_buffers: [
                vec![0.0; 556],
                vec![0.0; 441],
                vec![0.0; 341],
                vec![0.0; 225],
            ],
            allpass_positions: [0; 4],
            room_size: 0.5,
            damping: 0.5,
            wet: 0.3,
            dry: 0.7,
        }
    }

    /// Technical implementation of the set_room_size logic.
    pub fn set_room_size(&mut self, size: f32) {
        self.room_size = size.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_damping logic.
    pub fn set_damping(&mut self, damp: f32) {
        self.damping = damp.clamp(0.0, 1.0);
        for f in self.comb_filters.iter_mut() {
            *f = OnePoleFilter::with_coefficient(1.0 - damp);
        }
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, wet: f32) {
        self.wet = wet.clamp(0.0, 1.0);
        self.dry = 1.0 - self.wet;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let mut comb_sum = 0.0;

        for i in 0..8 {
            let buffer = &mut self.comb_buffers[i];
            let pos = self.comb_positions[i];
            let delayed = buffer[pos];
            let damped = self.comb_filters[i].process(delayed);
            buffer[pos] = input + (damped * self.room_size);
            self.comb_positions[i] = (pos + 1) % buffer.len();
            comb_sum += delayed;
        }

        let mut ap_out = comb_sum * 0.125;
        for i in 0..4 {
            let buffer = &mut self.allpass_buffers[i];
            let pos = self.allpass_positions[i];
            let delayed = buffer[pos];
            let output = -ap_out + delayed;
            buffer[pos] = ap_out + (delayed * 0.5);
            self.allpass_positions[i] = (pos + 1) % buffer.len();
            ap_out = output;
        }

        input * self.dry + ap_out * self.wet
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        for b in self.comb_buffers.iter_mut() {
            for x in b.iter_mut() {
                *x = 0.0;
            }
        }
        for b in self.allpass_buffers.iter_mut() {
            for x in b.iter_mut() {
                *x = 0.0;
            }
        }
    }
}

impl Default for ReverbEffect {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
