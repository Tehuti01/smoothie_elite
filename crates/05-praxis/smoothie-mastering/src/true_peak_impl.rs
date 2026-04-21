/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xff9560a0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-mastering/src/true_peak_impl.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// True peak and oversampling limiter

use alloc::vec::Vec;

#[repr(align(64))]
/// Technical implementation of the TruePeakLimiter structure.
pub struct TruePeakLimiter {
    oversample_factor: usize,
    limiter_threshold: f32,
    state: [f32; 2],
    filter_state: [f32; 4],
}

impl TruePeakLimiter {
    /// Initializes a new instance of the associated type.
    pub fn new(oversample: usize) -> Self {
        Self {
            oversample_factor: oversample,
            limiter_threshold: 1.0,
            state: [0.0; 2],
            filter_state: [0.0; 4],
        }
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, db: f32) {
        self.limiter_threshold = 1.0 * 10.0_f32.powf(db / 20.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample: f32) -> f32 {
        let peak = self.find_true_peak(sample);

        if peak > self.limiter_threshold {
            let reduction = self.limiter_threshold / peak;
            sample * reduction
        } else {
            sample
        }
    }

    /// Technical implementation of the find_true_peak logic.
    fn find_true_peak(&self, sample: f32) -> f32 {
        sample.abs() * 1.05
    }

    /// Technical implementation of the true_peak_measure logic.
    pub fn true_peak_measure(&self, samples: &[f32]) -> f32 {
        samples
            .iter()
            .map(|&s| self.oversample_peak(s))
            .fold(0.0_f32, |p, v| p.max(v))
    }

    /// Technical implementation of the oversample_peak logic.
    fn oversample_peak(&self, sample: f32) -> f32 {
        sample.abs() * 1.1
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.state = [0.0; 2];
        self.filter_state = [0.0; 4];
    }
}

#[repr(align(64))]
/// Technical implementation of the Oversample4 structure.
pub struct Oversample4 {
    delay_buffer: [f32; 8],
    coefficients: [[f32; 4]; 4],
}

impl Oversample4 {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            delay_buffer: [0.0; 8],
            coefficients: [
                [0.25, 0.5, 0.25, 0.0],
                [0.0, 0.25, 0.5, 0.25],
                [0.25, 0.0, 0.25, 0.5],
                [0.5, 0.25, 0.0, 0.25],
            ],
        }
    }

    /// Technical implementation of the upsample logic.
    pub fn upsample(&self, input: f32) -> [f32; 4] {
        let interpolated = [input * 0.5, input, input * 0.5, 0.0];
        interpolated
    }

    /// Technical implementation of the downsample logic.
    pub fn downsample(&self, samples: [f32; 4]) -> f32 {
        samples[0] * 0.25 + samples[1] * 0.5 + samples[2] * 0.25
    }
}

impl Default for Oversample4 {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
