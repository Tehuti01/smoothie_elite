/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7fe388f8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/early/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// before the diffuse reverb tail. A good early reflection pattern is
///
///
/// a physical room model (image-source method for a rectangular room).
/// direct signal, each with a physical inverse-distance gain law.
/// # Tap Times
/// Room dimensions 8m × 12m × 3m (small studio) at c = 343 m/s.
/// Delays are scaled by `size` to simulate different room dimensions.
use alloc::vec::Vec;

/// A single early-reflection tap.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the EarlyTap structure.
pub struct EarlyTap {
    pub delay_ms: f32,
    pub gain_l: f32,
    pub gain_r: f32,
}

/// Default early reflection taps for a small studio (8m × 12m × 3m).
const DEFAULT_TAPS: [EarlyTap; 12] = [
    EarlyTap {
        delay_ms: 8.7,
        gain_l: 0.70,
        gain_r: 0.00,
    },
    EarlyTap {
        delay_ms: 11.3,
        gain_l: 0.00,
        gain_r: 0.70,
    },
    EarlyTap {
        delay_ms: 17.4,
        gain_l: -0.50,
        gain_r: 0.50,
    },
    EarlyTap {
        delay_ms: 22.7,
        gain_l: 0.50,
        gain_r: -0.50,
    },
    EarlyTap {
        delay_ms: 28.1,
        gain_l: 0.35,
        gain_r: 0.35,
    },
    EarlyTap {
        delay_ms: 34.8,
        gain_l: -0.25,
        gain_r: -0.25,
    },
    EarlyTap {
        delay_ms: 43.2,
        gain_l: 0.20,
        gain_r: 0.20,
    },
    EarlyTap {
        delay_ms: 51.6,
        gain_l: 0.15,
        gain_r: -0.15,
    },
    EarlyTap {
        delay_ms: 58.4,
        gain_l: -0.12,
        gain_r: 0.12,
    },
    EarlyTap {
        delay_ms: 67.3,
        gain_l: 0.10,
        gain_r: 0.10,
    },
    EarlyTap {
        delay_ms: 78.9,
        gain_l: -0.08,
        gain_r: -0.08,
    },
    EarlyTap {
        delay_ms: 91.5,
        gain_l: 0.06,
        gain_r: 0.06,
    },
];

/// Technical implementation of the EarlyReflections structure.
pub struct EarlyReflections {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    tap_positions: Vec<usize>,
    tap_gains_l: Vec<f32>,
    tap_gains_r: Vec<f32>,
    buf_size: usize,
}

impl EarlyReflections {
    /// Initializes a new instance of the associated type.
    pub fn new(taps: &[EarlyTap], size: f32, sample_rate: f32) -> Self {
        let size = size.clamp(0.25, 4.0);
        // Maximum delay determines buffer size
        let max_delay_ms = taps.iter().map(|t| t.delay_ms).fold(0.0_f32, f32::max);
        let max_delay_samples = ((max_delay_ms / 1000.0) * sample_rate * size) as usize + 16;
        let buf_size = max_delay_samples.next_power_of_two();

        let tap_positions: Vec<usize> = taps
            .iter()
            .map(|t| ((t.delay_ms / 1000.0) * sample_rate * size) as usize)
            .collect();
        let tap_gains_l: Vec<f32> = taps.iter().map(|t| t.gain_l).collect();
        let tap_gains_r: Vec<f32> = taps.iter().map(|t| t.gain_r).collect();

        Self {
            buffer_l: vec![0.0; buf_size],
            buffer_r: vec![0.0; buf_size],
            write_pos: 0,
            tap_positions,
            tap_gains_l,
            tap_gains_r,
            buf_size,
        }
    }

    /// Technical implementation of the with_default_taps logic.
    pub fn with_default_taps(size: f32, sample_rate: f32) -> Self {
        Self::new(&DEFAULT_TAPS, size, sample_rate)
    }

    /// Process one stereo sample. Returns the early reflection contribution.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        // Write input to ring buffer
        self.buffer_l[self.write_pos] = in_l;
        self.buffer_r[self.write_pos] = in_r;

        let mut out_l = 0.0_f32;
        let mut out_r = 0.0_f32;

        for (i, &tap) in self.tap_positions.iter().enumerate() {
            let read =
                (self.write_pos + self.buf_size - tap.min(self.buf_size - 1)) % self.buf_size;
            out_l += self.buffer_l[read] * self.tap_gains_l[i];
            out_r += self.buffer_r[read] * self.tap_gains_r[i];
        }

        self.write_pos = (self.write_pos + 1) % self.buf_size;
        (out_l, out_r)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for s in self.buffer_l.iter_mut() {
            *s = 0.0;
        }
        for s in self.buffer_r.iter_mut() {
            *s = 0.0;
        }
        self.write_pos = 0;
    }
}
