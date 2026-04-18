use crate::audio::Sample;

/// A real-time Impulse Response (IR) loader for cabinet simulation.
///
/// This module performs time-domain convolution to apply speaker cabinet 
/// characteristics to an audio signal. It uses a fixed-size FIR window (2048 
/// samples) to ensure high-quality response while maintaining real-time 
/// performance on Apple Silicon.
pub struct IRLoader {
    sample_rate: f32,
    /// The source impulse response data.
    ir_data: Vec<f32>,
    /// Circular buffer size for delay lines.
    buffer_size: usize,
    /// Per-channel convolution buffers [Left, Right].
    buffers: [Vec<f32>; 2],
    /// Current write position in the circular buffers.
    write_index: usize,
    /// Dry/Wet mix ratio (0.0 to 1.0).
    wet_mix: f32,
}

impl IRLoader {
    /// Creates a new IRLoader with a 2-second buffer capacity.
    pub fn new(sample_rate: u32) -> Self {
        let buffer_size = 2048; // Optimized for real-time stability
        Self {
            sample_rate: sample_rate as f32,
            ir_data: vec![1.0; 1], // Default to pass-through
            buffer_size,
            buffers: [vec![0.0; buffer_size], vec![0.0; buffer_size]],
            write_index: 0,
            wet_mix: 1.0,
        }
    }

    /// Loads new IR data into the engine.
    pub fn load_ir(&mut self, ir_data: Vec<f32>) {
        self.ir_data = ir_data;
    }

    pub fn set_wet_mix(&mut self, m: f32) {
        self.wet_mix = m.clamp(0.0, 1.0);
    }

    /// Processes a stereo sample through the convolution engine.
    pub fn process(&mut self, input: Sample) -> Sample {
        // Store inputs in circular buffers
        self.buffers[0][self.write_index] = input.left;
        self.buffers[1][self.write_index] = input.right;

        let mut out_l = 0.0_f32;
        let mut out_r = 0.0_f32;
        
        // Convolution window limited to 2048 for real-time performance (~46ms @ 44.1k)
        let ir_len = self.ir_data.len().min(self.buffer_size);

        for i in 0..ir_len {
            // Read backwards from write index
            let idx = if self.write_index >= i {
                self.write_index - i
            } else {
                self.buffer_size + self.write_index - i
            };

            let ir_val = self.ir_data[i];
            out_l += self.buffers[0][idx] * ir_val;
            out_r += self.buffers[1][idx] * ir_val;
        }

        // Advance circular buffer
        self.write_index = (self.write_index + 1) % self.buffer_size;

        Sample {
            left: input.left * (1.0 - self.wet_mix) + out_l * self.wet_mix,
            right: input.right * (1.0 - self.wet_mix) + out_r * self.wet_mix,
        }
    }

    /// Clears internal buffers and resets the write pointer.
    pub fn reset(&mut self) {
        self.buffers[0].fill(0.0);
        self.buffers[1].fill(0.0);
        self.write_index = 0;
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
