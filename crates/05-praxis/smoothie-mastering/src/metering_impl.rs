/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xba05026c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-mastering/src/metering_impl.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// Loudness and peak metering

use alloc::vec::Vec;

#[repr(align(64))]
/// Technical implementation of the LoudnessMeter structure.
pub struct LoudnessMeter {
    block_buffer: Vec<f32>,
    block_size: usize,
    sample_rate: f32,
    lufs_accum: f32,
    block_count: u64,
    peak_sample: f32,
    integrated_lufs: f32,
    short_term_lufs: f32,
    momentary_lufs: f32,
    gating_blocks: Vec<f32>,
}

impl LoudnessMeter {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let block_size = (sample_rate * 0.4) as usize;
        block_size.next_power_of_two();

        Self {
            block_buffer: Vec::with_capacity(block_size),
            block_size,
            sample_rate,
            lufs_accum: 0.0,
            block_count: 0,
            peak_sample: 0.0,
            integrated_lufs: -23.0,
            short_term_lufs: -23.0,
            momentary_lufs: -23.0,
            gating_blocks: Vec::with_capacity(1000),
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample: f32) {
        self.block_buffer.push(sample.abs());
        self.peak_sample = self.peak_sample.max(sample.abs());

        if self.block_buffer.len() >= self.block_size {
            self.process_block();
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_block(&mut self) {
        let sum_sq: f32 = self.block_buffer.iter().map(|&s| s * s).sum();
        let rms = (sum_sq / self.block_buffer.len() as f32).sqrt();

        let loudness = if rms > 1e-9 {
            -0.691 + 10.0 * rms.log10()
        } else {
            -70.0
        };

        self.momentary_lufs = loudness;

        self.gating_blocks.push(loudness);
        if self.gating_blocks.len() > 400 {
            self.gating_blocks.remove(0);
        }

        let short_term: f32 = self.gating_blocks.iter().rev().take(30).sum::<f32>() / 30.0;
        self.short_term_lufs = short_term;

        self.lufs_accum += loudness;
        self.block_count += 1;

        self.integrated_lufs = self.lufs_accum / self.block_count as f32;

        self.block_buffer.clear();
    }

    /// Technical implementation of the momentary logic.
    pub fn momentary(&self) -> f32 {
        self.momentary_lufs
    }

    /// Technical implementation of the short_term logic.
    pub fn short_term(&self) -> f32 {
        self.short_term_lufs
    }

    /// Technical implementation of the integrated logic.
    pub fn integrated(&self) -> f32 {
        self.integrated_lufs
    }

    /// Technical implementation of the peak logic.
    pub fn peak(&self) -> f32 {
        self.peak_sample
    }

    /// Technical implementation of the peak_db logic.
    pub fn peak_db(&self) -> f32 {
        if self.peak_sample > 1e-9 {
            20.0 * self.peak_sample.log10()
        } else {
            -120.0
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.block_buffer.clear();
        self.lufs_accum = 0.0;
        self.block_count = 0;
        self.peak_sample = 0.0;
        self.integrated_lufs = -23.0;
        self.short_term_lufs = -23.0;
        self.momentary_lufs = -23.0;
        self.gating_blocks.clear();
    }
}

#[repr(align(64))]
/// Technical implementation of the RmsMeter structure.
pub struct RmsMeter {
    accumulator: f32,
    count: u64,
    window_samples: usize,
}

impl RmsMeter {
    /// Initializes a new instance of the associated type.
    pub fn new(window_ms: f32, sample_rate: f32) -> Self {
        let window_samples = ((window_ms / 1000.0) * sample_rate) as usize;

        Self {
            accumulator: 0.0,
            count: 0,
            window_samples,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, sample: f32) {
        self.accumulator += sample * sample;
        self.count += 1;

        if self.count > self.window_samples as u64 {
            self.accumulator *= 0.999;
            self.count = self.window_samples as u64;
        }
    }

    /// Technical implementation of the rms logic.
    pub fn rms(&self) -> f32 {
        (self.accumulator / self.count as f32).sqrt()
    }

    /// Technical implementation of the rms_db logic.
    pub fn rms_db(&self) -> f32 {
        let r = self.rms();
        if r > 1e-9 {
            20.0 * r.log10()
        } else {
            -120.0
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.accumulator = 0.0;
        self.count = 0;
    }
}
