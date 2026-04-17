use std::f32::consts::PI;

/// A non-linear emulation of a vacuum tube (triode) gain stage.
///
/// This module simulates the harmonic distortion and 'soft-clipping' 
/// characteristics of analog tubes. It includes bias adjustment for 
/// asymmetrical clipping and a smoothing filter to emulate power supply sag.
pub struct TubeStage {
    sample_rate: f32,
    /// Gain applied before clipping.
    gain: f32,
    /// Bias offset for clipping asymmetry.
    bias: f32,
    /// Level of harmonic saturation.
    saturation: f32,
    /// Smoothing filter states for [Left, Right] channels.
    state: [f32; 2],
}

impl TubeStage {
    /// Creates a new TubeStage initialized with default guitar-centric values.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            gain: 0.75,
            bias: 0.5,
            saturation: 0.5,
            state: [0.0; 2],
        }
    }

    pub fn set_gain(&mut self, g: f32) {
        self.gain = g.clamp(0.0, 1.0);
    }
    pub fn set_bias(&mut self, b: f32) {
        self.bias = b.clamp(0.0, 1.0);
    }
    pub fn set_saturation(&mut self, s: f32) {
        self.saturation = s.clamp(0.0, 1.0);
    }

    /// Processes a single sample through the non-linear tube model.
    ///
    /// # Arguments
    /// * `input` - The input sample value.
    /// * `ch` - The channel index (0 for Left, 1 for Right).
    pub fn process(&mut self, input: f32, ch: usize) -> f32 {
        let gain_amount = 1.0 + self.gain * 15.0;
        let bias_voltage = (self.bias - 0.5) * 0.3;

        let x = input * gain_amount;
        let biased = x + bias_voltage;

        // Soft-clipping using tanh-like approximation
        let soft_clip = (biased / (1.0 + biased.abs())).tanh();
        
        // Add harmonic richness based on saturation
        let tube_curve = soft_clip * (1.0 + 0.2 * self.saturation * input * input);

        let softened = tube_curve.tanh();
        let saturated = (softened * 0.95) / (1.0 + self.gain * 0.3);

        // Smoothing filter (state is per-channel)
        let smoothed = self.state[ch] * 0.3 + saturated * 0.7;
        self.state[ch] = smoothed;

        smoothed
    }

    /// Resets the tube stages to a clean state.
    pub fn reset(&mut self) {
        self.state = [0.0; 2];
    }
}
