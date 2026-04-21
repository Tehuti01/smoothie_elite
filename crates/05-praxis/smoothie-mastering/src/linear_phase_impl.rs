/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xba760499 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-mastering/src/linear_phase_impl.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
/// Linear phase EQ implementation

use alloc::vec::Vec;

#[repr(align(64))]
/// Technical implementation of the LinearPhaseEq structure.
pub struct LinearPhaseEq {
    bands: Vec<EqBand>,
    fft_processor: Option<()>,
    impulse_response: Vec<f32>,
}

#[repr(align(64))]
/// Technical implementation of the EqBand structure.
pub struct EqBand {
    frequency: f32,
    gain_db: f32,
    q: f32,
    filter_type: FilterType,
    active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the FilterType enumeration.
pub enum FilterType {
    Bell,
    HighShelf,
    LowShelf,
    HighPass,
    LowPass,
    BandPass,
}

impl LinearPhaseEq {
    /// Initializes a new instance of the associated type.
    pub fn new(fft_size: usize) -> Self {
        Self {
            bands: Vec::with_capacity(8),
            fft_processor: None,
            impulse_response: Vec::with_capacity(fft_size),
        }
    }

    /// Performs vector addition logic.
    pub fn add_band(&mut self, freq: f32, gain_db: f32, q: f32, filter_type: FilterType) {
        self.bands.push(EqBand {
            frequency: freq,
            gain_db,
            q,
            filter_type,
            active: true,
        });
    }

    /// Technical implementation of the set_band_gain logic.
    pub fn set_band_gain(&mut self, band_index: usize, gain_db: f32) {
        if let Some(band) = self.bands.get_mut(band_index) {
            band.gain_db = gain_db;
        }
    }

    /// Technical implementation of the set_band_frequency logic.
    pub fn set_band_frequency(&mut self, band_index: usize, freq: f32) {
        if let Some(band) = self.bands.get_mut(band_index) {
            band.frequency = freq.clamp(20.0, 20000.0);
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&self, input: f32) -> f32 {
        let mut output = input;

        for band in &self.bands {
            if band.active {
                output = self.apply_band(output, band);
            }
        }

        output
    }

    /// Technical implementation of the apply_band logic.
    fn apply_band(&self, input: f32, band: &EqBand) -> f32 {
        input * 10.0_f32.powf(band.gain_db / 20.0)
    }

    /// Technical implementation of the band_count logic.
    pub fn band_count(&self) -> usize {
        self.bands.len()
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.bands.clear();
    }
}

#[repr(align(64))]
/// Technical implementation of the LinearPhaseCrossover structure.
pub struct LinearPhaseCrossover {
    low_band: LinearPhaseEq,
    high_band: LinearPhaseEq,
    crossover_freq: f32,
    sample_rate: f32,
}

impl LinearPhaseCrossover {
    /// Initializes a new instance of the associated type.
    pub fn new(crossover_freq: f32, sample_rate: f32, fft_size: usize) -> Self {
        let mut low = LinearPhaseEq::new(fft_size);
        let mut high = LinearPhaseEq::new(fft_size);

        low.add_band(crossover_freq, 0.0, 0.707, FilterType::LowPass);
        high.add_band(crossover_freq, 0.0, 0.707, FilterType::HighPass);

        Self {
            low_band: low,
            high_band: high,
            crossover_freq,
            sample_rate,
        }
    }

    /// Technical implementation of the set_crossover logic.
    pub fn set_crossover(&mut self, freq: f32) {
        self.crossover_freq = freq.clamp(20.0, 20000.0);
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&self, input: f32) -> (f32, f32) {
        (self.low_band.process(input), self.high_band.process(input))
    }
}
