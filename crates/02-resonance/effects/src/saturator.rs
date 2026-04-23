/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd877c224 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/saturator.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

///
/// presence, and character to audio signals.
use smoothie_core::primitives::Sample;

/// Saturation algorithm type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SaturationType enumeration.
pub enum SaturationType {
    /// Soft clipping (tanh-like smooth saturation).
    Soft,
    /// Hard clipping (abrupt limiting).
    Hard,
    /// Tube emulation (asymmetric even-harmonic saturation).
    Tube,
    /// Tape emulation (magnetic hysteresis approximation).
    Tape,
    /// Foldback distortion (wraps signal around threshold).
    Foldback,
    /// Bitcrusher (sample rate and bit depth reduction).
    Bitcrush,
}

/// Technical implementation of the Saturator structure.
pub struct Saturator {
    drive: f32,
    output_gain: f32,
    mix: f32,
    algorithm: SaturationType,
    /// Bitcrusher bits (for Bitcrush mode).
    bits: u8,
    /// Bitcrusher rate reduction factor.
    rate_reduction: f32,
    /// Bitcrusher sample holder.
    hold_sample: f32,
    hold_counter: f32,
}

impl Saturator {
    /// Initializes a new instance of the associated type.
    pub fn new(algorithm: SaturationType) -> Self {
        Self {
            drive: 1.0,
            output_gain: 1.0,
            mix: 1.0,
            algorithm,
            bits: 16,
            rate_reduction: 1.0,
            hold_sample: 0.0,
            hold_counter: 0.0,
        }
    }

    /// Technical implementation of the set_drive logic.
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive.max(0.0);
    }
    /// Technical implementation of the set_output_gain logic.
    pub fn set_output_gain(&mut self, gain: f32) {
        self.output_gain = gain.max(0.0);
    }
    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
    /// Technical implementation of the set_algorithm logic.
    pub fn set_algorithm(&mut self, algo: SaturationType) {
        self.algorithm = algo;
    }
    /// Technical implementation of the set_bits logic.
    pub fn set_bits(&mut self, bits: u8) {
        self.bits = bits.clamp(1, 32);
    }
    /// Technical implementation of the set_rate_reduction logic.
    pub fn set_rate_reduction(&mut self, factor: f32) {
        self.rate_reduction = factor.max(1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let driven = input * self.drive;

        let saturated = match self.algorithm {
            SaturationType::Soft => self.soft_clip(driven),
            SaturationType::Hard => self.hard_clip(driven),
            SaturationType::Tube => self.tube_saturate(driven),
            SaturationType::Tape => self.tape_saturate(driven),
            SaturationType::Foldback => self.foldback(driven),
            SaturationType::Bitcrush => self.bitcrush(driven),
        };

        let output = input * (1.0 - self.mix) + saturated * self.mix;
        output * self.output_gain
    }

    #[inline]
    /// Technical implementation of the soft_clip logic.
    fn soft_clip(&self, x: f32) -> f32 {
        smoothie_core::math::tanh_approx(x)
    }

    #[inline]
    /// Technical implementation of the hard_clip logic.
    fn hard_clip(&self, x: f32) -> f32 {
        x.max(-1.0).min(1.0)
    }

    /// Technical implementation of the tube_saturate logic.
    fn tube_saturate(&self, x: f32) -> f32 {
        // Asymmetric: positive half gets more saturation
        if x >= 0.0 {
            1.0 - (-3.0 * x).exp_tube()
        } else {
            -(1.0 - (3.0 * x).exp_tube())
        }
    }

    /// Technical implementation of the tape_saturate logic.
    fn tape_saturate(&self, x: f32) -> f32 {
        // Soft saturation with slight compression
        let sign = if x >= 0.0 { 1.0 } else { -1.0 };
        let abs_x = x.abs();
        if abs_x < 0.5 {
            x
        } else {
            sign * (0.5 + (abs_x - 0.5) / (1.0 + (abs_x - 0.5) * (abs_x - 0.5)))
        }
    }

    /// Technical implementation of the foldback logic.
    fn foldback(&self, x: f32) -> f32 {
        let threshold = 1.0;
        let mut y = x;
        while y > threshold || y < -threshold {
            if y > threshold {
                y = threshold - (y - threshold);
            }
            if y < -threshold {
                y = -threshold - (y + threshold);
            }
        }
        y
    }

    /// Technical implementation of the bitcrush logic.
    fn bitcrush(&mut self, x: f32) -> f32 {
        // Rate reduction
        self.hold_counter += 1.0;
        if self.hold_counter >= self.rate_reduction {
            self.hold_counter = 0.0;
            // Bit depth reduction
            let levels = (1u32 << self.bits) as f32;
            self.hold_sample = (x * levels).floor_approx() / levels;
        }
        self.hold_sample
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.hold_sample = 0.0;
        self.hold_counter = 0.0;
    }
}

impl Default for Saturator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(SaturationType::Soft)
    }
}

trait ExpTube {
    /// Technical implementation of the exp_tube logic.
    fn exp_tube(self) -> f32;
}
impl ExpTube for f32 {
    /// Technical implementation of the exp_tube logic.
    fn exp_tube(self) -> f32 {
        smoothie_core::math::exp_approx(self)
    }
}

trait FloorApprox {
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f32;
}
impl FloorApprox for f32 {
    /// Technical implementation of the floor_approx logic.
    fn floor_approx(self) -> f32 {
        smoothie_core::math::floor_approx(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_soft_clip_bounded logic.
    fn test_soft_clip_bounded() {
        let mut sat = Saturator::new(SaturationType::Soft);
        sat.set_drive(10.0);
        let output = sat.process(1.0);
        assert!(output.abs() <= 1.1);
    }

    #[test]
    /// Technical implementation of the test_hard_clip_exact logic.
    fn test_hard_clip_exact() {
        let mut sat = Saturator::new(SaturationType::Hard);
        sat.set_drive(5.0);
        let output = sat.process(0.5);
        assert!(output.abs() <= 1.0);
    }

    #[test]
    /// Technical implementation of the test_foldback_bounded logic.
    fn test_foldback_bounded() {
        let mut sat = Saturator::new(SaturationType::Foldback);
        sat.set_drive(3.0);
        let output = sat.process(0.8);
        assert!(output.abs() <= 1.01);
    }
}
