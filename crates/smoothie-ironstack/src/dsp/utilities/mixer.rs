/// A multi-channel signal summing utility.
///
/// The Mixer provides individual gain controls for multiple input channels 
/// and a global master gain control. It is used to combine parallel signal 
/// paths (e.g., dry/wet mixing or multi-mic cabinet simulations).
pub struct Mixer {
    /// Gain factors for each individual channel.
    channels: Vec<f32>,
    /// Master gain applied to the final summed signal.
    master: f32,
}

impl Mixer {
    /// Creates a new Mixer with the specified number of input channels.
    pub fn new(channel_count: usize) -> Self {
        Self {
            channels: vec![1.0; channel_count.max(1)],
            master: 1.0,
        }
    }

    /// Sets the gain for a specific input channel.
    pub fn set_channel(&mut self, channel: usize, gain: f32) {
        if channel < self.channels.len() {
            self.channels[channel] = gain.clamp(0.0, 2.0);
        }
    }

    /// Sets the master gain applied to the output.
    pub fn set_master(&mut self, gain: f32) {
        self.master = gain.clamp(0.0, 2.0);
    }

    /// Mixes multiple mono inputs based on their channel gains and the master gain.
    pub fn mix(&mut self, inputs: &[f32]) -> f32 {
        let mut output = 0.0f32;
        for (i, &input) in inputs.iter().enumerate() {
            let gain = self.channels.get(i).copied().unwrap_or(1.0);
            output += input * gain;
        }
        output * self.master / inputs.len() as f32
    }

    /// Mixes multiple stereo signals.
    pub fn mix_stereo(&mut self, left_inputs: &[f32], right_inputs: &[f32]) -> (f32, f32) {
        let left = self.mix(left_inputs);
        let right = self.mix(right_inputs);
        (left, right)
    }

    /// Returns the number of configured input channels.
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new(2)
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
