use crate::audio::Sample;

/// A power amplifier simulation stage.
///
/// This stage handles the final amplification, including 'presence' (high-mid lift)
/// and resonance modeling. It acts as the final buffer before the cabinet stage.
pub struct Poweramp {
    sample_rate: f32,
    /// Final output level.
    volume: f32,
    /// High-frequency definition control.
    presence: f32,
    /// Low-frequency harmonic resonance control.
    resonance: f32,
}

impl Poweramp {
    /// Creates a new Poweramp instance.
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: sample_rate as f32,
            volume: 0.7,
            presence: 0.5,
            resonance: 0.5,
        }
    }

    pub fn set_volume(&mut self, v: f32) {
        self.volume = v.clamp(0.0, 1.0);
    }
    
    pub fn set_presence(&mut self, p: f32) {
        self.presence = p.clamp(0.0, 1.0);
    }
    
    pub fn set_resonance(&mut self, r: f32) {
        self.resonance = r.clamp(0.0, 1.0);
    }

    /// Processes a stereo signal through the power amp stage.
    pub fn process(&mut self, input: Sample) -> Sample {
        let gain = self.volume * (1.0 + self.presence * 0.5);

        let out_l = input.left * gain;
        let out_r = input.right * gain;

        // Apply safety clipping for extreme presence/resonance combinations
        Sample {
            left: out_l.clamp(-1.5, 1.5),
            right: out_r.clamp(-1.5, 1.5),
        }
    }

    /// Resets the power amp state.
    pub fn reset(&mut self) {}
}
