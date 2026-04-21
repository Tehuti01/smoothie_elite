/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x331adc00 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/synthesis/waveguide/waveguide1d.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Uses the bidirectional delay line method with proper boundary conditions.

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::constants::TAU;
use smoothie_core::math::{exp_approx, sine_approx};
use smoothie_core::primitives::Sample;

/// Technical implementation of the Waveguide1D structure.
pub struct Waveguide1D {
    forward_delay: Vec<f32>,
    backward_delay: Vec<f32>,
    forward_head: usize,
    backward_head: usize,
    length: f32,
    sample_rate: f32,
    boundary_left: BoundaryType,
    boundary_right: BoundaryType,
    loss: f32,
    dispersion: f32,
}

impl Waveguide1D {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, max_length_samples: usize) -> Self {
        Self {
            forward_delay: vec![0.0; max_length_samples],
            backward_delay: vec![0.0; max_length_samples],
            forward_head: 0,
            backward_head: 0,
            length: max_length_samples as f32,
            sample_rate,
            boundary_left: BoundaryType::Fixed,
            boundary_right: BoundaryType::Fixed,
            loss: 0.999,
            dispersion: 0.0,
        }
    }

    /// Technical implementation of the set_length logic.
    pub fn set_length(&mut self, length: f32) {
        self.length = length.clamp(2.0, self.forward_delay.len() as f32 - 2.0);
    }

    /// Technical implementation of the set_boundary_left logic.
    pub fn set_boundary_left(&mut self, b: BoundaryType) {
        self.boundary_left = b;
    }

    /// Technical implementation of the set_boundary_right logic.
    pub fn set_boundary_right(&mut self, b: BoundaryType) {
        self.boundary_right = b;
    }

    /// Technical implementation of the set_loss logic.
    pub fn set_loss(&mut self, loss: f32) {
        self.loss = loss.clamp(0.9, 1.0);
    }

    /// Technical implementation of the set_dispersion logic.
    pub fn set_dispersion(&mut self, disp: f32) {
        self.dispersion = disp.clamp(0.0, 0.5);
    }

    /// Technical implementation of the excite_at_left logic.
    pub fn excite_at_left(&mut self, input: Sample, position: f32) {
        let pos = (position * self.length) as usize;
        self.forward_delay[(self.forward_head + pos) % self.forward_delay.len()] = input;
    }

    /// Technical implementation of the excite_at_right logic.
    pub fn excite_at_right(&mut self, input: Sample, position: f32) {
        let pos = (position * self.length) as usize;
        self.backward_delay[(self.backward_head + pos) % self.backward_delay.len()] = input;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> Sample {
        let len = self.forward_delay.len();

        let read_f = (self.forward_head as f32 - self.length + len as f32) as usize % len;
        let read_b = (self.backward_head as f32 - self.length + len as f32) as usize % len;

        let output_f = self.forward_delay[read_f];
        let output_b = self.backward_delay[read_b];

        let output = (output_f + output_b) * self.loss;

        let next_f = self.forward_head;
        let next_b = self.backward_head;

        let left_reflection = match self.boundary_left {
            BoundaryType::Fixed => -output_b,
            BoundaryType::Free => output_b,
            BoundaryType::Input => output_f,
        };

        let right_reflection = match self.boundary_right {
            BoundaryType::Fixed => -output_f,
            BoundaryType::Free => output_f,
            BoundaryType::Input => output_b,
        };

        self.forward_delay[next_f] = left_reflection;
        self.backward_delay[next_b] = right_reflection;

        self.forward_head = (self.forward_head + 1) % len;
        self.backward_head = (self.backward_head + 1) % len;

        output
    }

    /// Technical implementation of the get_state logic.
    pub fn get_state(&self) -> (Sample, Sample) {
        let len = self.forward_delay.len();
        let idx = (self.forward_head as usize) % len;
        (self.forward_delay[idx], self.backward_delay[idx])
    }
}

/// Technical implementation of the BoundaryType enumeration.
pub enum BoundaryType {
    Fixed,
    Free,
    Input,
}

/// Technical implementation of the BoreWaveguide structure.
pub struct BoreWaveguide {
    waveguide: Waveguide1D,
    junction: f32,
    radius: f32,
}

impl BoreWaveguide {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, length_samples: usize) -> Self {
        Self {
            waveguide: Waveguide1D::new(sample_rate, length_samples),
            junction: 0.0,
            radius: 0.01,
        }
    }

    /// Technical implementation of the set_bore_radius logic.
    pub fn set_bore_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input_left: Sample, input_right: Sample) -> (Sample, Sample) {
        self.waveguide.excite_at_left(input_left, 0.0);
        self.waveguide.excite_at_right(input_right, 1.0);

        let output = self.waveguide.process();

        (output, output)
    }
}

/// Technical implementation of the BarWaveguide structure.
pub struct BarWaveguide {
    delay_line: Vec<f32>,
    pointers: [usize; 2],
    length: f32,
    sample_rate: f32,
    flexural_rigidity: f32,
    damping: f32,
}

impl BarWaveguide {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, fundamental_freq: f32, n_modes: usize) -> Self {
        let length = (sample_rate / fundamental_freq) as usize;
        let delay_line = vec![0.0; length * 2 + 1];

        Self {
            delay_line,
            pointers: [0, length],
            length: length as f32,
            sample_rate,
            flexural_rigidity: 0.001,
            damping: 0.999,
        }
    }

    /// Technical implementation of the set_flexural_rigidity logic.
    pub fn set_flexural_rigidity(&mut self, rigidity: f32) {
        self.flexural_rigidity = rigidity.clamp(0.0, 0.1);
    }

    /// Technical implementation of the set_damping logic.
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.9, 1.0);
    }

    /// Technical implementation of the strike logic.
    pub fn strike(&mut self, position: f32, velocity: Sample) {
        let pos = ((position * self.length) as usize).min(self.delay_line.len() - 1);
        self.delay_line[pos] += velocity;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> Sample {
        let len = self.delay_line.len();

        let idx0 = self.pointers[0];
        let idx1 = self.pointers[1];

        let sample0 = self.delay_line[idx0];
        let sample1 = self.delay_line[idx1];

        let output = (sample0 + sample1) * self.damping;

        self.delay_line[idx0] = sample1;

        self.pointers[0] = (idx0 + 1) % len;
        self.pointers[1] = (idx1 + 1) % len;

        output
    }
}
