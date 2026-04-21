/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x155ec5fb | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/ring_buffer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(align(64))]
/// Technical implementation of the RingBuffer structure.
pub struct RingBuffer<T> {
    data: Box<[Option<T>]>,
    capacity: usize,
    read_idx: AtomicUsize,
    write_idx: AtomicUsize,
}

impl<T> RingBuffer<T> {
    /// Initializes a new instance of the associated type.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.next_power_of_two();
        let mut data = Vec::with_capacity(cap);
        for _ in 0..cap {
            data.push(None);
        }

        Self {
            data: data.into_boxed_slice(),
            capacity: cap,
            read_idx: AtomicUsize::new(0),
            write_idx: AtomicUsize::new(0),
        }
    }

    /// Technical implementation of the capacity logic.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        let w = self.write_idx.load(Ordering::Acquire);
        let r = self.read_idx.load(Ordering::Acquire);
        w.wrapping_sub(r) & (self.capacity - 1)
    }

    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Technical implementation of the is_full logic.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity - 1
    }

    /// Technical implementation of the write_one logic.
    pub fn write_one(&mut self, value: T) -> bool {
        let w = self.write_idx.load(Ordering::Relaxed);
        let r = self.read_idx.load(Ordering::Acquire);

        let next_w = (w + 1) & (self.capacity - 1);
        if next_w == r {
            return false;
        }

        self.data[w] = Some(value);
        self.write_idx.store(next_w, Ordering::Release);
        true
    }

    /// Technical implementation of the read_one logic.
    pub fn read_one(&mut self) -> Option<T> {
        let r = self.read_idx.load(Ordering::Relaxed);
        let w = self.write_idx.load(Ordering::Acquire);

        if r == w {
            return None;
        }

        let value = self.data[r].take();
        self.read_idx
            .store((r + 1) & (self.capacity - 1), Ordering::Release);

        value
    }

    /// Alias for write_one (for industrial API compatibility)
    pub fn push(&mut self, value: T) -> bool {
        self.write_one(value)
    }

    /// Alias for read_one (for industrial API compatibility)
    pub fn pop(&mut self) -> Option<T> {
        self.read_one()
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        while self.read_one().is_some() {}
    }
}

/// Simple fixed-size ring buffer for samples
#[repr(align(64))]
/// Technical implementation of the SampleRingBuffer structure.
pub struct SampleRingBuffer {
    buffer: [f32; 256],
    read_idx: usize,
    write_idx: usize,
    count: usize,
}

impl SampleRingBuffer {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            buffer: [0.0; 256],
            read_idx: 0,
            write_idx: 0,
            count: 0,
        }
    }

    /// Technical implementation of the capacity logic.
    pub const fn capacity(&self) -> usize {
        256
    }

    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Technical implementation of the is_full logic.
    pub fn is_full(&self) -> bool {
        self.count == 256
    }

    /// Technical implementation of the write logic.
    pub fn write(&mut self, value: f32) -> bool {
        if self.count >= 256 {
            return false;
        }
        self.buffer[self.write_idx] = value;
        self.write_idx = (self.write_idx + 1) & 255;
        self.count += 1;
        true
    }

    /// Technical implementation of the read logic.
    pub fn read(&mut self) -> Option<f32> {
        if self.count == 0 {
            return None;
        }
        let value = self.buffer[self.read_idx];
        self.read_idx = (self.read_idx + 1) & 255;
        self.count -= 1;
        Some(value)
    }

    /// Technical implementation of the peek logic.
    pub fn peek(&self) -> Option<f32> {
        if self.count == 0 {
            return None;
        }
        Some(self.buffer[self.read_idx])
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.read_idx = 0;
        self.write_idx = 0;
        self.count = 0;
    }
}

impl Default for SampleRingBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Stereo ring buffer
#[repr(align(64))]
/// Technical implementation of the StereoRingBuffer structure.
pub struct StereoRingBuffer {
    left: SampleRingBuffer,
    right: SampleRingBuffer,
}

impl StereoRingBuffer {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            left: SampleRingBuffer::new(),
            right: SampleRingBuffer::new(),
        }
    }

    /// Technical implementation of the write_stereo logic.
    pub fn write_stereo(&mut self, left: f32, right: f32) -> bool {
        self.left.write(left) && self.right.write(right)
    }

    /// Technical implementation of the read_stereo logic.
    pub fn read_stereo(&mut self) -> Option<(f32, f32)> {
        match (self.left.read(), self.right.read()) {
            (Some(l), Some(r)) => Some((l, r)),
            _ => {
                self.left.clear();
                self.right.clear();
                None
            }
        }
    }

    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.left.len().min(self.right.len())
    }
    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.left.is_empty()
    }
}

impl Default for StereoRingBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Delay line using ring buffer
#[repr(align(64))]
/// Technical implementation of the DelayLine structure.
pub struct DelayLine {
    buffer: [f32; 8192],
    write_idx: usize,
    max_delay: usize,
}

impl DelayLine {
    /// Initializes a new instance of the associated type.
    pub fn new(max_delay_samples: usize) -> Self {
        let max = max_delay_samples.min(8192);
        Self {
            buffer: [0.0; 8192],
            write_idx: 0,
            max_delay: max,
        }
    }

    /// Technical implementation of the write logic.
    pub fn write(&mut self, sample: f32) {
        self.buffer[self.write_idx] = sample;
        self.write_idx = (self.write_idx + 1) & 8191;
    }

    /// Technical implementation of the read logic.
    pub fn read(&self, delay: usize) -> f32 {
        let delay = delay.min(self.max_delay);
        let read_idx = (self.write_idx + 8192 - delay - 1) & 8191;
        self.buffer[read_idx]
    }

    /// Technical implementation of the read_interp logic.
    pub fn read_interp(&self, delay: f32) -> f32 {
        let int_delay = delay as usize;
        let frac = delay - int_delay as f32;
        let a = self.read(int_delay);
        let b = self.read(int_delay + 1);
        a + (b - a) * frac
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.buffer = [0.0; 8192];
    }

    /// Technical implementation of the max_delay logic.
    pub fn max_delay(&self) -> usize {
        self.max_delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_sample_ring_buffer logic.
    fn test_sample_ring_buffer() {
        let mut rb = SampleRingBuffer::new();

        rb.write(1.0);
        rb.write(2.0);
        assert_eq!(rb.len(), 2);

        assert_eq!(rb.read(), Some(1.0));
        assert_eq!(rb.read(), Some(2.0));
        assert_eq!(rb.read(), None);
    }

    #[test]
    /// Technical implementation of the test_delay_line logic.
    fn test_delay_line() {
        let mut delay = DelayLine::new(100);

        delay.write(1.0);
        delay.write(0.0);

        assert_eq!(delay.read(0), 0.0);
        assert_eq!(delay.read(1), 1.0);
    }
}
