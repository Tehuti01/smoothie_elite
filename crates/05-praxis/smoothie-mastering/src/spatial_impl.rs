/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x439632c8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-mastering/src/spatial_impl.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// Stereo width and mid-side processing

#[repr(align(64))]
/// Technical implementation of the MidSideCodec structure.
pub struct MidSideCodec {
    width: f32,
    pan: f32,
}

impl MidSideCodec {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            width: 1.0,
            pan: 0.5,
        }
    }

    /// Technical implementation of the set_width logic.
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(0.0, 2.0);
    }

    /// Technical implementation of the set_pan logic.
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(0.0, 1.0);
    }

    /// Technical implementation of the encode logic.
    pub fn encode(&self, left: f32, right: f32) -> (f32, f32) {
        let mid = (left + right) * 0.5;
        let side = (left - right) * 0.5 * self.width;

        (mid, side)
    }

    /// Technical implementation of the decode logic.
    pub fn decode(&self, mid: f32, side: f32) -> (f32, f32) {
        let adjusted_side = side / self.width.max(0.001);
        let left = mid + adjusted_side;
        let right = mid - adjusted_side;

        (left * self.pan, right * self.pan)
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32) {
        let (mid, side) = self.encode(left, right);
        self.decode(mid, side)
    }
}

impl Default for MidSideCodec {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[repr(align(64))]
/// Technical implementation of the StereoWidth structure.
pub struct StereoWidth {
    correlation: f32,
    width: f32,
    user_width: f32,
}

impl StereoWidth {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            correlation: 0.0,
            width: 1.0,
            user_width: 1.0,
        }
    }

    /// Technical implementation of the set_width logic.
    pub fn set_width(&mut self, width: f32) {
        self.user_width = width.clamp(0.0, 2.0);
        self.width = width;
    }

    /// Technical implementation of the measure_correlation logic.
    pub fn measure_correlation(&mut self, left: f32, right: f32) -> f32 {
        self.correlation = if left.abs() > 1e-9 || right.abs() > 1e-9 {
            let dot = left * right;
            let mag = ((left * left + right * right) * 0.5).sqrt();
            if mag > 1e-9 {
                dot / mag
            } else {
                0.0
            }
        } else {
            0.0
        };
        self.correlation
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mid = (left + right) * 0.5;
        let side = (left - right) * 0.5 * self.width;

        let adjusted_left = mid + side;
        let adjusted_right = mid - side;

        (adjusted_left, adjusted_right)
    }

    /// Technical implementation of the correlation_value logic.
    pub fn correlation_value(&self) -> f32 {
        self.correlation
    }

    /// Technical implementation of the width_value logic.
    pub fn width_value(&self) -> f32 {
        self.width
    }
}

impl Default for StereoWidth {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[repr(align(64))]
/// Technical implementation of the CorrelationMeter structure.
pub struct CorrelationMeter {
    buffer: alloc::vec::Vec<f32>,
    buffer_l: alloc::vec::Vec<f32>,
    buffer_r: alloc::vec::Vec<f32>,
    position: usize,
    window_size: usize,
}

impl CorrelationMeter {
    /// Initializes a new instance of the associated type.
    pub fn new(window_samples: usize) -> Self {
        Self {
            buffer: alloc::vec::Vec::with_capacity(window_samples),
            buffer_l: alloc::vec::Vec::with_capacity(window_samples),
            buffer_r: alloc::vec::Vec::with_capacity(window_samples),
            position: 0,
            window_size: window_samples,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, left: f32, right: f32) {
        if self.buffer_l.len() < self.window_size {
            self.buffer_l.push(left);
            self.buffer_r.push(right);
        } else {
            self.buffer_l[self.position] = left;
            self.buffer_r[self.position] = right;
            self.position = (self.position + 1) % self.window_size;
        }
    }

    /// Technical implementation of the correlation logic.
    pub fn correlation(&self) -> f32 {
        if self.buffer_l.is_empty() {
            return 0.0;
        }

        let sum_lr: f32 = self
            .buffer_l
            .iter()
            .zip(self.buffer_r.iter())
            .map(|(&l, &r)| l * r)
            .sum();

        let sum_l2: f32 = self.buffer_l.iter().map(|&l| l * l).sum();
        let sum_r2: f32 = self.buffer_r.iter().map(|&r| r * r).sum();

        let denom = (sum_l2 * sum_r2).sqrt();
        if denom > 1e-9 {
            sum_lr / denom
        } else {
            0.0
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.buffer_l.clear();
        self.buffer_r.clear();
        self.position = 0;
    }
}
