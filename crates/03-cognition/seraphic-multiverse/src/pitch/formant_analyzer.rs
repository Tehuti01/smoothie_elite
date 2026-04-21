/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5b0eca6f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-multiverse/src/pitch/formant_analyzer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::PHI;

/// Maps the spectral energy distribution and timbre of vocal signals.
#[repr(align(64))]
/// Technical implementation of the FormantAnalyzer structure.
pub struct FormantAnalyzer {
    centroids: [f32; 8], // Multiband energy tracking
    prev_input: f32,
    zcr: f32, // Zero Crossing Rate (sibilance detection)
}

impl FormantAnalyzer {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            centroids: [0.0; 8],
            prev_input: 0.0,
            zcr: 0.0,
        }
    }

    /// 🧠 Analyze a block of spectral data
    /// Calculates RMS energy across 8 uniform frequency bands.
    pub fn analyze_spectral_density(&mut self, spectrum: &[f32]) {
        let band_size = spectrum.len() / 8;
        if band_size == 0 {
            return;
        }

        for i in 0..8 {
            let start = i * band_size;
            let end = start + band_size;

            let mut energy = 0.0;
            for j in start..end {
                let s = spectrum[j];
                energy += s * s;
            }

            // RMS = sqrt(mean(squares))
            self.centroids[i] = (energy / band_size as f32).sqrt();
        }

        // Apply PHI-aligned resonance scaling for timbre profiling
        for i in 0..8 {
            self.centroids[i] *= (PHI).powi(i as i32 / 4);
        }
    }

    /// 🧠 Detect sibilance (de-esser logic)
    /// High ZCR indicates unvoiced speech (s, t, ch) or noise.
    pub fn calculate_zcr(&mut self, block: &[f32]) -> f32 {
        if block.is_empty() {
            return 0.0;
        }

        let mut crosses = 0;
        for &sample in block {
            // Check for zero crossing sign change
            if (sample > 0.0) != (self.prev_input > 0.0) {
                crosses += 1;
            }
            self.prev_input = sample;
        }

        self.zcr = (crosses as f32) / (block.len() as f32);
        self.zcr
    }

    /// 🦾 Return Timbre Profile
    pub fn get_timbre_vector(&self) -> [f32; 8] {
        self.centroids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_zcr_sine_vs_noise logic.
    fn test_zcr_sine_vs_noise() {
        let mut analyzer = FormantAnalyzer::new();

        // Sine wave (low ZCR)
        let mut sine = [0.0f32; 100];
        for i in 0..100 {
            sine[i] = (i as f32 * 0.1).sin();
        }
        let zcr_sine = analyzer.calculate_zcr(&sine);

        // Alternating pulses (high ZCR)
        let mut noise = [0.0f32; 100];
        for i in 0..100 {
            noise[i] = if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        let zcr_noise = analyzer.calculate_zcr(&noise);

        assert!(
            zcr_noise > zcr_sine,
            "Noise should have higher ZCR than Sine"
        );
    }

    #[test]
    /// Technical implementation of the test_spectral_density_bands logic.
    fn test_spectral_density_bands() {
        let mut analyzer = FormantAnalyzer::new();
        let mut spectrum = [0.0f32; 64]; // 8 bins per band

        // Put energy in the first band
        for i in 0..8 {
            spectrum[i] = 1.0;
        }

        analyzer.analyze_spectral_density(&spectrum);
        let timbre = analyzer.get_timbre_vector();

        assert!(timbre[0] > 0.0);
        assert_eq!(timbre[1], 0.0);
    }
}

/// 🛡️ System Integrity Verification: Timbre profiling verified.
pub const ANALYZER_DENSITY: &str = "SERAPHIC_300IQ_SPECTRAL_TIMBRE";
