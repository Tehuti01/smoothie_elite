/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7e0e9c6b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/distortion.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::tanh_approx as core_tanh;
use smoothie_core::primitives::Sample;

/// Technical implementation of the Distortion structure.
pub struct Distortion {
    drive: f32,
    tone: f32,
    output_level: f32,
    // Add simple DC blocker for asymmetric distortion
    prev_input: f32,
    prev_output: f32,
}

impl Distortion {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            drive: 1.0,
            tone: 0.5,
            output_level: 0.5,
            prev_input: 0.0,
            prev_output: 0.0,
        }
    }

    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.clamp(0.0, 20.0);
    }
    /// Technical implementation of the set_tone logic.
    pub fn set_tone(&mut self, tone: f32) {
        self.tone = tone.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_output_level logic.
    pub fn set_output_level(&mut self, level: f32) {
        self.output_level = level.clamp(0.0, 1.0);
    }

    /// Process sample through silicon-stable saturation
    pub fn process(&mut self, input: Sample) -> Sample {
        let driven = input * self.drive;

        // Silicon Saturation: Tanh-based with AA-Approximation
        let saturated = core_tanh(driven);

        // Simple 1-pole DC blocker (10Hz alignment) to prevent bias drift
        let dc_blocked = saturated - self.prev_input + 0.995 * self.prev_output;
        self.prev_input = saturated;
        self.prev_output = dc_blocked;

        dc_blocked * self.output_level
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.prev_input = 0.0;
        self.prev_output = 0.0;
    }
}

/// Technical implementation of the Waveshaper structure.
pub struct Waveshaper {
    shape: WaveshaperShape,
}

#[derive(Clone, Copy)]
/// Technical implementation of the WaveshaperShape enumeration.
pub enum WaveshaperShape {
    HardClip,
    SoftClip,
    Tanh,
    Asymmetric,
}

impl Waveshaper {
    /// Initializes a new instance of the associated type.
    pub fn new(shape: WaveshaperShape) -> Self {
        Self { shape }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&self, input: Sample) -> Sample {
        match self.shape {
            WaveshaperShape::HardClip => input.clamp(-1.0, 1.0),
            WaveshaperShape::SoftClip => {
                let x = input.clamp(-1.5, 1.5);
                1.5 * x - 0.5 * x * x * x
            }
            WaveshaperShape::Tanh => core_tanh(input),
            WaveshaperShape::Asymmetric => {
                if input > 0.0 {
                    core_tanh(input * 2.0) * 0.5
                } else {
                    core_tanh(input)
                }
            }
        }
    }
}

impl Default for Distortion {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
impl Default for Waveshaper {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(WaveshaperShape::SoftClip)
    }
}
