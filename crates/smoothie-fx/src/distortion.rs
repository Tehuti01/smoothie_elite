//! Distortion / overdrive / fuzz algorithms.

/// Asymmetric tube overdrive (even-harmonic emphasis like a real tube).
pub struct TubeOverdrive {
    /// Drive amount [0.1, 20.0].
    pub drive:  f32,
    /// Tone (one-pole highpass coefficient).
    tone_state: f32,
    tone_coeff: f32,
    /// Output level.
    pub level:  f32,
}

impl TubeOverdrive {
    pub fn new(sample_rate: f32) -> Self {
        let tone_coeff = (-2.0 * std::f32::consts::PI * 5000.0 / sample_rate).exp();
        Self { drive: 3.0, tone_state: 0.0, tone_coeff, level: 0.5 }
    }

    pub fn set_tone_hz(&mut self, hz: f32, sample_rate: f32) {
        self.tone_coeff = (-2.0 * std::f32::consts::PI * hz / sample_rate).exp();
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let driven = x * self.drive;
        let shaped = if driven >= 0.0 {
            driven.tanh()
        } else {
            // Asymmetric negative half — softer, generates even harmonics
            (driven * 0.8).tanh() * 1.05
        };
        // Tone filter
        self.tone_state += (1.0 - self.tone_coeff) * (shaped - self.tone_state);
        let toned = shaped - self.tone_state;
        toned * self.level
    }
}

/// Fuzz pedal — hard clip with bias.
pub struct Fuzz {
    pub drive: f32,
    pub fuzz:  f32,
    pub level: f32,
    dc_state:  f32,
}

impl Fuzz {
    pub fn new() -> Self { Self { drive: 5.0, fuzz: 0.8, level: 0.5, dc_state: 0.0 } }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let driven = x * self.drive;
        // Asymmetric hard clip with fuzz amount
        let clipped = if driven > self.fuzz {
            self.fuzz
        } else if driven < -(self.fuzz * 0.8) {
            -(self.fuzz * 0.8)
        } else {
            driven
        };
        // DC blocking
        self.dc_state = 0.999 * self.dc_state + clipped;
        let out = clipped - self.dc_state * 0.001;
        out * self.level
    }
}

impl Default for Fuzz {
    fn default() -> Self { Self::new() }
}

/// Diode clipper simulation (germanium-style soft clip).
pub struct DiodeClipper {
    pub drive: f32,
    pub bias:  f32,
}

impl DiodeClipper {
    pub fn new() -> Self { Self { drive: 2.0, bias: 0.05 } }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let inp = x * self.drive + self.bias;
        // Diode characteristic approximation: sinh-like asymmetric
        let pos = (inp * 2.0).exp();
        let neg = (-inp).exp();
        let y = (pos - neg) / (pos + neg + 1.0); // modified sigmoid
        y * 0.8
    }
}

impl Default for DiodeClipper {
    fn default() -> Self { Self::new() }
}

/// Wavefolder — generates rich upper harmonics like Buchla-style folders.
pub struct Wavefolder {
    pub stages: usize,
    pub drive:  f32,
}

impl Wavefolder {
    pub fn new() -> Self { Self { stages: 2, drive: 1.5 } }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let mut y = x * self.drive;
        for _ in 0..self.stages {
            y = (y * std::f32::consts::PI).sin();
        }
        y
    }
}

impl Default for Wavefolder {
    fn default() -> Self { Self::new() }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
