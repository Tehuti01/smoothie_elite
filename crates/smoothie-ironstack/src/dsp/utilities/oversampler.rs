/// A utility for upsampling and downsampling audio signals.
///
/// Oversampling is used to reduce aliasing distortion by performing non-linear 
/// processing (like distortion or clipping) at a higher internal sample rate 
/// before filtering and downsampling back to the original rate.
pub struct Oversampler {
    /// The oversampling factor (2x, 4x, 8x).
    factor: usize,
    buffer_up: Vec<f32>,
    buffer_down: Vec<f32>,
    up_idx: usize,
    down_idx: usize,
}

impl Oversampler {
    /// Creates a new Oversampler with the specified factor.
    ///
    /// # Arguments
    /// * `factor` - Oversampling factor, clamped between 2 and 8.
    pub fn new(factor: usize) -> Self {
        let buffer_size = 256 * factor;
        Self {
            factor: factor.clamp(2, 8),
            buffer_up: vec![0.0; buffer_size],
            buffer_down: vec![0.0; buffer_size],
            up_idx: 0,
            down_idx: 0,
        }
    }

    /// Prepares a sample for oversampled processing by replicating it.
    /// Returns a slice of upsampled samples.
    pub fn oversample(&mut self, input: f32) -> &[f32] {
        self.buffer_up[self.up_idx] = input;
        self.up_idx = (self.up_idx + 1) % self.factor;

        let start = self.up_idx * (self.buffer_up.len() / self.factor);
        &self.buffer_up[start..start + (self.buffer_up.len() / self.factor)]
    }

    /// Collects a highly-sampled signal and averages it down to the target sample rate.
    pub fn downsample(&mut self, input: f32) -> f32 {
        self.buffer_down[self.down_idx] = input;
        self.down_idx = (self.down_idx + 1) % self.factor;

        if self.down_idx == 0 {
            let sum: f32 = self.buffer_down.iter().take(self.factor).sum();
            sum / self.factor as f32
        } else {
            0.0
        }
    }

    /// Returns the current oversampling factor.
    pub fn factor(&self) -> usize {
        self.factor
    }

    /// Resets internal buffers to silence.
    pub fn clear(&mut self) {
        self.buffer_up.fill(0.0);
        self.buffer_down.fill(0.0);
        self.up_idx = 0;
        self.down_idx = 0;
    }
}

impl Default for Oversampler {
    fn default() -> Self {
        Self::new(2)
    }
}
