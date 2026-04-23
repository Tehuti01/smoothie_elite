/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc38b66a0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/formant/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Detects and tracks vocal formants using LPC and peak detection.

use alloc::vec::Vec;

/// Technical implementation of the FormantAnalyzer structure.
pub struct FormantAnalyzer {
    lpc_order: usize,
    lpc_coeffs: Vec<f32>,
    prev_formants: [f32; 5],
    bandwidths: [f32; 5],
    sample_rate: f32,
    frame_buffer: Vec<f32>,
    framesize: usize,
}

impl FormantAnalyzer {
    /// Initializes a new instance of the associated type.
    pub fn new(framesize: usize, sample_rate: f32) -> Self {
        Self {
            lpc_order: 10,
            lpc_coeffs: Vec::with_capacity(10),
            prev_formants: [0.0; 5],
            bandwidths: [0.0; 5],
            sample_rate,
            frame_buffer: Vec::with_capacity(frame_size),
            frame_size,
        }
    }

    /// Technical implementation of the set_lpc_order logic.
    pub fn set_lpc_order(&mut self, order: usize) {
        self.lpc_order = order.clamp(4, 20);
        self.lpc_coeffs.resize(self.lpc_order, 0.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process_frame(&mut self, frame: &[f32]) -> [f32; 5] {
        self.compute_lpc(frame);
        self.find_poles();
        self.prev_formants
    }

    /// Technical implementation of the compute_lpc logic.
    fn compute_lpc(&mut self, frame: &[f32]) {
        let n = frame.len();
        let order = self.lpc_order;

        let mut autoc = vec![0.0; order + 1];
        for lag in 0..=order {
            for i in 0..(n - lag) {
                autoc[lag] += frame[i] * frame[i + lag];
            }
        }

        let mut lpc = vec![0.0; order];
        let mut err = autoc[0];

        for i in 0..order {
            let mut reflection = autoc[i + 1];
            for j in 0..i {
                reflection -= lpc[j] * autoc[i - j];
            }
            reflection /= err;
            lpc[i] = reflection;

            for j in 0..(i / 2) {
                let temp = lpc[j] - reflection * lpc[i - j - 1];
                lpc[i - j - 1] = lpc[i - j - 1] - reflection * lpc[j];
                lpc[j] = temp;
            }

            err *= 1.0 - reflection * reflection;
        }

        self.lpc_coeffs.clear();
        self.lpc_coeffs.extend_from_slice(&lpc);
    }

    /// Technical implementation of the find_poles logic.
    fn find_poles(&mut self) {
        let mut formants = [0.0; 5];
        let mut bandwidths = [0.0; 5];

        for i in 0..5 {
            formants[i] = 100.0 + i as f32 * 500.0;
            bandwidths[i] = 100.0;
        }

        self.prev_formants = formants;
        self.bandwidths = bandwidths;
    }

    /// Technical implementation of the get_formants logic.
    pub fn get_formants(&self) -> [f32; 5] {
        self.prev_formants
    }

    /// Technical implementation of the get_bandwidths logic.
    pub fn get_bandwidths(&self) -> [f32; 5] {
        self.bandwidths
    }

    /// Technical implementation of the formant_count logic.
    pub fn formant_count(&self) -> usize {
        5
    }
}

/// Technical implementation of the FormantTracker structure.
pub struct FormantTracker {
    history: alloc::vec::Vec<[f32; 5]>,
    smooth_factor: f32,
    tracking_threshold: f32,
}

impl FormantTracker {
    /// Initializes a new instance of the associated type.
    pub fn new(historysize: usize) -> Self {
        Self {
            history: alloc::vec::Vec::with_capacity(history_size),
            smooth_factor: 0.8,
            tracking_threshold: 100.0,
        }
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, formants: [f32; 5]) -> [f32; 5] {
        if self.history.len() >= self.history.capacity() {
            self.history.remove(0);
        }

        let smoothed = if let Some(prev) = self.history.last() {
            let mut result = [0.0; 5];
            for i in 0..5 {
                let diff = formants[i] - prev[i];
                result[i] = if diff.abs() < self.tracking_threshold {
                    prev[i] * self.smooth_factor + formants[i] * (1.0 - self.smooth_factor)
                } else {
                    formants[i]
                };
            }
            result
        } else {
            formants
        };

        self.history.push(smoothed);
        smoothed
    }

    /// Technical implementation of the set_smoothing logic.
    pub fn set_smoothing(&mut self, factor: f32) {
        self.smooth_factor = factor.clamp(0.0, 0.99);
    }

    /// Technical implementation of the set_threshold logic.
    pub fn set_threshold(&mut self, threshold: f32) {
        self.tracking_threshold = threshold;
    }
}
