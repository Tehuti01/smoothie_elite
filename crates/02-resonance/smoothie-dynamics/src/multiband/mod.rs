/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1f3468b6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/multiband/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Splits audio into frequency bands for independent dynamics control.

use super::{Compressor, CompressorParams, CompressorStyle};

/// Technical implementation of the MultibandDynamics structure.
pub struct MultibandDynamics {
    crossovers: [CrossoverFilter; 3],
    compressors: [Compressor; 3],
    gains: [f32; 3],
    makeup: f32,
    band_count: usize,
}

/// Technical implementation of the CrossoverFilter structure.
pub struct CrossoverFilter {
    lowpass_b: [f32; 3],
    lowpass_a: [f32; 2],
    highpass_b: [f32; 3],
    highpass_a: [f32; 2],
    lp_state: [f32; 2],
    hp_state: [f32; 2],
}

impl CrossoverFilter {
    /// Initializes a new instance of the associated type.
    fn new(freq: f32, sample_rate: f32) -> Self {
        let w = 2.0 * core::f32::consts::PI * freq / sample_rate;
        let k = w.tan();
        let k2 = k * k;
        let dk = 1.0 / (1.0 + k / 2.0_f32.sqrt() + k2);

        let b0 = k2 * dk;
        let b1 = 2.0 * k2 * dk;
        let b2 = b0;
        let a1 = 2.0 * (k2 - 1.0) * dk;
        let a2 = (1.0 - k / 2.0_f32.sqrt() + k2) * dk;

        Self {
            lowpass_b: [b0, b1, b2],
            lowpass_a: [a1, a2],
            highpass_b: [1.0 - 2.0 * dk, -2.0 * dk, 1.0],
            highpass_a: [a1, a2],
            lp_state: [0.0; 2],
            hp_state: [0.0; 2],
        }
    }

    /// Primary real-time signal processing execution block.
    fn process_lowpass(&mut self, input: f32) -> f32 {
        let b = &self.lowpass_b;
        let a = &self.lowpass_a;

        let out = b[0] * input + b[1] * self.lp_state[0] + b[2] * self.lp_state[1]
            - a[0] * self.lp_state[0]
            - a[1] * self.lp_state[1];

        self.lp_state[1] = self.lp_state[0];
        self.lp_state[0] = input;
        self.lp_state[0] = out;
        out
    }

    /// Primary real-time signal processing execution block.
    fn process_highpass(&mut self, input: f32) -> f32 {
        let b = &self.highpass_b;
        let a = &self.highpass_a;

        let out = b[0] * input + b[1] * self.hp_state[0] + b[2] * self.hp_state[1]
            - a[0] * self.hp_state[0]
            - a[1] * self.hp_state[1];

        self.hp_state[1] = self.hp_state[0];
        self.hp_state[0] = input;
        out
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.lp_state = [0.0; 2];
        self.hp_state = [0.0; 2];
    }
}

impl MultibandDynamics {
    /// Initializes a new instance of the associated type.
    pub fn new(crossover_freqs: [f32; 2], sample_rate: f32) -> Self {
        let mut compressors = {
            let params = CompressorParams::default();
            [
                Compressor::new(CompressorStyle::Vca, params, sample_rate),
                Compressor::new(CompressorStyle::Vca, params, sample_rate),
                Compressor::new(CompressorStyle::Vca, params, sample_rate),
            ]
        };

        Self {
            crossovers: [
                CrossoverFilter::new(crossover_freqs[0], sample_rate),
                CrossoverFilter::new(crossover_freqs[1], sample_rate),
                CrossoverFilter::new(20000.0, sample_rate),
            ],
            compressors,
            gains: [1.0; 3],
            makeup: 1.0,
            band_count: 3,
        }
    }

    /// Technical implementation of the set_crossover logic.
    pub fn set_crossover(&mut self, band: usize, freq: f32, sample_rate: f32) {
        if band < 2 {
            self.crossovers[band] = CrossoverFilter::new(freq, sample_rate);
        }
    }

    /// Technical implementation of the compressor_mut logic.
    pub fn compressor_mut(&mut self, band: usize) -> Option<&mut Compressor> {
        self.compressors.get_mut(band)
    }

    /// Technical implementation of the set_makeup logic.
    pub fn set_makeup(&mut self, db: f32) {
        self.makeup = 10.0_f32.powf(db / 20.0);
    }

    /// Technical implementation of the set_band_gain logic.
    pub fn set_band_gain(&mut self, band: usize, db: f32) {
        if band < 3 {
            self.gains[band] = 10.0_f32.powf(db / 20.0);
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let lo = self.crossovers[0].process_lowpass(in_l);
        let lo_l = self.crossovers[0].process_highpass(lo);

        let mid = self.crossovers[1].process_lowpass(in_l);
        let mid_l = self.crossovers[1].process_highpass(mid);

        let hi = self.crossovers[2].process_highpass(in_l);

        let (lo_out, _) = self.compressors[0].process_stereo(lo_l, lo_l);
        let (mid_out, _) = self.compressors[1].process_stereo(mid_l, mid_l);
        let (hi_out, _) = self.compressors[2].process_stereo(hi, hi);

        let out_l = (lo_out * self.gains[0] + mid_out * self.gains[1] + hi_out * self.gains[2])
            * self.makeup;

        let lo_r = self.crossovers[0].process_lowpass(in_r);
        let lo_rr = self.crossovers[0].process_highpass(lo_r);
        let mid_r = self.crossovers[1].process_lowpass(in_r);
        let mid_rr = self.crossovers[1].process_highpass(mid_r);
        let hi_r = self.crossovers[2].process_highpass(in_r);

        let (lo_out_r, _) = self.compressors[0].process_stereo(lo_rr, lo_rr);
        let (mid_out_r, _) = self.compressors[1].process_stereo(mid_rr, mid_rr);
        let (hi_out_r, _) = self.compressors[2].process_stereo(hi_r, hi_r);

        let out_r =
            (lo_out_r * self.gains[0] + mid_out_r * self.gains[1] + hi_out_r * self.gains[2])
                * self.makeup;

        (out_l, out_r)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for crossover in self.crossovers.iter_mut() {
            crossover.reset();
        }
        for comp in self.compressors.iter_mut() {
            comp.reset();
        }
    }
}
