//! Lock-free, allocation-free audio buffer for the processing thread.

/// A multi-channel audio buffer.
///
/// Wraps a mutable slice of channel slices, matching the layout provided
/// by the host. Zero-copy — no data is duplicated.
pub struct AudioBuffer<'a> {
    channels: &'a mut [&'a mut [f32]],
    sample_rate: f32,
}

impl<'a> AudioBuffer<'a> {
    /// Construct from raw channel slices provided by the host wrapper.
    ///
    /// # Safety
    /// The caller must ensure the lifetime and non-aliasing of channel pointers.
    #[inline]
    pub fn new(channels: &'a mut [&'a mut [f32]], sample_rate: f32) -> Self {
        Self { channels, sample_rate }
    }

    /// Number of channels.
    #[inline]
    pub fn channels(&self) -> usize { self.channels.len() }

    /// Number of samples per channel in this block.
    #[inline]
    pub fn samples(&self) -> usize {
        self.channels.first().map(|c| c.len()).unwrap_or(0)
    }

    /// Host sample rate.
    #[inline]
    pub fn sample_rate(&self) -> f32 { self.sample_rate }

    /// Immutable access to a channel slice.
    #[inline]
    pub fn channel(&self, index: usize) -> &[f32] { &self.channels[index] }

    /// Mutable access to a single channel slice.
    #[inline]
    pub fn channel_mut(&mut self, index: usize) -> &mut [f32] { &mut self.channels[index] }

    /// Apply a scalar gain to all channels.
    #[inline]
    pub fn apply_gain(&mut self, gain: f32) {
        if gain == 1.0 { return; }
        if gain == 0.0 { return self.silence(); }
        
        for ch in self.channels.iter_mut() {
            for s in ch.iter_mut() {
                *s *= gain;
            }
        }
    }

    /// Apply a block of gains (one per sample) to all channels.
    /// Useful for smoothed parameter automation.
    #[inline]
    pub fn apply_block_gain(&mut self, gains: &[f32]) {
        let n = self.samples().min(gains.len());
        for ch in self.channels.iter_mut() {
            for i in 0..n {
                ch[i] *= gains[i];
            }
        }
    }

    /// Zero (silence) all channels.
    #[inline]
    pub fn silence(&mut self) {
        for ch in self.channels.iter_mut() {
            ch.fill(0.0);
        }
    }

    /// Copy this buffer into `dst`, converting mono→stereo if needed.
    pub fn copy_to(&self, dst: &mut AudioBuffer<'_>) {
        let src_ch = self.channels();
        let dst_ch = dst.channels();
        let n = self.samples().min(dst.samples());
        for d in 0..dst_ch {
            let s = d.min(src_ch - 1);
            dst.channels[d][..n].copy_from_slice(&self.channels[s][..n]);
        }
    }
}

/// A single multi-channel frame (mutable reference to one sample per channel).
/// This 'Elite' version uses a zero-cost wrapper around the underlying buffer.
pub struct FrameMut<'a> {
    buffer: &'a mut AudioBuffer<'a>,
    index: usize,
}

impl<'a> FrameMut<'a> {
    /// Get the sample for a specific channel.
    #[inline]
    pub fn get(&self, ch: usize) -> f32 {
        self.buffer.channel(ch)[self.index]
    }

    /// Set the sample for a specific channel.
    #[inline]
    pub fn set(&mut self, ch: usize, value: f32) {
        self.buffer.channel_mut(ch)[self.index] = value;
    }

    /// Number of channels in this frame.
    #[inline]
    pub fn channels(&self) -> usize { self.buffer.channels() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_gain_and_silence() {
        let mut l = vec![1.0; 100];
        let mut r = vec![1.0; 100];
        let mut channels = [&mut l[..], &mut r[..]];
        let mut buffer = AudioBuffer::new(&mut channels, 44100.0);
        
        buffer.apply_gain(0.5);
        assert_eq!(buffer.channel(0)[0], 0.5);
        
        buffer.silence();
        assert_eq!(buffer.channel(0)[0], 0.0);
        assert_eq!(buffer.channel(1)[99], 0.0);
    }

    #[test]
    fn test_buffer_copy_mono_to_stereo() {
        let mut mono_data = vec![1.0, 2.0, 3.0];
        let mut mono_ch = [&mut mono_data[..]];
        let mono_buf = AudioBuffer::new(&mut mono_ch, 44100.0);
        
        let mut l = vec![0.0; 3];
        let mut r = vec![0.0; 3];
        let mut stereo_ch = [&mut l[..], &mut r[..]];
        let mut stereo_buf = AudioBuffer::new(&mut stereo_ch, 44100.0);
        
        mono_buf.copy_to(&mut stereo_buf);
        assert_eq!(stereo_buf.channel(0)[1], 2.0);
        assert_eq!(stereo_buf.channel(1)[1], 2.0);
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
