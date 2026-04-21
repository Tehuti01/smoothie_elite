/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xad286844 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/frequency_display.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// Advanced spectrum display with peak hold and frequency scaling

use alloc::vec::Vec;

#[repr(align(64))]
/// Technical implementation of the SpectrumDisplay structure.
pub struct SpectrumDisplay {
    bins: Vec<DisplayBin>,
    peak_hold: Vec<f32>,
    peak_decay: f32,
    peak_hold_time: f32,
    peak_counter: u32,
    min_db: f32,
    max_db: f32,
}

impl SpectrumDisplay {
    /// Initializes a new instance of the associated type.
    pub fn new(bin_count: usize) -> Self {
        let mut bins = Vec::with_capacity(bin_count);
        for i in 0..bin_count {
            bins.push(DisplayBin {
                frequency: 0.0,
                magnitude: -120.0,
                smoothed: -120.0,
            });
        }

        Self {
            bins,
            peak_hold: vec![-120.0; bin_count],
            peak_decay: 0.995,
            peak_hold_time: 1.5,
            peak_counter: 0,
            min_db: -90.0,
            max_db: 0.0,
        }
    }

    /// Technical implementation of the set_range logic.
    pub fn set_range(&mut self, min_db: f32, max_db: f32) {
        self.min_db = min_db;
        self.max_db = max_db;
    }

    /// Technical implementation of the set_peak_hold_time logic.
    pub fn set_peak_hold_time(&mut self, seconds: f32) {
        self.peak_hold_time = seconds;
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, magnitudes: &[f32], sample_rate: f32, fft_size: usize) {
        let bin_count = self.bins.len().min(magnitudes.len());

        for i in 0..bin_count {
            let freq = (i as f32 * sample_rate) / fft_size as f32;
            let mag_db = magnitudes[i];

            self.bins[i].frequency = freq;
            self.bins[i].magnitude = mag_db.clamp(self.min_db, self.max_db);

            let smoothed = self.bins[i].smoothed;
            let attack = if mag_db > smoothed { 0.8 } else { 0.95 };
            self.bins[i].smoothed = smoothed * attack + mag_db * (1.0 - attack);

            if mag_db > self.peak_hold[i] {
                self.peak_hold[i] = mag_db;
                self.peak_counter = (self.peak_hold_time * 44100.0) as u32;
            } else if self.peak_counter > 0 {
                self.peak_counter -= 1;
            } else {
                self.peak_hold[i] *= self.peak_decay;
            }
        }
    }

    /// Technical implementation of the get_bin logic.
    pub fn get_bin(&self, index: usize) -> Option<&DisplayBin> {
        self.bins.get(index)
    }

    /// Technical implementation of the get_peak logic.
    pub fn get_peak(&self, index: usize) -> f32 {
        self.peak_hold.get(index).copied().unwrap_or(-120.0)
    }

    /// Technical implementation of the bin_count logic.
    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }
}

#[repr(align(64))]
/// Technical implementation of the LogFrequencyDisplay structure.
pub struct LogFrequencyDisplay {
    display: SpectrumDisplay,
    frequencies: alloc::vec::Vec<f32>,
    band_count: usize,
    min_freq: f32,
    max_freq: f32,
}

impl LogFrequencyDisplay {
    /// Initializes a new instance of the associated type.
    pub fn new(band_count: usize, min_freq: f32, max_freq: f32) -> Self {
        let frequencies: alloc::vec::Vec<f32> = (0..band_count)
            .map(|i| {
                let t = i as f32 / (band_count - 1) as f32;
                min_freq * (max_freq / min_freq).powf(t)
            })
            .collect();

        Self {
            display: SpectrumDisplay::new(band_count),
            frequencies,
            band_count,
            min_freq,
            max_freq,
        }
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, magnitudes: &[f32], sample_rate: f32, fft_size: usize) {
        let mapped = self.map_to_log_bands(magnitudes, sample_rate, fft_size);
        self.display.update(&mapped, sample_rate, fft_size);
    }

    /// Technical implementation of the map_to_log_bands logic.
    fn map_to_log_bands(
        &self,
        magnitudes: &[f32],
        sample_rate: f32,
        fft_size: usize,
    ) -> alloc::vec::Vec<f32> {
        let mut result = alloc::vec::Vec::with_capacity(self.band_count);

        for &target_freq in &self.frequencies {
            let bin_idx = (target_freq * fft_size as f32 / sample_rate) as usize;
            let mag = magnitudes.get(bin_idx).copied().unwrap_or(-120.0);
            result.push(mag);
        }

        result
    }

    /// Technical implementation of the get_band logic.
    pub fn get_band(&self, index: usize) -> Option<&DisplayBin> {
        self.display.get_bin(index)
    }

    /// Technical implementation of the get_band_peak logic.
    pub fn get_band_peak(&self, index: usize) -> f32 {
        self.display.get_peak(index)
    }
}

#[repr(align(64))]
/// Technical implementation of the SpectrogramDisplay structure.
pub struct SpectrogramDisplay {
    history: alloc::vec::Vec<alloc::vec::Vec<f32>>,
    history_index: usize,
    time_resolution: usize,
    bin_count: usize,
    max_db: f32,
    min_db: f32,
}

impl SpectrogramDisplay {
    /// Initializes a new instance of the associated type.
    pub fn new(history_lines: usize, bin_count: usize) -> Self {
        let mut history = alloc::vec::Vec::with_capacity(history_lines);
        for _ in 0..history_lines {
            history.push(alloc::vec::Vec::with_capacity(bin_count));
        }

        Self {
            history,
            history_index: 0,
            time_resolution: history_lines / 10,
            bin_count,
            max_db: 0.0,
            min_db: -90.0,
        }
    }

    /// Technical implementation of the set_range logic.
    pub fn set_range(&mut self, min_db: f32, max_db: f32) {
        self.min_db = min_db;
        self.max_db = max_db;
    }

    /// Performs vector addition logic.
    pub fn add_frame(&mut self, magnitudes: &[f32]) {
        let frame = self.history[self.history_index].clone();

        for (i, &mag) in magnitudes.iter().enumerate().take(self.bin_count) {
            if i >= frame.len() {
                continue;
            }
            self.history[self.history_index][i] = mag.clamp(self.min_db, self.max_db);
        }

        self.history_index = (self.history_index + 1) % self.history.len();
    }

    /// Technical implementation of the get_frame logic.
    pub fn get_frame(&self, time_index: usize) -> Option<&[f32]> {
        self.history.get(time_index).map(|v| v.as_slice())
    }

    /// Technical implementation of the get_column logic.
    pub fn get_column(&self, time_index: usize, bin_index: usize) -> Option<f32> {
        self.history
            .get(time_index)
            .and_then(|v| v.get(bin_index).copied())
    }
}
