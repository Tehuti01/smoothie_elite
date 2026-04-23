/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb45b2388 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/effects/src/shelving.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::primitives::Sample;

#[derive(Clone, Copy, Default)]
/// Technical implementation of the ShelfCurve enumeration.
pub enum ShelfCurve {
    #[default]
    Standard,
    Sharp,
    Soft,
    Baxandall,
}

/// Technical implementation of the LowShelf structure.
pub struct LowShelf {
    freq: f32,
    gain: f32,
    curve: ShelfCurve,
    sample_rate: f32,
    coeff: [f32; 5],
    state: [f32; 2],
}

/// Technical implementation of the HighShelf structure.
pub struct HighShelf {
    freq: f32,
    gain: f32,
    curve: ShelfCurve,
    sample_rate: f32,
    coeff: [f32; 5],
    state: [f32; 2],
}

/// Technical implementation of the ShelvingEq structure.
pub struct ShelvingEq {
    low: LowShelf,
    high: HighShelf,
}

impl LowShelf {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            freq: 100.0,
            gain: 0.0,
            curve: ShelfCurve::Standard,
            sample_rate,
            coeff: [0.0; 5],
            state: [0.0; 2],
        }
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.max(20.0).min(self.sample_rate * 0.45);
        self.update_coeffs();
    }

    /// Technical implementation of the set_gain logic.
    pub fn set_gain(&mut self, gain_db: f32) {
        self.gain = gain_db.clamp(-24.0, 24.0);
        self.update_coeffs();
    }

    /// Technical implementation of the set_curve logic.
    pub fn set_curve(&mut self, curve: ShelfCurve) {
        self.curve = curve;
        self.update_coeffs();
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / self.sample_rate;
        let curve_mod = match self.curve {
            ShelfCurve::Standard => 1.0,
            ShelfCurve::Sharp => 1.5,
            ShelfCurve::Soft => 0.5,
            ShelfCurve::Baxandall => 0.7,
        };
        let a = 10.0_f32.powf(self.gain / 40.0);
        let alpha = (w.sin() / (1.0 + 1.0 / (1.0 + curve_mod * a.abs()))) * curve_mod;

        self.coeff[0] = (a - 1.0) * (w.cos() + alpha) / (a + 1.0);
        self.coeff[1] = (a - 1.0) * (w.cos() - alpha) / (a + 1.0);
        self.coeff[2] = 1.0;
        self.coeff[3] = -w.cos() + alpha;
        self.coeff[4] = -w.cos() - alpha;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let out = self.coeff[0] * input + self.coeff[1] * self.state[0] + self.state[1];
        self.state[1] = self.state[0];
        self.state[0] = input;
        out
    }
}

impl HighShelf {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            freq: 8000.0,
            gain: 0.0,
            curve: ShelfCurve::Standard,
            sample_rate,
            coeff: [0.0; 5],
            state: [0.0; 2],
        }
    }

    /// Technical implementation of the set_freq logic.
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.max(1000.0).min(self.sample_rate * 0.45);
        self.update_coeffs();
    }

    /// Technical implementation of the set_gain logic.
    pub fn set_gain(&mut self, gain_db: f32) {
        self.gain = gain_db.clamp(-24.0, 24.0);
        self.update_coeffs();
    }

    /// Technical implementation of the set_curve logic.
    pub fn set_curve(&mut self, curve: ShelfCurve) {
        self.curve = curve;
        self.update_coeffs();
    }

    /// Technical implementation of the update_coeffs logic.
    fn update_coeffs(&mut self) {
        let w = 2.0 * core::f32::consts::PI * self.freq / self.sample_rate;
        let curve_mod = match self.curve {
            ShelfCurve::Standard => 1.0,
            ShelfCurve::Sharp => 1.5,
            ShelfCurve::Soft => 0.5,
            ShelfCurve::Baxandall => 0.7,
        };
        let a = 10.0_f32.powf(self.gain / 40.0);
        let alpha = (w.sin() / (1.0 + 1.0 / (1.0 + curve_mod * a.abs()))) * curve_mod;

        self.coeff[0] = (a + 1.0) * (w.cos() - alpha) / (a + 1.0);
        self.coeff[1] = (a + 1.0) * (w.cos() + alpha) / (a + 1.0);
        self.coeff[2] = 1.0;
        self.coeff[3] = -w.cos() - alpha;
        self.coeff[4] = -w.cos() + alpha;
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let out = self.coeff[0] * input + self.coeff[1] * self.state[0] + self.state[1];
        self.state[1] = self.state[0];
        self.state[0] = input;
        out
    }
}

impl ShelvingEq {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            low: LowShelf::new(sample_rate),
            high: HighShelf::new(sample_rate),
        }
    }

    /// Technical implementation of the set_low_gain logic.
    pub fn set_low_gain(&mut self, gain_db: f32) {
        self.low.set_gain(gain_db);
    }
    /// Technical implementation of the set_low_freq logic.
    pub fn set_low_freq(&mut self, freq: f32) {
        self.low.set_freq(freq);
    }
    /// Technical implementation of the set_low_curve logic.
    pub fn set_low_curve(&mut self, curve: ShelfCurve) {
        self.low.set_curve(curve);
    }
    /// Technical implementation of the set_high_gain logic.
    pub fn set_high_gain(&mut self, gain_db: f32) {
        self.high.set_gain(gain_db);
    }
    /// Technical implementation of the set_high_freq logic.
    pub fn set_high_freq(&mut self, freq: f32) {
        self.high.set_freq(freq);
    }
    /// Technical implementation of the set_high_curve logic.
    pub fn set_high_curve(&mut self, curve: ShelfCurve) {
        self.high.set_curve(curve);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: Sample) -> Sample {
        let s = self.low.process(input);
        self.high.process(s)
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, left: Sample, right: Sample) -> (Sample, Sample) {
        (self.process(left), self.process(right))
    }
}

impl Default for LowShelf {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for HighShelf {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
impl Default for ShelvingEq {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
