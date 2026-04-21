/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1283d90b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-spectrum/src/display/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::math::FloatExt;

#[derive(Clone, Copy, Default, Debug)]
/// Technical implementation of the DisplayBin structure.
pub struct DisplayBin {
    pub rms: f32,
    pub peak: f32,
    pub center_freq_hz: f32,
}

/// Technical implementation of the SpectrumDisplay structure.
pub struct SpectrumDisplay {
    bins: Vec<DisplayBin>,
    fft_map: Vec<(usize, usize)>,
    peak_decay: f32,
}

impl SpectrumDisplay {
    /// Initializes a new instance of the associated type.
    pub fn new(
        display_bins: usize,
        fft_bins: usize,
        sample_rate: f32,
        min_hz: f32,
        max_hz: f32,
        peak_decay: f32,
    ) -> Self {
        // log(x) approximation: ln(x) via integer trick
        let fast_ln = |x: f32| -> f32 {
            if x <= 0.0 {
                return -100.0;
            }
            let n = x.to_bits();
            let exp = ((n >> 23) & 0xFF) as i32 - 127;
            let m = f32::from_bits((n & 0x7FFFFF) | 0x3F800000) - 1.0;
            exp as f32 * core::f32::consts::LN_2 + m * (1.0 - m * 0.5)
        };

        let fast_exp = |x: f32| -> f32 { smoothie_core::math::exp_approx(x) };

        let log_min = fast_ln(min_hz.max(1.0));
        let log_max = fast_ln(max_hz.min(sample_rate * 0.5));
        let bin_hz = sample_rate / (fft_bins * 2) as f32;

        let mut bins = Vec::with_capacity(display_bins);
        let mut fft_map = Vec::with_capacity(display_bins);

        for d in 0..display_bins {
            let t0 = d as f32 / display_bins as f32;
            let t1 = (d + 1) as f32 / display_bins as f32;
            let tc = (t0 + t1) * 0.5;

            let freq0 = fast_exp(log_min + t0 * (log_max - log_min));
            let freq1 = fast_exp(log_min + t1 * (log_max - log_min));
            let center = fast_exp(log_min + tc * (log_max - log_min));

            let fft_start = ((freq0 / bin_hz) as usize).min(fft_bins - 1);
            let fft_end = ((freq1 / bin_hz) as usize)
                .min(fft_bins - 1)
                .max(fft_start + 1);

            fft_map.push((fft_start, fft_end));
            bins.push(DisplayBin {
                center_freq_hz: center,
                ..Default::default()
            });
        }

        Self {
            bins,
            fft_map,
            peak_decay,
        }
    }

    /// Technical implementation of the update logic.
    pub fn update(&mut self, magnitudes: &[f32]) {
        for (d, bin) in self.bins.iter_mut().enumerate() {
            let (start, end) = self.fft_map[d];
            let mut sum = 0.0_f32;
            let mut count = 0usize;
            for &mag in magnitudes.get(start..end).unwrap_or(&[]) {
                sum += mag * mag;
                count += 1;
            }
            let rms = if count > 0 {
                fast_sqrt(sum / count as f32)
            } else {
                0.0
            };
            bin.rms = rms;
            bin.peak = if rms > bin.peak {
                rms
            } else {
                (bin.peak - self.peak_decay).max(0.0)
            };
        }
    }

    /// Technical implementation of the bins logic.
    pub fn bins(&self) -> &[DisplayBin] {
        &self.bins
    }
}

/// Technical implementation of the fast_sqrt logic.
fn fast_sqrt(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let xhalf = 0.5 * x;
    let mut i = x.to_bits();
    i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    x * (y * (1.5 - xhalf * y * y))
}
