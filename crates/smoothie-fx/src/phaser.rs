//! Phaser — cascaded first-order allpass filters with LFO modulation.

use smoothie_math::trig::Phasor;
use std::f32::consts::PI;

/// Single first-order allpass section.
struct AllpassSection {
    a1:    f32,
    state: f32,
}

impl AllpassSection {
    fn new() -> Self { Self { a1: 0.0, state: 0.0 } }

    fn set_freq(&mut self, hz: f32, sample_rate: f32) {
        let omega = 2.0 * PI * hz / sample_rate;
        let tan_half = (omega * 0.5).tan();
        self.a1 = (tan_half - 1.0) / (tan_half + 1.0);
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let y = self.a1 * x + self.state;
        self.state = x - self.a1 * y;
        y
    }
}

/// N-stage phaser (up to 12 stages).
pub struct Phaser {
    stages:  Vec<AllpassSection>,
    lfo:     Phasor,
    /// LFO rate in Hz.
    pub rate:     f32,
    /// LFO depth (fraction of frequency range).
    pub depth:    f32,
    /// Centre frequency of the notch sweep (Hz).
    pub centre:   f32,
    /// Feedback coefficient.
    pub feedback: f32,
    fb_state: f32,
    /// Wet/dry [0, 1].
    pub mix:  f32,
    sample_rate: f32,
}

impl Phaser {
    pub fn new(num_stages: usize, sample_rate: f32) -> Self {
        let stages = (0..num_stages.clamp(2, 12)).map(|_| AllpassSection::new()).collect();
        Self {
            stages,
            lfo: Phasor::new(0.5, sample_rate),
            rate: 0.5,
            depth: 0.7,
            centre: 1000.0,
            feedback: 0.6,
            fb_state: 0.0,
            mix: 0.5,
            sample_rate,
        }
    }

    pub fn set_rate(&mut self, hz: f32) {
        self.rate = hz;
        self.lfo.set_freq(hz, self.sample_rate);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // LFO in [0, 1]
        let lfo = self.lfo.tick();
        let lfo_sin = (lfo * std::f32::consts::TAU).sin() * 0.5 + 0.5;
        let lo = self.centre * (1.0 - self.depth);
        let hi = self.centre * (1.0 + self.depth);
        let freq = lo + lfo_sin * (hi - lo);

        for stage in self.stages.iter_mut() {
            stage.set_freq(freq, self.sample_rate);
        }

        let inp = x + self.fb_state * self.feedback;
        let mut y = inp;
        for stage in self.stages.iter_mut() {
            y = stage.process(y);
        }
        self.fb_state = y;

        x * (1.0 - self.mix) + y * self.mix
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
