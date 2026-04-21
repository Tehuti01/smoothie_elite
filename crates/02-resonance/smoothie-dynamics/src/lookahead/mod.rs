/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9d0d9fb3 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/lookahead/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Provides zero attack time while preventing inter-sample overshoots.

/// Technical implementation of the LookaheadBuffer structure.
pub struct LookaheadBuffer {
    buffer: alloc::vec::Vec<f32>,
    write_pos: usize,
    lookahead_samples: usize,
}

impl LookaheadBuffer {
    /// Initializes a new instance of the associated type.
    pub fn new(lookahead_ms: f32, sample_rate: f32) -> Self {
        let lookahead_samples = ((lookahead_ms / 1000.0) * sample_rate) as usize;
        let lookahead_samples = lookahead_samples.next_power_of_two().max(64);

        Self {
            buffer: alloc::vec::Vec::with_capacity(lookahead_samples),
            write_pos: 0,
            lookahead_samples,
        }
    }

    /// Technical implementation of the set_lookahead logic.
    pub fn set_lookahead(&mut self, lookahead_ms: f32, sample_rate: f32) {
        let samples = ((lookahead_ms / 1000.0) * sample_rate) as usize;
        self.lookahead_samples = samples.next_power_of_two().max(64);
        self.buffer.resize(self.lookahead_samples, 0.0);
        self.write_pos = 0;
    }

    /// Technical implementation of the write logic.
    pub fn write(&mut self, sample: f32) {
        if self.buffer.len() < self.lookahead_samples {
            self.buffer.push(sample);
        } else {
            self.buffer[self.write_pos] = sample;
            self.write_pos = (self.write_pos + 1) & (self.lookahead_samples - 1);
        }
    }

    /// Technical implementation of the read logic.
    pub fn read(&self) -> f32 {
        if self.buffer.len() < self.lookahead_samples {
            self.buffer.last().copied().unwrap_or(0.0)
        } else {
            let read_pos =
                (self.write_pos + self.lookahead_samples - 1) & (self.lookahead_samples - 1);
            self.buffer[read_pos]
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32, limiter_fn: impl Fn(f32) -> f32) -> f32 {
        self.write(input);
        let lookahead_val = self.read();
        limiter_fn(lookahead_val)
    }
}

/// Technical implementation of the LookaheadLimiter structure.
pub struct LookaheadLimiter {
    buffer: alloc::vec::Vec<f32>,
    write_pos: usize,
    frame_size: usize,
    gain_reduction: f32,
}

impl LookaheadLimiter {
    /// Initializes a new instance of the associated type.
    pub fn new(lookahead_ms: f32, sample_rate: f32) -> Self {
        let frame_size = ((lookahead_ms / 1000.0) * sample_rate) as usize;
        let frame_size = frame_size.next_power_of_two().max(64);

        Self {
            buffer: alloc::vec::Vec::with_capacity(frame_size),
            write_pos: 0,
            frame_size,
            gain_reduction: 1.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32, threshold: f32) -> f32 {
        if self.buffer.len() < self.frame_size {
            self.buffer.push(input);
            return input;
        }

        let prev_peak = self.find_peak();

        self.buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) & (self.frame_size - 1);

        if prev_peak > threshold {
            self.gain_reduction = threshold / prev_peak;
        } else {
            self.gain_reduction += (1.0 - self.gain_reduction) * 0.01;
        }

        input * self.gain_reduction
    }

    /// Technical implementation of the find_peak logic.
    fn find_peak(&self) -> f32 {
        self.buffer.iter().fold(0.0_f32, |p, &s| p.max(s.abs()))
    }

    /// Technical implementation of the gain_reduction_db logic.
    pub fn gain_reduction_db(&self) -> f32 {
        if self.gain_reduction > 0.0 {
            20.0 * self.gain_reduction.log10()
        } else {
            -80.0
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.buffer.resize(self.frame_size, 0.0);
        self.write_pos = 0;
        self.gain_reduction = 1.0;
    }
}

/// Technical implementation of the PeakHold structure.
pub struct PeakHold {
    peak: f32,
    hold_counter: u32,
    hold_samples: u32,
    decay_rate: f32,
}

impl PeakHold {
    /// Initializes a new instance of the associated type.
    pub fn new(hold_ms: f32, sample_rate: f32) -> Self {
        let hold_samples = ((hold_ms / 1000.0) * sample_rate) as u32;

        Self {
            peak: 0.0,
            hold_counter: 0,
            hold_samples: hold_samples.max(1),
            decay_rate: 0.999,
        }
    }

    /// Technical implementation of the set_hold logic.
    pub fn set_hold(&mut self, hold_ms: f32, sample_rate: f32) {
        self.hold_samples = ((hold_ms / 1000.0) * sample_rate) as u32;
    }

    /// Technical implementation of the set_decay logic.
    pub fn set_decay(&mut self, rate: f32) {
        self.decay_rate = rate.clamp(0.9, 0.9999);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();

        if abs_input > self.peak {
            self.peak = abs_input;
            self.hold_counter = self.hold_samples;
        } else if self.hold_counter > 0 {
            self.hold_counter -= 1;
        } else {
            self.peak *= self.decay_rate;
        }

        self.peak.min(input.abs())
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.peak = 0.0;
        self.hold_counter = 0;
    }
}
