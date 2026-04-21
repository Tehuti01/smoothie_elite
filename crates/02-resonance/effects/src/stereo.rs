/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa555bece | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/stereo.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::constants::TAU;
use smoothie_core::math::sine_approx;
use smoothie_core::prelude::*;
use smoothie_core::primitives::Sample;

///
/// Width of 0.0 = mono, 1.0 = original, 2.0 = extra wide.
/// Technical implementation of the StereoWidener structure.
pub struct StereoWidener {
    width: f32,
}

impl StereoWidener {
    /// Initializes a new instance of the associated type.
    pub fn new(width: f32) -> Self {
        Self {
            width: width.max(0.0),
        }
    }

    /// Technical implementation of the set_width logic.
    pub fn set_width(&mut self, width: f32) {
        self.width = width.max(0.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&self, left: Sample, right: Sample) -> (Sample, Sample) {
        let mid = (left + right) * 0.5;
        let side = (left - right) * 0.5;

        let mid_scaled = mid * (2.0 - self.width);
        let side_scaled = side * self.width;

        (mid_scaled + side_scaled, mid_scaled - side_scaled)
    }
}

/// Technical implementation of the AutoPan structure.
pub struct AutoPan {
    lfo_phase: f32,
    lfo_rate: f32,
    depth: f32,
    sample_rate: f32,
}

impl AutoPan {
    /// Initializes a new instance of the associated type.
    pub fn new(rate: f32, depth: f32, sample_rate: f32) -> Self {
        Self {
            lfo_phase: 0.0,
            lfo_rate: rate,
            depth: depth.clamp(0.0, 1.0),
            sample_rate,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.lfo_rate = rate;
    }
    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        let lfo = sine_approx(self.lfo_phase * TAU);
        self.lfo_phase += self.lfo_rate / self.sample_rate;
        while self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let pan = 0.5 + lfo * self.depth * 0.5;
        let left_gain = 1.0 - pan;
        let right_gain = pan;

        (
            left * left_gain + right * left_gain,
            left * right_gain + right * right_gain,
        )
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.lfo_phase = 0.0;
    }
}

/// Technical implementation of the Tremolo structure.
pub struct Tremolo {
    lfo_phase: f32,
    rate: f32,
    depth: f32,
    sample_rate: f32,
}

impl Tremolo {
    /// Initializes a new instance of the associated type.
    pub fn new(rate: f32, depth: f32, sample_rate: f32) -> Self {
        Self {
            lfo_phase: 0.0,
            rate,
            depth: depth.clamp(0.0, 1.0),
            sample_rate,
        }
    }

    /// Technical implementation of the set_rate logic.
    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }
    /// Technical implementation of the set_depth logic.
    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let lfo = sine_approx(self.lfo_phase * TAU);
        self.lfo_phase += self.rate / self.sample_rate;
        while self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        let modulation = 1.0 - self.depth * 0.5 * (1.0 - lfo);
        input * modulation
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.lfo_phase = 0.0;
    }
}

/// Technical implementation of the MidSide structure.
pub struct MidSide;

impl MidSide {
    #[inline]
    /// Technical implementation of the encode logic.
    pub fn encode(left: f32, right: f32) -> (f32, f32) {
        ((left + right) * 0.5, (left - right) * 0.5)
    }

    #[inline]
    /// Technical implementation of the decode logic.
    pub fn decode(mid: f32, side: f32) -> (f32, f32) {
        (mid + side, mid - side)
    }
}

/// Technical implementation of the StereoBalance structure.
pub struct StereoBalance {
    balance: f32,
    left_gain: f32,
    right_gain: f32,
}

impl StereoBalance {
    /// Initializes a new instance of the associated type.
    pub fn new(_sample_rate: f32) -> Self {
        Self {
            balance: 0.0,
            left_gain: 1.0,
            right_gain: 1.0,
        }
    }

    /// Technical implementation of the set_balance logic.
    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance.clamp(-1.0, 1.0);
        self.update_gains();
    }

    /// Technical implementation of the update_gains logic.
    fn update_gains(&mut self) {
        let b = self.balance;
        if b < 0.0 {
            self.left_gain = 1.0;
            self.right_gain = 1.0 + b;
        } else {
            self.left_gain = 1.0 - b;
            self.right_gain = 1.0;
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (left * self.left_gain, right * self.right_gain)
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: &[Sample], output: &mut [Sample]) {
        for i in 0..input.len().min(output.len() / 2) {
            let (l, r) = self.process(input[i * 2], input[i * 2 + 1]);
            output[i * 2] = l;
            output[i * 2 + 1] = r;
        }
    }
}

/// Technical implementation of the ChannelSwap structure.
pub struct ChannelSwap;

impl ChannelSwap {
    /// Primary real-time signal processing execution block.
    pub fn process(left: Sample, right: Sample) -> (Sample, Sample) {
        (right, left)
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(input: &[Sample], output: &mut [Sample]) {
        for i in 0..input.len().min(output.len() / 2) {
            output[i * 2] = input[i * 2 + 1];
            output[i * 2 + 1] = input[i * 2];
        }
    }
}

/// Technical implementation of the MonoStereo structure.
pub struct MonoStereo {
    mix: f32,
}

impl MonoStereo {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { mix: 0.0 }
    }

    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        let mid = (left + right) * 0.5;
        let out_left = left * (1.0 - self.mix) + mid * self.mix;
        let out_right = right * (1.0 - self.mix) + mid * self.mix;
        (out_left, out_right)
    }
}

/// Technical implementation of the StereoCorrelation structure.
pub struct StereoCorrelation {
    correlation: f32,
    buffer_l: [f32; 4096],
    buffer_r: [f32; 4096],
    pos: usize,
    sample_rate: f32,
}

impl StereoCorrelation {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            correlation: 0.0,
            buffer_l: [0.0; 4096],
            buffer_r: [0.0; 4096],
            pos: 0,
            sample_rate,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, left: Sample, right: Sample) {
        self.buffer_l[self.pos] = left;
        self.buffer_r[self.pos] = right;
        self.pos = (self.pos + 1) % 4096;

        if self.pos == 0 {
            self.calculate_correlation();
        }
    }

    /// Technical implementation of the calculate_correlation logic.
    fn calculate_correlation(&mut self) {
        let mut sum_lr = 0.0;
        let mut sum_l2 = 0.0;
        let mut sum_r2 = 0.0;

        for i in 0..4096 {
            sum_lr += self.buffer_l[i] * self.buffer_r[i];
            sum_l2 += self.buffer_l[i] * self.buffer_l[i];
            sum_r2 += self.buffer_r[i] * self.buffer_r[i];
        }

        let denom = (sum_l2 * sum_r2).sqrt();
        if denom > 0.0001 {
            self.correlation = sum_lr / denom;
        }
    }

    /// Technical implementation of the get_correlation logic.
    pub fn get_correlation(&self) -> f32 {
        self.correlation
    }
}

impl Default for StereoBalance {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for MonoStereo {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
impl Default for StereoCorrelation {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_stereo_widener_mono logic.
    fn test_stereo_widener_mono() {
        let widener = StereoWidener::new(0.0);
        let (l, r) = widener.process(0.5, -0.3);
        assert!(
            (l - r).abs() < 0.01,
            "Mono mode should produce identical L/R"
        );
    }

    #[test]
    /// Technical implementation of the test_stereo_widener_unity logic.
    fn test_stereo_widener_unity() {
        let widener = StereoWidener::new(1.0);
        let (l, r) = widener.process(0.8, 0.2);
        assert!((l - 0.8).abs() < 0.01);
        assert!((r - 0.2).abs() < 0.01);
    }

    #[test]
    /// Technical implementation of the test_mid_side_roundtrip logic.
    fn test_mid_side_roundtrip() {
        let (mid, side) = MidSide::encode(0.7, 0.3);
        let (l, r) = MidSide::decode(mid, side);
        assert!((l - 0.7).abs() < 0.001);
        assert!((r - 0.3).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_tremolo_modulates logic.
    fn test_tremolo_modulates() {
        let mut trem = Tremolo::new(5.0, 1.0, 44100.0);
        let mut min_val = f32::MAX;
        let mut max_val = f32::MIN;
        for _ in 0..44100 {
            let v = trem.process(1.0);
            if v < min_val {
                min_val = v;
            }
            if v > max_val {
                max_val = v;
            }
        }
        assert!(max_val > 0.9);
        assert!(min_val < 0.5);
    }
}
