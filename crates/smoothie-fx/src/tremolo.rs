//! Tremolo — amplitude modulation by an LFO with multiple waveform shapes.

use smoothie_math::trig::Phasor;
use std::f32::consts::TAU;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TremoloShape {
    Sine,
    Triangle,
    Square,
    SawUp,
    SawDown,
    Random,
}

pub struct Tremolo {
    lfo:   Phasor,
    rng:   smoothie_math::random::Xorshift32,
    rng_a: f32,
    rng_b: f32,
    rng_t: f32,
    /// LFO shape.
    pub shape:  TremoloShape,
    /// Rate in Hz.
    pub rate:   f32,
    /// Depth [0, 1] — 0 = no effect, 1 = mutes to silence.
    pub depth:  f32,
    /// Stereo phase offset [0, 1] for tremolo panning.
    pub stereo: f32,
    lfo2:  Phasor,
    sample_rate: f32,
}

impl Tremolo {
    pub fn new(sample_rate: f32) -> Self {
        let mut lfo2 = Phasor::new(4.0, sample_rate);
        lfo2.phase = 0.5;
        Self {
            lfo: Phasor::new(4.0, sample_rate),
            lfo2,
            rng: smoothie_math::random::Xorshift32::new(12345),
            rng_a: 0.0, rng_b: 0.0, rng_t: 0.0,
            shape: TremoloShape::Sine,
            rate: 4.0,
            depth: 0.5,
            stereo: 0.0,
            sample_rate,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.rate = hz;
        self.lfo.set_freq(hz, self.sample_rate);
        self.lfo2.set_freq(hz, self.sample_rate);
    }

    #[inline]
    fn lfo_value(phase: f32, shape: TremoloShape) -> f32 {
        match shape {
            TremoloShape::Sine     => (phase * TAU).sin() * 0.5 + 0.5,
            TremoloShape::Triangle => {
                let t = phase * 2.0;
                if t < 1.0 { t } else { 2.0 - t }
            }
            TremoloShape::Square   => if phase < 0.5 { 1.0 } else { 0.0 },
            TremoloShape::SawUp    => phase,
            TremoloShape::SawDown  => 1.0 - phase,
            TremoloShape::Random   => 0.0, // handled separately
        }
    }

    /// Process stereo. Returns (left, right) with amplitude modulation applied.
    pub fn process_stereo(&mut self, l: f32, r: f32) -> (f32, f32) {
        let phase_l = self.lfo.tick();
        let phase_r = self.lfo2.tick();

        let mod_l = if self.shape == TremoloShape::Random {
            self.rng_t += 1.0 / (self.sample_rate / self.rate);
            if self.rng_t >= 1.0 {
                self.rng_t = 0.0;
                self.rng_a = self.rng_b;
                self.rng_b = self.rng.next_f32_01();
            }
            self.rng_a + self.rng_t * (self.rng_b - self.rng_a)
        } else {
            Self::lfo_value(phase_l, self.shape)
        };

        let mod_r = if self.stereo > 0.0 {
            Self::lfo_value(phase_r, self.shape)
        } else {
            mod_l
        };

        let gain_l = 1.0 - self.depth * mod_l;
        let gain_r = 1.0 - self.depth * (mod_l + (mod_r - mod_l) * self.stereo);

        (l * gain_l, r * gain_r)
    }
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
