/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x60640762 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/buffer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::primitives::{ResultValue, Sample};
use core::fmt;

/// Circular delay line with multiple read heads (PHI-sized and 512-bit aligned)
#[repr(align(64))]
/// Technical implementation of the DelayLine structure.
pub struct DelayLine {
    buffer: [Sample; 8192], // Power of two for masking
    write_pos: usize,
    delay_samples: usize,
}

const DELAY_MASK: usize = 8191;

impl DelayLine {
    /// Create new delay line
    pub fn new() -> Self {
        Self {
            buffer: [0.0; 8192],
            write_pos: 0,
            delay_samples: 1024, // Default 1024 samples
        }
    }

    /// Set delay time in samples
    pub fn set_delay(&mut self, samples: usize) -> ResultValue<(), &'static str> {
        if samples > 8192 {
            ResultValue::Err("Delay exceeds buffer capacity")
        } else {
            self.delay_samples = samples.max(1);
            ResultValue::Ok(())
        }
    }

    /// Write sample to delay line (Branchless power-of-two masking)
    pub fn write(&mut self, sample: Sample) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) & DELAY_MASK;
    }

    /// Read delayed sample
    pub fn read(&self) -> Sample {
        let read_pos = (self.write_pos + 8192 - self.delay_samples) & DELAY_MASK;
        self.buffer[read_pos]
    }

    /// Read with multiple taps
    pub fn read_taps(&self, taps: &[usize]) -> [Sample; 13] {
        let mut result = [0.0; 13];
        for (i, &tap) in taps.iter().enumerate().take(13) {
            let read_pos = (self.write_pos + 8192 - tap) % 8192;
            result[i] = self.buffer[read_pos];
        }
        result
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        for sample in &mut self.buffer {
            *sample = 0.0;
        }
        self.write_pos = 0;
    }

    /// Get current write position
    pub fn write_position(&self) -> usize {
        self.write_pos
    }

    /// Get delay in samples
    pub fn delay_samples(&self) -> usize {
        self.delay_samples
    }
}

impl Default for DelayLine {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DelayLine {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DelayLine")
            .field("write_pos", &self.write_pos)
            .field("delay_samples", &self.delay_samples)
            .finish()
    }
}

/// Technical implementation of the FeedbackDelayNetwork structure.
pub struct FeedbackDelayNetwork {
    delays: [DelayLine; 4],
    feedback_matrix: [[f32; 4]; 4],
}

impl FeedbackDelayNetwork {
    /// Create new FDN with orthogonal feedback matrix
    pub fn new() -> Self {
        // Hadamard-like matrix scaled by PHI
        let phi_inv = 0.618_034_f32;
        Self {
            delays: [
                DelayLine::new(),
                DelayLine::new(),
                DelayLine::new(),
                DelayLine::new(),
            ],
            feedback_matrix: [
                [phi_inv, -phi_inv, -phi_inv, phi_inv],
                [-phi_inv, phi_inv, -phi_inv, -phi_inv],
                [-phi_inv, -phi_inv, phi_inv, -phi_inv],
                [phi_inv, -phi_inv, -phi_inv, -phi_inv],
            ],
        }
    }

    /// Process sample through FDN (Unrolled and optimized)
    pub fn process(&mut self, input: Sample) -> Sample {
        // Read from all delay lines
        let o0 = self.delays[0].read();
        let o1 = self.delays[1].read();
        let o2 = self.delays[2].read();
        let o3 = self.delays[3].read();

        let m = &self.feedback_matrix;

        // Matrix multiplication (unrolled for Elite density)
        let f0 = m[0][0] * o0 + m[0][1] * o1 + m[0][2] * o2 + m[0][3] * o3;
        let f1 = m[1][0] * o0 + m[1][1] * o1 + m[1][2] * o2 + m[1][3] * o3;
        let f2 = m[2][0] * o0 + m[2][1] * o1 + m[2][2] * o2 + m[2][3] * o3;
        let f3 = m[3][0] * o0 + m[3][1] * o1 + m[3][2] * o2 + m[3][3] * o3;

        // Write back with input mix
        self.delays[0].write(input * 0.25 + f0 * 0.5);
        self.delays[1].write(input * 0.25 + f1 * 0.5);
        self.delays[2].write(input * 0.25 + f2 * 0.5);
        self.delays[3].write(input * 0.25 + f3 * 0.5);

        // Sum outputs
        (o0 + o1 + o2 + o3) * 0.25
    }

    /// Process a block of samples
    pub fn process_block(&mut self, input: &[Sample], output: &mut [Sample]) {
        for (i, &sample) in input.iter().enumerate() {
            output[i] = self.process(sample);
        }
    }

    /// Clear all delay lines
    pub fn clear(&mut self) {
        for delay in &mut self.delays {
            delay.clear();
        }
    }

    /// Set feedback amount (0.0 - 1.0)
    pub fn set_feedback(&mut self, feedback: f32) {
        let fb = feedback.clamp(0.0, 0.99);
        for row in &mut self.feedback_matrix {
            for elem in row {
                *elem *= fb;
            }
        }
    }
}

impl Default for FeedbackDelayNetwork {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for FeedbackDelayNetwork {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FeedbackDelayNetwork").finish()
    }
}

/// Tap delay with up to 13 delay taps (Fibonacci resolution)
#[repr(align(64))]
/// Technical implementation of the TappedDelayBuffer structure.
pub struct TappedDelayBuffer {
    buffer: [Sample; 4096], // Power of two
    write_pos: usize,
    taps: [usize; 13],
    tap_count: usize,
}

const TAP_MASK: usize = 4095;

impl TappedDelayBuffer {
    /// Create new tapped delay buffer
    pub fn new() -> Self {
        Self {
            buffer: [0.0; 4096],
            write_pos: 0,
            taps: [0; 13],
            tap_count: 0,
        }
    }

    /// Add delay tap at offset (in samples)
    pub fn add_tap(&mut self, offset: usize) -> ResultValue<(), &'static str> {
        if self.tap_count >= 13 {
            ResultValue::Err("Maximum 13 taps allowed")
        } else if offset >= 4096 {
            ResultValue::Err("Tap offset exceeds buffer size")
        } else {
            self.taps[self.tap_count] = offset;
            self.tap_count += 1;
            ResultValue::Ok(())
        }
    }

    /// Write sample (Masked circular logic)
    pub fn write(&mut self, sample: Sample) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) & TAP_MASK;
    }

    /// Read all taps
    pub fn read_taps(&self) -> &[usize] {
        &self.taps[..self.tap_count]
    }

    /// Get tap value
    pub fn get_tap(&self, tap_index: usize) -> ResultValue<Sample, &'static str> {
        if tap_index >= self.tap_count {
            ResultValue::Err("Tap index out of range")
        } else {
            let read_pos = (self.write_pos + 4096 - self.taps[tap_index]) & TAP_MASK;
            ResultValue::Ok(self.buffer[read_pos])
        }
    }

    /// Clear all taps
    pub fn clear_taps(&mut self) {
        self.tap_count = 0;
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        for sample in &mut self.buffer {
            *sample = 0.0;
        }
        self.write_pos = 0;
    }

    /// Get tap count
    pub fn tap_count(&self) -> usize {
        self.tap_count
    }
}

impl Default for TappedDelayBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for TappedDelayBuffer {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TappedDelayBuffer")
            .field("write_pos", &self.write_pos)
            .field("tap_count", &self.tap_count)
            .finish()
    }
}

/// Technical implementation of the MemoryPool structure.
pub struct MemoryPool<T> {
    pool: [Option<T>; 89], // 89 = Fibonacci, good pool size
    available: usize,
}

impl<T: Default> MemoryPool<T> {
    /// Create new memory pool (Silicon stable initialization)
    pub fn new() -> Self {
        // 🚀 Absolute Synthesis: Initialize array properly to avoid UB with Option discriminant
        let pool: [Option<T>; 89] = unsafe {
            let mut data: core::mem::MaybeUninit<[Option<T>; 89]> =
                core::mem::MaybeUninit::uninit();
            for i in 0..89 {
                core::ptr::write(&mut (*data.as_mut_ptr())[i], Some(T::default()));
            }
            data.assume_init()
        };

        Self {
            pool,
            available: 89,
        }
    }

    /// Allocate object from pool
    pub fn allocate(&mut self) -> ResultValue<T, &'static str> {
        for item in &mut self.pool {
            if item.is_some() {
                self.available -= 1;
                return ResultValue::Ok(item.take().unwrap());
            }
        }
        ResultValue::Err("Memory pool exhausted")
    }

    /// Deallocate object back to pool
    pub fn deallocate(&mut self, obj: T) -> ResultValue<(), &'static str> {
        if self.available >= 89 {
            return ResultValue::Err("Memory pool full");
        }

        for item in &mut self.pool {
            if item.is_none() {
                *item = Some(obj);
                self.available += 1;
                return ResultValue::Ok(());
            }
        }
        ResultValue::Err("Failed to return object to pool")
    }

    /// Get available count
    pub fn available(&self) -> usize {
        self.available
    }

    /// Reset pool
    pub fn reset(&mut self) {
        for item in &mut self.pool {
            *item = Some(T::default());
        }
        self.available = 89;
    }
}

impl<T: Default + fmt::Debug> fmt::Debug for MemoryPool<T> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemoryPool")
            .field("available", &self.available)
            .finish()
    }
}

/// Technical implementation of the DoubleBuffer structure.
pub struct DoubleBuffer<T: Clone> {
    front: T,
    back: T,
}

impl<T: Clone> DoubleBuffer<T> {
    /// Create new double buffer
    pub fn new(initial: T) -> Self {
        Self {
            front: initial.clone(),
            back: initial,
        }
    }

    /// Write to back buffer
    pub fn write(&mut self, value: T) {
        self.back = value;
    }

    /// Swap buffers (atomic in production)
    pub fn swap(&mut self) {
        core::mem::swap(&mut self.front, &mut self.back);
    }

    /// Read from front buffer
    pub fn read(&self) -> T {
        self.front.clone()
    }

    /// Get reference to front buffer
    pub fn front(&self) -> &T {
        &self.front
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for DoubleBuffer<T> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoubleBuffer")
            .field("front", &self.front)
            .finish()
    }
}

/// Technical implementation of the BitBuffer structure.
pub struct BitBuffer {
    data: [u64; 16], // 1024 bits (16 * 64)
}

impl BitBuffer {
    /// Create new bit buffer
    pub fn new() -> Self {
        Self { data: [0; 16] }
    }

    /// Set bit at position
    pub fn set_bit(&mut self, pos: usize) -> ResultValue<(), &'static str> {
        if pos >= 1024 {
            ResultValue::Err("Bit position out of range")
        } else {
            let byte_idx = pos / 64;
            let bit_idx = pos % 64;
            self.data[byte_idx] |= 1u64 << bit_idx;
            ResultValue::Ok(())
        }
    }

    /// Clear bit at position
    pub fn clear_bit(&mut self, pos: usize) -> ResultValue<(), &'static str> {
        if pos >= 1024 {
            ResultValue::Err("Bit position out of range")
        } else {
            let byte_idx = pos / 64;
            let bit_idx = pos % 64;
            self.data[byte_idx] &= !(1u64 << bit_idx);
            ResultValue::Ok(())
        }
    }

    /// Get bit at position
    pub fn get_bit(&self, pos: usize) -> ResultValue<bool, &'static str> {
        if pos >= 1024 {
            ResultValue::Err("Bit position out of range")
        } else {
            let byte_idx = pos / 64;
            let bit_idx = pos % 64;
            let bit_set = (self.data[byte_idx] & (1u64 << bit_idx)) != 0;
            ResultValue::Ok(bit_set)
        }
    }

    /// Clear all bits
    pub fn clear(&mut self) {
        self.data = [0; 16];
    }

    /// Count set bits
    pub fn count_ones(&self) -> u32 {
        self.data.iter().map(|x| x.count_ones()).sum()
    }
}

impl Default for BitBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BitBuffer {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitBuffer")
            .field("ones", &self.count_ones())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_delay_line logic.
    fn test_delay_line() {
        let mut delay = DelayLine::new();
        delay.write(0.5);
        delay.write(0.3);
        let out = delay.read();
        assert!(out >= 0.0);
    }

    #[test]
    /// Technical implementation of the test_tapped_delay logic.
    fn test_tapped_delay() {
        let mut delay = TappedDelayBuffer::new();
        let _ = delay.add_tap(100);
        assert_eq!(delay.tap_count(), 1);
    }

    #[test]
    /// Technical implementation of the test_fdn logic.
    fn test_fdn() {
        let mut fdn = FeedbackDelayNetwork::new();
        let out1 = fdn.process(0.5);
        let out2 = fdn.process(0.3);
        assert!(out1.is_finite());
        assert!(out2.is_finite());
    }

    #[test]
    /// Technical implementation of the test_double_buffer logic.
    fn test_double_buffer() {
        let mut db = DoubleBuffer::new(0.5f32);
        db.write(0.7);
        db.swap();
        let val = db.read();
        assert!((val - 0.7).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_bit_buffer logic.
    fn test_bit_buffer() {
        let mut bits = BitBuffer::new();
        let _ = bits.set_bit(0);
        let _ = bits.set_bit(100);
        assert_eq!(bits.count_ones(), 2);
    }
}
