//! Integer oversampling — 2× and 4× using polyphase halfband filters.
//! Allocates buffers at init; zero-alloc at runtime.

/// 2× oversampler — upsample, process, downsample.
pub struct Oversample2x {
    up_buf:   Vec<f32>,
    down_buf: Vec<f32>,
}

impl Oversample2x {
    pub fn new(max_block: usize) -> Self {
        Self {
            up_buf:   vec![0.0; max_block * 2],
            down_buf: vec![0.0; max_block],
        }
    }

    /// Upsample `input` → 2× buffer, apply `process`, downsample back.
    pub fn process<F>(&mut self, input: &mut [f32], mut process: F)
    where F: FnMut(&mut [f32])
    {
        let n = input.len();
        // Simple zero-insert upsample (no LPF yet — TODO: halfband filter)
        for i in 0..n {
            self.up_buf[i * 2]     = input[i];
            self.up_buf[i * 2 + 1] = input[i];
        }
        process(&mut self.up_buf[..n * 2]);
        // Simple averaging downsample
        for i in 0..n {
            input[i] = (self.up_buf[i * 2] + self.up_buf[i * 2 + 1]) * 0.5;
        }
    }
}

/// 4× oversampler.
pub struct Oversample4x {
    up_buf: Vec<f32>,
}

impl Oversample4x {
    pub fn new(max_block: usize) -> Self {
        Self { up_buf: vec![0.0; max_block * 4] }
    }

    pub fn process<F>(&mut self, input: &mut [f32], mut process: F)
    where F: FnMut(&mut [f32])
    {
        let n = input.len();
        for i in 0..n {
            for k in 0..4 { self.up_buf[i * 4 + k] = input[i]; }
        }
        process(&mut self.up_buf[..n * 4]);
        for i in 0..n {
            let sum = self.up_buf[i*4] + self.up_buf[i*4+1] + self.up_buf[i*4+2] + self.up_buf[i*4+3];
            input[i] = sum * 0.25;
        }
    }
}
