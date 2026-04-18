//! # Audio Primitives
//!
//! This module defines the core data structures used for audio processing throughout
//! the IRONSTACK-100 engine. It includes types for sample rates, individual stereo samples,
//! and various types of audio buffers.

/// Represents an audio sample rate in Hz.
/// Provides utility methods for calculating Nyquist frequency and sample period.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SampleRate(pub u32);

impl SampleRate {
    /// Creates a new SampleRate, clamped between 22.05kHz and 192kHz.
    pub fn new(hz: u32) -> Self {
        Self(hz.max(22050).min(192000))
    }

    /// Returns the Nyquist frequency (half the sample rate).
    pub fn nyquist(&self) -> f64 {
        self.0 as f64 / 2.0
    }

    /// Returns the sample period (tau) in seconds.
    pub fn tau(&self) -> f64 {
        1.0 / self.0 as f64
    }
}

/// A single stereo audio sample consisting of left and right channels.
#[derive(Debug, Clone, Copy)]
pub struct Sample {
    /// Left channel amplitude (-1.0 to 1.0).
    pub left: f32,
    /// Right channel amplitude (-1.0 to 1.0).
    pub right: f32,
}

impl Sample {
    /// Creates a new stereo sample.
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }

    /// Creates a mono sample by duplicating the value to both channels.
    pub fn mono(value: f32) -> Self {
        Self {
            left: value,
            right: value,
        }
    }

    /// Creates a silent sample (0.0 for both channels).
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }

    /// Returns the sum of both channels.
    pub fn sum(&self) -> f32 {
        self.left + self.right
    }

    /// Returns the average of both channels.
    pub fn average(&self) -> f32 {
        (self.left + self.right) * 0.5
    }

    /// Returns the maximum absolute amplitude of either channel.
    pub fn max(&self) -> f32 {
        self.left.abs().max(self.right.abs())
    }

    /// Returns the Root Mean Square (RMS) amplitude of the sample.
    pub fn rms(&self) -> f32 {
        ((self.left * self.left + self.right * self.right) * 0.5).sqrt()
    }

    /// Clamps the sample amplitude to the normalized range [-1.0, 1.0].
    pub fn clip(&self) -> Self {
        Self {
            left: self.left.clamp(-1.0, 1.0),
            right: self.right.clamp(-1.0, 1.0),
        }
    }

    /// Linearly interpolates between this sample and another sample.
    pub fn lerp(&self, other: &Sample, t: f32) -> Sample {
        let t = t.clamp(0.0, 1.0);
        Self {
            left: self.left + (other.left - self.left) * t,
            right: self.right + (other.right - self.right) * t,
        }
    }
}

impl std::ops::Add for Sample {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            left: self.left + other.left,
            right: self.right + other.right,
        }
    }
}

impl std::ops::Mul<f32> for Sample {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            left: self.left * scalar,
            right: self.right * scalar,
        }
    }
}

/// A multi-frame stereo audio buffer.
/// Manages left and right channel vectors separately for optimized processing.
pub struct AudioBuffer {
    left: Vec<f32>,
    right: Vec<f32>,
    capacity: usize,
    current_frame: u64,
}

impl AudioBuffer {
    /// Creates a new AudioBuffer with the specified frame capacity.
    pub fn new(max_frames: usize) -> Self {
        Self {
            left: vec![0.0; max_frames],
            right: vec![0.0; max_frames],
            capacity: max_frames,
            current_frame: 0,
        }
    }

    /// Alias for `new`.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(capacity)
    }

    /// Clears the buffer contents and resets the frame counter.
    pub fn clear(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.current_frame = 0;
    }

    /// Sets a specific frame in the buffer.
    pub fn set_frame(&mut self, frame: usize, sample: Sample) {
        if frame < self.capacity {
            self.left[frame] = sample.left;
            self.right[frame] = sample.right;
        }
    }

    /// Retrieves a sample at the specified frame index.
    pub fn get_frame(&self, frame: usize) -> Option<Sample> {
        if frame < self.current_frame as usize {
            Some(Sample::new(self.left[frame], self.right[frame]))
        } else {
            None
        }
    }

    /// Pushes a stereo sample to the end of the current frame sequence.
    pub fn push(&mut self, sample: Sample) {
        let frame = self.current_frame as usize;
        if frame < self.capacity {
            self.left[frame] = sample.left;
            self.right[frame] = sample.right;
            self.current_frame += 1;
        }
    }

    /// Returns the number of frames currently in use.
    pub fn frames(&self) -> u64 {
        self.current_frame
    }

    /// Returns the total frame capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns a slice of the left channel data up to the current frame.
    pub fn left_channel(&self) -> &[f32] {
        &self.left[..self.current_frame as usize]
    }

    /// Returns a slice of the right channel data up to the current frame.
    pub fn right_channel(&self) -> &[f32] {
        &self.right[..self.current_frame as usize]
    }

    /// Returns a mutable slice of the left channel data.
    pub fn left_channel_mut(&mut self) -> &mut [f32] {
        &mut self.left[..self.current_frame as usize]
    }

    /// Returns a mutable slice of the right channel data.
    pub fn right_channel_mut(&mut self) -> &mut [f32] {
        &mut self.right[..self.current_frame as usize]
    }

    /// Mixes another buffer into this one with a specified gain.
    pub fn mix(&mut self, other: &AudioBuffer, gain: f32) {
        let frames = self.current_frame.min(other.current_frame) as usize;
        for i in 0..frames {
            self.left[i] += other.left[i] * gain;
            self.right[i] += other.right[i] * gain;
        }
    }

    /// Multiplies all samples in the buffer by a gain scalar.
    pub fn multiply(&mut self, gain: f32) {
        for i in 0..self.current_frame as usize {
            self.left[i] *= gain;
            self.right[i] *= gain;
        }
    }

    /// Clamps all samples in the buffer to the range [-1.0, 1.0].
    pub fn clip(&mut self) {
        for i in 0..self.current_frame as usize {
            self.left[i] = self.left[i].clamp(-1.0, 1.0);
            self.right[i] = self.right[i].clamp(-1.0, 1.0);
        }
    }

    /// Fills the buffer from a mono slice, duplicating data to both channels.
    pub fn process_mono_to_stereo(&mut self, mono: &[f32]) {
        let len = mono.len().min(self.capacity);
        for i in 0..len {
            self.left[i] = mono[i];
            self.right[i] = mono[i];
        }
        self.current_frame = len as u64;
    }

    /// Returns a mono vector by averaging the stereo channels.
    pub fn process_stereo_to_mono(&self) -> Vec<f32> {
        let mut mono = vec![0.0; self.current_frame as usize];
        for i in 0..self.current_frame as usize {
            mono[i] = (self.left[i] + self.right[i]) * 0.5;
        }
        mono
    }
}

/// A circular buffer for managing delayed audio signals.
/// Used in delay, reverb, and modulation effects.
pub struct RingBuffer {
    buffer: Vec<f32>,
    write_pos: usize,
    read_pos: usize,
    size: usize,
}

impl RingBuffer {
    /// Creates a new silent RingBuffer of the specified size.
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            write_pos: 0,
            read_pos: 0,
            size,
        }
    }

    /// Writes a single sample into the buffer at the current write position.
    pub fn write(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.size;
    }

    /// Reads a sample from 'delay' frames ago.
    pub fn read(&self, delay: usize) -> f32 {
        let delay = delay.min(self.size - 1);
        let pos = (self.write_pos + self.size - 1 - delay) % self.size;
        self.buffer[pos]
    }

    /// Reads a sample from a fractional delay using linear interpolation.
    pub fn read_interp(&self, delay: f32) -> f32 {
        let delay = delay.min(self.size as f32 - 1.0);
        let int = delay as usize;
        let frac = delay - int as f32;
        let s1 = self.read(int);
        let s2 = self.read(int + 1);
        s1 + (s2 - s1) * frac
    }

    /// Resets the buffer to silence and moves positions back to the start.
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.read_pos = 0;
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
