//! Ring modulator and frequency shifter.

use smoothie_math::trig::Phasor;

pub struct RingMod {
    carrier: Phasor,
    /// Carrier frequency Hz.
    pub freq: f32,
    /// Wet/dry [0, 1].
    pub mix:  f32,
    sample_rate: f32,
}

impl RingMod {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            carrier: Phasor::new(440.0, sample_rate),
            freq: 440.0,
            mix: 1.0,
            sample_rate,
        }
    }

    pub fn set_freq(&mut self, hz: f32) {
        self.freq = hz;
        self.carrier.set_freq(hz, self.sample_rate);
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        let phase = self.carrier.tick();
        let carrier = (phase * std::f32::consts::TAU).sin();
        x * (1.0 - self.mix) + (x * carrier) * self.mix
    }
}
