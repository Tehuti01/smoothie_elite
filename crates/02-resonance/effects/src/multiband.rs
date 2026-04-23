/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6e2e8c2e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/multiband.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::primitives::Sample;

/// Technical implementation of the Crossover structure.
pub struct Crossover {
    freq: f32,
    order: usize,
    sample_rate: f32,
    coeff: [f32; 4],
    state: [f32; 4],
}

impl Crossover {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            freq: 1000.0,
            order: 4,
            sample_rate,
            coeff: [0.0; 4],
            state: [0.0; 4],
        }
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.max(20.0).min(self.sample_rate * 0.45);
        self.update_coeffs();
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / self.sample_rate;
        let alpha = w.sin() / 2.0;

        if self.order == 4 {
            let a0 = 1.0 + alpha;
            self.coeff[0] = alpha / a0;
            self.coeff[1] = alpha / a0;
            self.coeff[2] = (1.0 - alpha) / a0;
            self.coeff[3] = (1.0 - alpha) / a0;
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process_low(&mut self, input: Sample) -> Sample {
        let lp = self.coeff[0] * input + self.coeff[1] * self.state[0] + self.state[1];
        self.state[1] = self.state[0];
        self.state[0] = input;
        lp
    }

    /// Primary real-time signal processing execution block.
    pub fn process_high(&mut self, input: Sample) -> Sample {
        
        input - self.process_low(input)
    }
}

/// Technical implementation of the Multiband3 structure.
pub struct Multiband3 {
    low_xo: Crossover,
    high_xo: Crossover,
    low_mid_xo: Crossover,
    low_buf: Sample,
    mid_buf: Sample,
    high_buf: Sample,
}

impl Multiband3 {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            low_xo: Crossover::new(sample_rate),
            high_xo: Crossover::new(sample_rate),
            low_mid_xo: Crossover::new(sample_rate),
            low_buf: 0.0,
            mid_buf: 0.0,
            high_buf: 0.0,
        }
    }

    /// Technical implementation of the set_low_freq logic.
    pub fn set_low_freq(&mut self, freq: f32) {
        self.low_xo.set_freq(freq);
    }
    /// Technical implementation of the set_high_freq logic.
    pub fn set_high_freq(&mut self, freq: f32) {
        self.high_xo.set_freq(freq);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> (Sample, Sample, Sample) {
        let low = self.low_xo.process_low(input);
        let hp = input - self.low_xo.process_low(input);
        let mid = self.low_mid_xo.process_low(hp);
        let high = hp - mid;

        self.low_buf = low;
        self.mid_buf = mid;
        self.high_buf = high;

        (low, mid, high)
    }

    /// Technical implementation of the get_low logic.
    pub fn get_low(&self) -> Sample {
        self.low_buf
    }
    /// Technical implementation of the get_mid logic.
    pub fn get_mid(&self) -> Sample {
        self.mid_buf
    }
    /// Technical implementation of the get_high logic.
    pub fn get_high(&self) -> Sample {
        self.high_buf
    }
}

/// Technical implementation of the Multiband4 structure.
pub struct Multiband4 {
    xo1: Crossover,
    xo2: Crossover,
    xo3: Crossover,
    band1: Sample,
    band2: Sample,
    band3: Sample,
    band4: Sample,
}

impl Multiband4 {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            xo1: Crossover::new(sample_rate),
            xo2: Crossover::new(sample_rate),
            xo3: Crossover::new(sample_rate),
            band1: 0.0,
            band2: 0.0,
            band3: 0.0,
            band4: 0.0,
        }
    }

    /// Technical implementation of the set_freq1 logic.
    pub fn set_freq1(&mut self, freq: f32) {
        self.xo1.set_freq(freq);
    }
    /// Technical implementation of the set_freq2 logic.
    pub fn set_freq2(&mut self, freq: f32) {
        self.xo2.set_freq(freq);
    }
    /// Technical implementation of the set_freq3 logic.
    pub fn set_freq3(&mut self, freq: f32) {
        self.xo3.set_freq(freq);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> (Sample, Sample, Sample, Sample) {
        let low1 = self.xo1.process_low(input);
        let high1 = input - low1;

        let low2 = self.xo2.process_low(high1);
        let high2 = high1 - low2;

        let low3 = self.xo3.process_low(high2);
        let high3 = high2 - low3;

        self.band1 = low1;
        self.band2 = low2;
        self.band3 = low3;
        self.band4 = high3;

        (low1, low2, low3, high3)
    }

    /// Technical implementation of the get_band1 logic.
    pub fn get_band1(&self) -> Sample {
        self.band1
    }
    /// Technical implementation of the get_band2 logic.
    pub fn get_band2(&self) -> Sample {
        self.band2
    }
    /// Technical implementation of the get_band3 logic.
    pub fn get_band3(&self) -> Sample {
        self.band3
    }
    /// Technical implementation of the get_band4 logic.
    pub fn get_band4(&self) -> Sample {
        self.band4
    }
}

/// Technical implementation of the MultibandCombiner structure.
pub struct MultibandCombiner {
    gains: [f32; 4],
}

impl MultibandCombiner {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self { gains: [1.0; 4] }
    }

    /// Technical implementation of the set_gain logic.
    pub fn set_gain(&mut self, band: usize, gain: f32) {
        if band < 4 {
            self.gains[band] = gain;
        }
    }

    /// Technical implementation of the combine4 logic.
    pub fn combine4(&self, b1: Sample, b2: Sample, b3: Sample, b4: Sample) -> Sample {
        b1 * self.gains[0] + b2 * self.gains[1] + b3 * self.gains[2] + b4 * self.gains[3]
    }

    /// Technical implementation of the combine3 logic.
    pub fn combine3(&self, low: Sample, mid: Sample, high: Sample) -> Sample {
        low * self.gains[0] + mid * self.gains[1] + high * self.gains[2]
    }
}

impl Default for Multiband3 {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for Multiband4 {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for MultibandCombiner {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
