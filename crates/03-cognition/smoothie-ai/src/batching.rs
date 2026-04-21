/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x58bcbf1b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-ai/src/batching.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::math::FloatExt;

/// Technical implementation of the BatchBuffer structure.
pub struct BatchBuffer {
    pub batch_size: usize,
    pub input_size: usize,
    pub data: Vec<f32>,
    pub valid: Vec<bool>,
    head: usize,
    tail: usize,
    count: usize,
}

impl BatchBuffer {
    /// Initializes a new instance of the associated type.
    pub fn new(batch_size: usize, input_size: usize) -> Self {
        Self {
            batch_size,
            input_size,
            data: vec![0.0; batch_size * input_size],
            valid: vec![false; batch_size],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Technical implementation of the push logic.
    pub fn push(&mut self, input: &[f32]) -> bool {
        if self.count >= self.batch_size {
            return false;
        }
        if input.len() != self.input_size {
            return false;
        }

        let idx = self.tail * self.input_size;
        for i in 0..self.input_size {
            self.data[idx + i] = input[i];
        }
        self.valid[self.tail] = true;
        self.tail = (self.tail + 1) % self.batch_size;
        self.count += 1;
        true
    }

    /// Technical implementation of the pop logic.
    pub fn pop(&mut self, output: &mut [f32]) -> bool {
        if self.count == 0 {
            return false;
        }
        if output.len() != self.input_size {
            return false;
        }

        let idx = self.head * self.input_size;
        for i in 0..self.input_size {
            output[i] = self.data[idx + i];
        }
        self.valid[self.head] = false;
        self.head = (self.head + 1) % self.batch_size;
        self.count -= 1;
        true
    }

    /// Technical implementation of the is_full logic.
    pub fn is_full(&self) -> bool {
        self.count >= self.batch_size
    }
    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        for v in self.valid.iter_mut() {
            *v = false;
        }
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }
}

/// Technical implementation of the PingPongBuffer structure.
pub struct PingPongBuffer {
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub using_a: bool,
    pub capacity: usize,
}

impl PingPongBuffer {
    /// Initializes a new instance of the associated type.
    pub fn new(capacity: usize) -> Self {
        Self {
            a: vec![0.0; capacity],
            b: vec![0.0; capacity],
            using_a: true,
            capacity,
        }
    }

    /// Technical implementation of the current logic.
    pub fn current(&mut self) -> &mut [f32] {
        if self.using_a {
            &mut self.a
        } else {
            &mut self.b
        }
    }

    /// Technical implementation of the swap logic.
    pub fn swap(&mut self) {
        self.using_a = !self.using_a;
    }

    /// Technical implementation of the write logic.
    pub fn write(&mut self, data: &[f32]) {
        let cap = self.capacity;
        let buf = self.current();
        let len = data.len().min(cap);
        for i in 0..len {
            buf[i] = data[i];
        }
    }

    /// Technical implementation of the read logic.
    pub fn read(&self) -> &[f32] {
        if self.using_a {
            &self.b
        } else {
            &self.a
        }
    }
}

/// Technical implementation of the CircularAudioBuffer structure.
pub struct CircularAudioBuffer {
    data: Vec<f32>,
    write_pos: usize,
    read_pos: usize,
    pub size: usize,
}

impl CircularAudioBuffer {
    /// Initializes a new instance of the associated type.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
            write_pos: 0,
            read_pos: 0,
            size,
        }
    }

    /// Technical implementation of the write logic.
    pub fn write(&mut self, input: &[f32]) {
        for &sample in input {
            self.data[self.write_pos] = sample;
            self.write_pos = (self.write_pos + 1) % self.size;
        }
    }

    /// Technical implementation of the read logic.
    pub fn read(&mut self, output: &mut [f32], num_samples: usize) {
        for i in 0..num_samples.min(output.len()) {
            output[i] = self.data[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.size;
        }
    }

    /// Technical implementation of the peek logic.
    pub fn peek(&self, offset: usize) -> f32 {
        self.data[(self.read_pos + offset) % self.size]
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.read_pos = self.write_pos;
    }
}

/// Technical implementation of the BatchProcessor structure.
pub struct BatchProcessor {
    batch_size: usize,
    input_size: usize,
    output_size: usize,
    buffer: Vec<f32>,
    temp_output: Vec<f32>,
}

impl BatchProcessor {
    /// Initializes a new instance of the associated type.
    pub fn new(batch_size: usize, input_size: usize, output_size: usize) -> Self {
        Self {
            batch_size,
            input_size,
            output_size,
            buffer: vec![0.0; batch_size * input_size],
            temp_output: vec![0.0; output_size],
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process_batch<F>(&mut self, outputs: &mut [f32], mut callback: F)
    where
        F: FnMut(usize, &[f32], &mut [f32]),
    {
        for b in 0..self.batch_size {
            let input = &self.buffer[b * self.input_size..(b + 1) * self.input_size];
            let out_slice = &mut outputs[b * self.output_size..(b + 1) * self.output_size];
            callback(b, input, out_slice);
        }
    }

    /// Technical implementation of the set_input logic.
    pub fn set_input(&mut self, batch_idx: usize, input: &[f32]) {
        let base = batch_idx * self.input_size;
        for i in 0..input.len().min(self.input_size) {
            self.buffer[base + i] = input[i];
        }
    }
}
