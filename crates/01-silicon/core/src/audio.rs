/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x294af17d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/audio.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::primitives::{ResultValue, Sample};
use crate::types::StereoSample;
use core::fmt;

/// Simple audio frame (collection of samples, 512-bit aligned)
#[repr(align(64))]
#[derive(Debug, Clone)]
/// Technical implementation of the AudioFrame structure.
pub struct AudioFrame {
    samples: [Sample; 64], // 64 = 2^6, perfect for NEON/SSE vectorization
    len: usize,
}

impl AudioFrame {
    /// Create empty audio frame
    pub fn new() -> Self {
        Self {
            samples: [0.0; 64],
            len: 0,
        }
    }

    /// Create frame with specific sample count
    pub fn with_capacity(capacity: usize) -> Self {
        let mut frame = Self::new();
        frame.len = capacity.min(64);
        frame
    }

    /// Get frame length
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if frame is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get frame capacity
    pub const fn capacity(&self) -> usize {
        64
    }

    /// Push sample to frame
    pub fn push(&mut self, sample: Sample) -> ResultValue<(), &'static str> {
        if self.len >= 64 {
            ResultValue::Err("AudioFrame capacity exceeded")
        } else {
            self.samples[self.len] = sample;
            self.len += 1;
            ResultValue::Ok(())
        }
    }

    /// Get sample at index
    pub fn get(&self, index: usize) -> ResultValue<Sample, &'static str> {
        if index < self.len {
            ResultValue::Ok(self.samples[index])
        } else {
            ResultValue::Err("Index out of bounds")
        }
    }

    /// Get mutable sample at index
    pub fn get_mut(&mut self, index: usize) -> ResultValue<&mut Sample, &'static str> {
        if index < self.len {
            ResultValue::Ok(&mut self.samples[index])
        } else {
            ResultValue::Err("Index out of bounds")
        }
    }

    /// Fill frame with silence
    pub fn silence(&mut self) {
        for sample in &mut self.samples[..self.len] {
            *sample = 0.0;
        }
    }

    /// Clear frame (reset length to 0)
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Get slice reference to active samples
    pub fn as_slice(&self) -> &[Sample] {
        &self.samples[..self.len]
    }

    /// Get mutable slice reference to active samples
    pub fn as_mut_slice(&mut self) -> &mut [Sample] {
        &mut self.samples[..self.len]
    }

    /// Scale frame by amplitude factor
    pub fn scale(&mut self, factor: f32) {
        for sample in &mut self.samples[..self.len] {
            *sample *= factor;
        }
    }

    /// Mix another frame into this one
    pub fn mix(&mut self, other: &AudioFrame, level: f32) -> ResultValue<(), &'static str> {
        if self.len != other.len {
            return ResultValue::Err("Frame lengths must match");
        }

        for i in 0..self.len {
            self.samples[i] += other.samples[i] * level;
        }
        ResultValue::Ok(())
    }

    /// Calculate RMS (Root Mean Square) level (Optimized)
    pub fn rms(&self) -> f32 {
        if self.len == 0 {
            return 0.0;
        }

        let mut sum = 0.0;
        let slice = &self.samples[..self.len];
        for &s in slice {
            sum += s * s;
        }

        crate::math::sqrt_approx(sum / self.len as f32)
    }

    /// Find peak absolute value
    pub fn peak(&self) -> f32 {
        self.samples[..self.len]
            .iter()
            .map(|s| s.abs())
            .fold(0.0f32, |max, val| if max > val { max } else { val })
    }

    /// Apply envelope to frame samples
    pub fn apply_envelope<F>(&mut self, mut envelope_fn: F)
    where
        F: FnMut(usize) -> f32,
    {
        for i in 0..self.len {
            self.samples[i] *= envelope_fn(i);
        }
    }
}

impl Default for AudioFrame {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Stereo audio frame (512-bit aligned channels)
#[repr(align(64))]
#[derive(Debug, Clone)]
/// Technical implementation of the StereoAudioFrame structure.
pub struct StereoAudioFrame {
    left: [Sample; 64],
    right: [Sample; 64],
    len: usize,
}

impl StereoAudioFrame {
    /// Create empty stereo frame
    pub fn new() -> Self {
        Self {
            left: [0.0; 64],
            right: [0.0; 64],
            len: 0,
        }
    }

    /// Get frame length
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if frame is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Push stereo sample
    pub fn push(&mut self, sample: StereoSample) -> ResultValue<(), &'static str> {
        if self.len >= 64 {
            ResultValue::Err("StereoAudioFrame capacity exceeded")
        } else {
            self.left[self.len] = sample.left;
            self.right[self.len] = sample.right;
            self.len += 1;
            ResultValue::Ok(())
        }
    }

    /// Get stereo sample at index
    pub fn get(&self, index: usize) -> ResultValue<StereoSample, &'static str> {
        if index < self.len {
            ResultValue::Ok(StereoSample::new(self.left[index], self.right[index]))
        } else {
            ResultValue::Err("Index out of bounds")
        }
    }

    /// Get left channel slice
    pub fn left_slice(&self) -> &[Sample] {
        &self.left[..self.len]
    }

    /// Get right channel slice
    pub fn right_slice(&self) -> &[Sample] {
        &self.right[..self.len]
    }

    /// Get mutable left channel slice
    pub fn left_mut(&mut self) -> &mut [Sample] {
        &mut self.left[..self.len]
    }

    /// Get mutable right channel slice
    pub fn right_mut(&mut self) -> &mut [Sample] {
        &mut self.right[..self.len]
    }

    /// Fill with silence
    pub fn silence(&mut self) {
        for i in 0..self.len {
            self.left[i] = 0.0;
            self.right[i] = 0.0;
        }
    }

    /// Clear frame
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Convert to mono
    pub fn to_mono(&self) -> AudioFrame {
        let mut mono = AudioFrame::with_capacity(self.len);
        for i in 0..self.len {
            mono.samples[i] = (self.left[i] + self.right[i]) * 0.5;
        }
        mono
    }

    /// Calculate RMS for both channels (Optimized)
    pub fn rms(&self) -> (f32, f32) {
        if self.len == 0 {
            return (0.0, 0.0);
        }

        let mut left_sum = 0.0;
        let mut right_sum = 0.0;
        for i in 0..self.len {
            left_sum += self.left[i] * self.left[i];
            right_sum += self.right[i] * self.right[i];
        }

        (
            crate::math::sqrt_approx(left_sum / self.len as f32),
            crate::math::sqrt_approx(right_sum / self.len as f32),
        )
    }

    /// Find peak absolute value (maximum across both channels)
    pub fn peak(&self) -> f32 {
        let left_peak = self.left[..self.len]
            .iter()
            .map(|s| s.abs())
            .fold(0.0f32, |max, val| if max > val { max } else { val });
        let right_peak = self.right[..self.len]
            .iter()
            .map(|s| s.abs())
            .fold(0.0f32, |max, val| if max > val { max } else { val });

        if left_peak > right_peak {
            left_peak
        } else {
            right_peak
        }
    }

    /// Calculate stereo width (correlating the channels)
    pub fn stereo_width(&self) -> f32 {
        if self.len == 0 {
            return 0.0;
        }

        let mut sum_sq = 0.0;
        let mut prod_sum = 0.0;

        for i in 0..self.len {
            let l = self.left[i];
            let r = self.right[i];
            sum_sq += l * l + r * r;
            prod_sum += l * r;
        }

        if sum_sq == 0.0 {
            0.0
        } else {
            1.0 - (prod_sum * 2.0 / sum_sq).abs()
        }
    }
}

impl Default for StereoAudioFrame {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the RingBuffer structure.
pub struct RingBuffer {
    buffer: [Sample; 4096],
    write_pos: usize,
}

const RING_MASK: usize = 4095;

impl RingBuffer {
    /// Create new ring buffer with maximum 4096 samples
    pub fn new() -> Self {
        Self {
            buffer: [0.0; 4096],
            write_pos: 0,
        }
    }

    /// Create ring buffer with specific capacity (Silicon note: Fixed at 4096 for masking)
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Write sample and advance position (Masked)
    pub fn write(&mut self, sample: Sample) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) & RING_MASK;
    }

    /// Read sample at offset from current position (Backwards in time)
    pub fn read(&self, offset: usize) -> Sample {
        // write_pos always points to the NEXT slot to be written.
        // offset 0 is the most recently written sample.
        let read_pos = (self.write_pos.wrapping_sub(offset + 1)) & RING_MASK;
        self.buffer[read_pos]
    }

    /// Read with fractional offset (interpolated, backwards in time)
    pub fn read_interpolated(&self, offset: f32) -> Sample {
        let offset_int = offset as usize;
        let offset_frac = offset - offset_int as f32;

        let idx0 = (self.write_pos.wrapping_sub(offset_int + 1)) & RING_MASK;
        let idx1 = (self.write_pos.wrapping_sub(offset_int + 2)) & RING_MASK;

        let sample0 = self.buffer[idx0];
        let sample1 = self.buffer[idx1];

        sample0 + offset_frac * (sample1 - sample0)
    }

    /// Clear buffer (fill with silence)
    pub fn clear(&mut self) {
        for sample in &mut self.buffer {
            *sample = 0.0;
        }
        self.write_pos = 0;
    }

    /// Get capacity
    pub const fn capacity(&self) -> usize {
        4096
    }

    /// Get write position
    pub fn write_position(&self) -> usize {
        self.write_pos
    }
}

impl Default for RingBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the MultiTapDelay structure.
pub struct MultiTapDelay {
    buffer: RingBuffer,
    tap_positions: [usize; 13], // 13 taps (Fibonacci harmonics)
    tap_count: usize,
}

impl MultiTapDelay {
    /// Create multi-tap delay
    pub fn new() -> Self {
        Self {
            buffer: RingBuffer::with_capacity(2048),
            tap_positions: [0; 13],
            tap_count: 0,
        }
    }

    /// Add delay tap at offset
    pub fn add_tap(&mut self, offset: usize) -> ResultValue<(), &'static str> {
        if self.tap_count >= 13 {
            ResultValue::Err("Maximum tap count (13) reached")
        } else {
            self.tap_positions[self.tap_count] = offset & RING_MASK;
            self.tap_count += 1;
            ResultValue::Ok(())
        }
    }

    /// Clear all taps
    pub fn clear_taps(&mut self) {
        self.tap_count = 0;
    }

    /// Write sample to buffer
    pub fn write(&mut self, sample: Sample) {
        self.buffer.write(sample);
    }

    /// Read all taps (returns array of samples)
    pub fn read_taps(&self) -> [Sample; 13] {
        let mut taps = [0.0; 13];
        for i in 0..self.tap_count {
            taps[i] = self.buffer.read(self.tap_positions[i]);
        }
        taps
    }

    /// Get tap count
    pub fn tap_count(&self) -> usize {
        self.tap_count
    }
}

impl Default for MultiTapDelay {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Graphical meter for level monitoring
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the PeakMeter structure.
pub struct PeakMeter {
    peak: f32,
    rms: f32,
    hold_time: usize,
    hold_samples: usize,
}

impl PeakMeter {
    /// Create new peak meter with hold time in samples
    pub fn new(hold_samples: usize) -> Self {
        Self {
            peak: 0.0,
            rms: 0.0,
            hold_time: hold_samples,
            hold_samples: 0,
        }
    }

    /// Process sample into meter
    pub fn process(&mut self, sample: f32) {
        let abs_sample = sample.abs();

        if abs_sample > self.peak {
            self.peak = abs_sample;
            self.hold_samples = self.hold_time;
        } else if self.hold_samples > 0 {
            self.hold_samples -= 1;
        } else {
            self.peak *= 0.99; // Slow decay
        }

        self.rms = self.rms * 0.99 + abs_sample * abs_sample * 0.01;
    }

    /// Get current peak level
    pub fn peak(&self) -> f32 {
        self.peak
    }

    /// Get RMS level
    pub fn rms(&self) -> f32 {
        crate::math::sqrt_approx(self.rms)
    }

    /// Reset meter
    pub fn reset(&mut self) {
        self.peak = 0.0;
        self.rms = 0.0;
        self.hold_samples = 0;
    }
}

impl Default for PeakMeter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100) // 1 second at 44.1kHz
    }
}

/// Technical implementation of the SpectrumAnalyzer structure.
pub struct SpectrumAnalyzer {
    bins: [f32; 256], // 256 frequency bins (2^8)
}

impl SpectrumAnalyzer {
    /// Create spectrum analyzer
    pub fn new() -> Self {
        Self { bins: [0.0; 256] }
    }

    /// Update spectrum (simplified magnitude analysis)
    pub fn update(&mut self, frame: &AudioFrame) {
        // Zero out bins
        for bin in &mut self.bins {
            *bin *= 0.95; // Decay
        }

        // Simplified: just distribute energy across bins based on content
        let peak = frame.peak();
        let rms = frame.rms();

        // Use simple energy distribution
        let energy = peak * rms;
        for i in 0..256.min(frame.len()) {
            let sample_val = frame.as_slice()[i].abs();
            self.bins[i] += sample_val * energy * 0.1;
        }
    }

    /// Get bin value
    pub fn get_bin(&self, bin: usize) -> f32 {
        if bin < 256 {
            self.bins[bin]
        } else {
            0.0
        }
    }

    /// Get all bins
    pub fn bins(&self) -> &[f32; 256] {
        &self.bins
    }

    /// Reset analyzer
    pub fn reset(&mut self) {
        self.bins = [0.0; 256];
    }
}

impl Default for SpectrumAnalyzer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for RingBuffer {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RingBuffer")
            .field("write_pos", &self.write_pos)
            .field("capacity", &4096)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_audio_frame logic.
    fn test_audio_frame() {
        let mut frame = AudioFrame::new();
        let _ = frame.push(0.5);
        let _ = frame.push(0.3);
        assert_eq!(frame.len(), 2);
        assert!(frame.get(0).is_ok());
    }

    #[test]
    /// Technical implementation of the test_stereo_frame logic.
    fn test_stereo_frame() {
        let mut frame = StereoAudioFrame::new();
        let sample = StereoSample::new(0.5, 0.3);
        let _ = frame.push(sample);
        assert_eq!(frame.len(), 1);
    }

    #[test]
    /// Technical implementation of the test_ring_buffer logic.
    fn test_ring_buffer() {
        let mut ring = RingBuffer::new();
        ring.write(0.5); // Written at index 0, write_pos becomes 1
        ring.write(0.3); // Written at index 1, write_pos becomes 2

        // Offset 0 is the most recently written sample (0.3)
        assert_eq!(ring.read(0), 0.3);
        // Offset 1 is the previous sample (0.5)
        assert_eq!(ring.read(1), 0.5);
    }

    #[test]
    /// Technical implementation of the test_multi_tap_delay logic.
    fn test_multi_tap_delay() {
        let mut delay = MultiTapDelay::new();
        let _ = delay.add_tap(10);
        assert_eq!(delay.tap_count(), 1);
    }

    #[test]
    /// Technical implementation of the test_peak_meter logic.
    fn test_peak_meter() {
        let mut meter = PeakMeter::new(100);
        meter.process(0.5);
        meter.process(0.7);
        assert!(meter.peak() > 0.6);
    }
}
