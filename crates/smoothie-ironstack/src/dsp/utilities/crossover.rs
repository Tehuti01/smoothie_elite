/// A utility for splitting a signal into two frequency bands (Low and High).
///
/// This is typically used for multiband processing, such as applying 
/// different distortion or compression settings to low and high frequencies.
/// It uses a second-order filter to derive the low band and subtracts it from 
/// the input to derive the high band.
pub struct Crossover {
    sample_rate: f64,
    /// The frequency at which the signal is split (Hz).
    low_freq: f32,
    high_freq: f32,
    low_x1: f32,
    low_x2: f32,
    low_y1: f32,
    low_y2: f32,
    high_x1: f32,
    high_x2: f32,
    high_y1: f32,
    high_y2: f32,
    c0: f32,
    c1: f32,
    c2: f32,
    c3: f32,
    c4: f32,
}

impl Crossover {
    /// Creates a new Crossover instance with a default split frequency of 500Hz.
    pub fn new(sample_rate: f64) -> Self {
        let mut c = Self {
            sample_rate,
            low_freq: 500.0,
            high_freq: 2000.0,
            low_x1: 0.0,
            low_x2: 0.0,
            low_y1: 0.0,
            low_y2: 0.0,
            high_x1: 0.0,
            high_x2: 0.0,
            high_y1: 0.0,
            high_y2: 0.0,
            c0: 0.0,
            c1: 0.0,
            c2: 0.0,
            c3: 0.0,
            c4: 0.0,
        };
        c.update_coeffs();
        c
    }

    /// Internal method to recalculate filter coefficients based on the split frequency.
    fn update_coeffs(&mut self) {
        let fs = self.sample_rate as f32;
        let fc = self.low_freq.clamp(100.0, fs * 0.4);

        let omega_c = 2.0 * std::f32::consts::PI * fc / fs;
        let k = omega_c.tan();
        let k_sq = k * k;

        let norm = 1.0 / (1.0 + k.sqrt() + k_sq);

        self.c0 = k_sq * norm;
        self.c1 = 2.0 * self.c0;
        self.c2 = self.c0;
        self.c3 = 2.0 * (k_sq - 1.0) * norm;
        self.c4 = (1.0 - k.sqrt() + k_sq) * norm;
    }

    /// Sets the split frequency (cutoff).
    pub fn set_frequency(&mut self, freq: f32) {
        let fs = self.sample_rate as f32;
        self.low_freq = freq.clamp(100.0, fs * 0.4);
        self.update_coeffs();
    }

    /// Processes a single sample and returns a tuple of (low_band, high_band).
    pub fn process(&mut self, input: f32) -> (f32, f32) {
        let low = self.c0 * input + self.c1 * self.low_x1 + self.c2 * self.low_x2
            - self.c3 * self.low_y1
            - self.c4 * self.low_y2;

        self.low_x2 = self.low_x1;
        self.low_x1 = input;
        self.low_y2 = self.low_y1;
        self.low_y1 = low;

        let high = input - low;

        (low, high)
    }

    /// Processes stereo samples and returns two tuples: ((l_low, r_low), (l_high, r_high)).
    pub fn process_stereo(&mut self, left: f32, right: f32) -> ((f32, f32), (f32, f32)) {
        let (l_low, l_high) = self.process(left);
        let (r_low, r_high) = self.process(right);
        ((l_low, r_low), (l_high, r_high))
    }

    /// Resets all internal delay lines to silence.
    pub fn clear(&mut self) {
        self.low_x1 = 0.0;
        self.low_x2 = 0.0;
        self.low_y1 = 0.0;
        self.low_y2 = 0.0;
        self.high_x1 = 0.0;
        self.high_x2 = 0.0;
        self.high_y1 = 0.0;
        self.high_y2 = 0.0;
    }
}

impl Default for Crossover {
    fn default() -> Self {
        Self::new(44100.0)
    }
}
