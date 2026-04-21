/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5a5ce54d | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/waveguide.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// to physically model realistic plucked, bowed, and struck strings.
use alloc::vec::Vec;
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;

/// Technical implementation of the FractionalDelay structure.
pub struct FractionalDelay {
    buffer: Vec<f32>,
    write_idx: usize,
    max_delay: f32,
    current_delay: f32,
}

impl FractionalDelay {
    /// Initializes a new instance of the associated type.
    pub fn new(max_delay_samples: usize) -> Self {
        Self {
            buffer: vec![0.0; max_delay_samples],
            write_idx: 0,
            max_delay: max_delay_samples as f32,
            current_delay: 0.0,
        }
    }

    /// Technical implementation of the set_delay logic.
    pub fn set_delay(&mut self, delay_samples: f32) {
        self.current_delay = delay_samples.clamp(1.0, self.max_delay - 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let len = self.buffer.len();

        // Calculate read position
        let read_pos = (self.write_idx as f32 - self.current_delay + len as f32) % (len as f32);

        let idx0 = read_pos as usize;
        let idx1 = (idx0 + 1) % len;
        let frac = read_pos - idx0 as f32;

        // Linear interpolation (cubic could be used for higher quality)
        let output = self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac;

        // Write input
        self.buffer[self.write_idx] = input;
        self.write_idx = (self.write_idx + 1) % len;

        output
    }
}

/// Technical implementation of the PluckedString structure.
pub struct PluckedString {
    delay_line: FractionalDelay,
    dampening_filter: f32,
    last_output: f32,
    feedback: f32,
    sample_rate: f32,
}

impl PluckedString {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, lowest_freq: f32) -> Self {
        let max_samples = (sample_rate / lowest_freq) as usize + 10;
        Self {
            delay_line: FractionalDelay::new(max_samples),
            dampening_filter: 0.5,
            last_output: 0.0,
            feedback: 0.99,
            sample_rate,
        }
    }

    /// Excite the string with an impulse (noise burst or specialized excitation signal)
    pub fn pluck(&mut self, frequency: f32, velocity: f32) {
        let delay_samples = self.sample_rate / frequency;
        self.delay_line.set_delay(delay_samples);

        // Inject energy into the delay line
        let samples_to_fill = delay_samples as usize;
        for _ in 0..samples_to_fill {
            // Generate deterministic pseudo-random noise burst for excitation
            let noise =
                (sine_approx((self.delay_line.write_idx as f32) * 0.1) * 2.0 - 1.0) * velocity;
            let _ = self.delay_line.process(noise);
        }
    }

    /// Technical implementation of the set_dampening logic.
    pub fn set_dampening(&mut self, filter_coeff: f32) {
        self.dampening_filter = filter_coeff.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> Sample {
        // Read delayed sample
        let delayed = self.delay_line.process(0.0); // Pass 0 as input; feedback handled explicitly

        // One-pole lowpass filter for dampening
        let filtered =
            delayed * (1.0 - self.dampening_filter) + self.last_output * self.dampening_filter;
        self.last_output = filtered;

        // Route filtered sample back into the delay line
        let input_to_delay = filtered * self.feedback;

        // Overwrite the most recent 0.0 input with the feedback
        let len = self.delay_line.buffer.len();
        let feedback_idx = (self.delay_line.write_idx + len - 1) % len;
        self.delay_line.buffer[feedback_idx] = input_to_delay;

        filtered
    }
}
