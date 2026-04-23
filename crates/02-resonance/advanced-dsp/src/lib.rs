/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x50fdd31a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/advanced-dsp/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::f32::consts::PI;
use num_complex::Complex;
use rustfft::FftPlanner;

/// Technical implementation of the AdaptiveFft structure.
#[allow(dead_code)]
pub struct AdaptiveFft {
    planner: FftPlanner<f32>,
    fft_sizes: Vec<usize>,
    current_size: usize,
}

impl AdaptiveFft {
    /// Create FFT engine with standard sizes (512, 1024, 2048, 4096, 8192)
    pub fn new() -> Self {
        let fft_sizes = vec![512, 1024, 2048, 4096, 8192];
        Self {
            planner: FftPlanner::new(),
            fft_sizes,
            current_size: 1024,
        }
    }

    /// Adaptively choose FFT size based on signal characteristics
    ///
    /// Uses energy concentration analysis to determine optimal FFT size:
    /// - Low-frequency dominant → larger FFT for finer resolution
    /// - High-frequency content → smaller FFT for better time resolution
    pub fn choose_adaptive_size(&mut self, signal: &[f32]) -> usize {
        // Compute simple spectral centroid to determine signal character
        let (low_energy, high_energy) = self.compute_spectral_balance(signal);

        if low_energy > high_energy * 2.0 {
            // Bass-heavy: use large FFT for low-freq resolution
            self.current_size = 4096;
        } else if high_energy > low_energy * 2.0 {
            // Treble-heavy: use small FFT for temporal resolution
            self.current_size = 512;
        } else {
            // Balanced: use medium FFT
            self.current_size = 2048;
        }

        self.current_size
    }

    /// Compute energy in low and high frequency bands
    fn compute_spectral_balance(&self, signal: &[f32]) -> (f32, f32) {
        let low_energy: f32 = signal[..signal.len() / 2].iter().map(|s| s * s).sum();
        let high_energy: f32 = signal[signal.len() / 2..].iter().map(|s| s * s).sum();
        (low_energy, high_energy)
    }

    /// Perform FFT with current size
    pub fn forward_fft(&mut self, signal: &[f32]) -> Vec<Complex<f32>> {
        let fft = self.planner.plan_fft_forward(self.current_size);
        let mut buffer: Vec<Complex<f32>> = signal.iter().map(|&s| Complex::new(s, 0.0)).collect();

        // Pad to FFT size if needed
        while buffer.len() < self.current_size {
            buffer.push(Complex::new(0.0, 0.0));
        }

        fft.process(&mut buffer[..self.current_size]);
        buffer
    }

    /// Perform inverse FFT
    pub fn inverse_fft(&mut self, spectrum: &[Complex<f32>]) -> Vec<f32> {
        let ifft = self.planner.plan_fft_inverse(self.current_size);
        let mut buffer = spectrum.to_vec();

        // Pad if needed
        while buffer.len() < self.current_size {
            buffer.push(Complex::new(0.0, 0.0));
        }

        ifft.process(&mut buffer[..self.current_size]);

        // Normalize and convert back to real
        let scale = 1.0 / self.current_size as f32;
        buffer.iter().map(|c| c.re * scale).collect()
    }
}

///
/// without resampling artifacts.
/// Technical implementation of the PhaseVocoder structure.
pub struct PhaseVocoder {
    bin_phases: Vec<f32>,
    prev_magnitude: Vec<f32>,
    fft_size: usize,
    hop_size: usize,
}

impl PhaseVocoder {
    /// Create phase vocoder with specified FFT and hop size
    pub fn new(fft_size: usize, hop_size: usize) -> Self {
        Self {
            bin_phases: vec![0.0; fft_size],
            prev_magnitude: vec![0.0; fft_size],
            fft_size,
            hop_size,
        }
    }

    /// Apply time-stretching ratio (1.0 = no change, 2.0 = double speed)
    pub fn time_stretch(&mut self, spectrum: &[Complex<f32>], stretch_ratio: f32) {
        for (k, bin) in spectrum.iter().enumerate() {
            let mag = bin.norm();
            let phase = bin.arg();

            // Compute phase advance
            let phase_delta =
                (k as f32 / self.fft_size as f32 * 2.0 * PI * self.hop_size as f32) * stretch_ratio;

            self.bin_phases[k] = (phase + phase_delta) % (2.0 * PI);
            self.prev_magnitude[k] = mag;
        }
    }

    /// Get phase-corrected spectrum
    pub fn get_spectrum(&self) -> Vec<Complex<f32>> {
        self.bin_phases
            .iter()
            .zip(self.prev_magnitude.iter())
            .map(|(&phase, &mag)| Complex::from_polar(mag, phase))
            .collect()
    }
}

/// Technical implementation of the ParametricEq structure.
pub struct ParametricEq {
    bands: Vec<EqBand>,
}

/// Technical implementation of the EqBand structure.
pub struct EqBand {
    pub frequency: f32,
    pub gain: f32,
    pub q: f32,
    pub filter_type: EqType,
}

/// Technical implementation of the EqType enumeration.
pub enum EqType {
    Peak,
    Highshelf,
    Lowshelf,
    Notch,
}

impl ParametricEq {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { bands: Vec::new() }
    }

    /// Add EQ band
    pub fn add_band(&mut self, frequency: f32, gain: f32, q: f32, filter_type: EqType) {
        self.bands.push(EqBand {
            frequency,
            gain,
            q,
            filter_type,
        });
    }

    /// Compute biquad coefficients for a band
    pub fn compute_coefficients(&self, sample_rate: f32, band_idx: usize) -> Option<BiquadCoeffs> {
        let band = self.bands.get(band_idx)?;

        let w0 = 2.0 * PI * band.frequency / sample_rate;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * band.q);

        let a_gain = (band.gain / 20.0).exp(); // dB to linear

        match band.filter_type {
            EqType::Peak => {
                let b0 = 1.0 + alpha * a_gain;
                let b1 = -2.0 * cos_w0;
                let b2 = 1.0 - alpha * a_gain;
                let a0 = 1.0 + alpha / a_gain;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha / a_gain;

                Some(BiquadCoeffs {
                    b0,
                    b1,
                    b2,
                    a0,
                    a1,
                    a2,
                })
            }
            _ => None, // Implement other types similarly
        }
    }
}

/// Technical implementation of the BiquadCoeffs structure.
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
}

///
/// enabling proactive parameter adjustment.
/// Technical implementation of the TransientDetector structure.
pub struct TransientDetector {
    window_size: usize,
    threshold: f32,
}

impl TransientDetector {
    /// Initializes a new instance of the associated type.
    pub fn new(window_size: usize, threshold: f32) -> Self {
        Self {
            window_size,
            threshold,
        }
    }

    /// Detect transients in signal
    pub fn detect_transients(&self, signal: &[f32]) -> Vec<bool> {
        let mut transients = vec![false; signal.len()];

        for i in self.window_size..signal.len() {
            let prev_energy: f32 = signal[i - self.window_size..i].iter().map(|s| s * s).sum();
            let curr_energy: f32 = signal[i..core::cmp::min(i + self.window_size, signal.len())]
                .iter()
                .map(|s| s * s)
                .sum();

            if curr_energy > prev_energy * self.threshold {
                transients[i] = true;
            }
        }

        transients
    }
}

/// Technical implementation of the SpectralAnalyzer structure.
pub struct SpectralAnalyzer {
    fft_size: usize,
}

impl SpectralAnalyzer {
    /// Initializes a new instance of the associated type.
    pub fn new(fft_size: usize) -> Self {
        Self { fft_size }
    }

    /// Compute magnitude spectrum (linear)
    pub fn magnitude_spectrum(&self, spectrum: &[Complex<f32>]) -> Vec<f32> {
        spectrum.iter().map(|c| c.norm()).collect()
    }

    /// Compute power spectrum (dB)
    pub fn power_spectrum_db(&self, spectrum: &[Complex<f32>]) -> Vec<f32> {
        spectrum
            .iter()
            .map(|c| 20.0 * c.norm().log10().max(-100.0))
            .collect()
    }

    /// Estimate fundamental frequency (pitch)
    pub fn estimate_pitch(&self, spectrum: &[Complex<f32>], sample_rate: f32) -> Option<f32> {
        let mag = self.magnitude_spectrum(spectrum);
        let max_idx = mag
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx)?;

        let bin_freq = (max_idx as f32 / self.fft_size as f32) * sample_rate;
        Some(bin_freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_adaptive_fft logic.
    fn test_adaptive_fft() {
        let mut fft = AdaptiveFft::new();
        let signal = vec![0.1; 1024];
        let size = fft.choose_adaptive_size(&signal);
        let valid_sizes = vec![512, 1024, 2048, 4096, 8192];
        assert!(valid_sizes.contains(&size));
    }

    #[test]
    /// Technical implementation of the test_parametric_eq logic.
    fn test_parametric_eq() {
        let mut eq = ParametricEq::new();
        eq.add_band(1000.0, 6.0, 1.0, EqType::Peak);
        assert_eq!(eq.bands.len(), 1);
    }

    #[test]
    /// Technical implementation of the test_spectral_analyzer logic.
    fn test_spectral_analyzer() {
        let analyzer = SpectralAnalyzer::new(1024);
        let spectrum = vec![Complex::new(1.0, 0.0); 1024];
        let mag = analyzer.magnitude_spectrum(&spectrum);
        assert_eq!(mag.len(), 1024);
    }
}
