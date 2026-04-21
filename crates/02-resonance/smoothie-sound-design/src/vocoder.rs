/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4f33dce2 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/vocoder.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;

const NUM_BANDS: usize = 32;

/// Technical implementation of the Vocoder structure.
pub struct Vocoder {
    pub analysis_filters: [BiquadFilter; NUM_BANDS],
    pub synthesis_filters: [BiquadFilter; NUM_BANDS],
    pub band_gains: [f32; NUM_BANDS],
}

#[derive(Clone, Copy)]
/// Technical implementation of the BiquadFilter structure.
pub struct BiquadFilter {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl Vocoder {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            analysis_filters: [BiquadFilter::default(); NUM_BANDS],
            synthesis_filters: [BiquadFilter::default(); NUM_BANDS],
            band_gains: [0.0; NUM_BANDS],
        }
    }

    /// Technical implementation of the init_bands logic.
    pub fn init_bands(&mut self, sample_rate: f32) {
        let min_freq = 80.0;
        let max_freq = 8000.0;
        for i in 0..NUM_BANDS {
            let ratio = i as f32 / NUM_BANDS as f32;
            let freq = min_freq * (max_freq / min_freq).powf(ratio);
            self.analysis_filters[i] = BiquadFilter::bandpass(freq, freq * 0.2, sample_rate);
            self.synthesis_filters[i] = BiquadFilter::bandpass(freq, freq * 0.2, sample_rate);
        }
    }
}

impl BiquadFilter {
    /// Technical implementation of the default logic.
    pub const fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
    /// Technical implementation of the bandpass logic.
    pub fn bandpass(freq: f32, bw: f32, sr: f32) -> Self {
        let omega = 2.0 * 3.14159 * freq / sr;
        let alpha = (3.14159 * bw / sr).tan();
        let cos_w = omega.cos();
        let b0 = alpha / (1.0 + alpha);
        let b2 = -b0;
        let a1 = -2.0 * cos_w / (1.0 + alpha);
        let a2 = (1.0 - alpha) / (1.0 + alpha);
        Self {
            b0,
            b1: 0.0,
            b2,
            a1,
            a2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}
