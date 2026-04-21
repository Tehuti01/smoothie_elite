/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x24fb32f0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-granular/src/stretch.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Synchronous overlap-add time stretching with optional pitch shift.

/// Technical implementation of the TimeStretch structure.
pub struct TimeStretch {
    pub window_size: usize,
    pub hop_in: usize,
    pub hop_out: usize,
    pub window_position: usize,
    pub input_buffer: [f32; 16384],
    pub overlap_buffer: [f32; 16384],
    pub buffer_position: usize,
}

impl TimeStretch {
    /// Initializes a new instance of the associated type.
    pub const fn new(window_size: usize) -> Self {
        let hop = window_size / 4;

        Self {
            window_size,
            hop_in: hop,
            hop_out: hop,
            window_position: 0,
            input_buffer: [0.0; 16384],
            overlap_buffer: [0.0; 16384],
            buffer_position: 0,
        }
    }

    /// Technical implementation of the set_stretch_ratio logic.
    pub fn set_stretch_ratio(&mut self, ratio: f32) {
        self.hop_out = ((self.hop_in as f32) / ratio) as usize;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let input_len = input.len();
        let output_len = output.len();

        let mut in_idx = 0;
        let mut out_idx = 0;

        while in_idx + self.window_size <= input_len && out_idx + self.hop_out <= output_len {
            self.window_position = in_idx;

            for i in 0..self.window_size {
                let pos = self.window_position + i;
                if pos < input_len {
                    let win_pos = i as f32 / self.window_size as f32;
                    let window = (win_pos * core::f32::consts::PI).sin();
                    let sample = input[pos] * window;
                    self.overlap_buffer[pos % 16384] += sample;
                }
            }

            for i in 0..self.hop_out {
                let pos = (self.window_position + i) % 16384;
                output[out_idx + i] = self.overlap_buffer[pos];
                self.overlap_buffer[pos] = 0.0;
            }

            in_idx += self.hop_in;
            out_idx += self.hop_out;
        }

        for i in 0..output_len.min(in_idx + self.hop_out) {
            output[i] = self.overlap_buffer[i % 16384];
            self.overlap_buffer[i % 16384] = 0.0;
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.window_position = 0;
        self.buffer_position = 0;
        self.input_buffer = [0.0; 16384];
        self.overlap_buffer = [0.0; 16384];
    }
}
