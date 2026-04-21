/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb362761f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/predelay.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// dry input and the reverb algorithm input. Pre-delay creates the
/// sounds without obscuring the attack transient.
use alloc::vec::Vec;
use smoothie_core::math::FloatExt;

/// Technical implementation of the PreDelay structure.
pub struct PreDelay {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    delay_samples: usize,
    buf_size: usize,
}

impl PreDelay {
    /// Allocate a pre-delay capable of holding up to `max_ms` milliseconds.
    pub fn new(max_ms: f32, sample_rate: f32) -> Self {
        let buf_size = ((max_ms / 1000.0) * sample_rate) as usize + 2;
        let buf_size = buf_size.next_power_of_two();
        Self {
            buffer_l: vec![0.0; buf_size],
            buffer_r: vec![0.0; buf_size],
            write_pos: 0,
            delay_samples: 0,
            buf_size,
        }
    }

    /// Set delay time in milliseconds [0, max_ms].
    pub fn set_delay_ms(&mut self, ms: f32, sample_rate: f32) {
        self.delay_samples = ((ms / 1000.0) * sample_rate) as usize;
        self.delay_samples = self.delay_samples.min(self.buf_size - 1);
    }

    /// Process one stereo sample through the delay line.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        self.buffer_l[self.write_pos] = in_l;
        self.buffer_r[self.write_pos] = in_r;
        let read = (self.write_pos + self.buf_size - self.delay_samples) % self.buf_size;
        let out = (self.buffer_l[read], self.buffer_r[read]);
        self.write_pos = (self.write_pos + 1) % self.buf_size;
        out
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
