/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x454e4749 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/dsp/engine.rs                                 │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core Synthesizer Engine (Oscillators, Filters).             │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_core::plugin::Reset;
use smoothie_dsp::filters::BiquadFilter;
use smoothie_dsp::oscillators::WavetableOscillator;

pub struct StargateEngine {
    oscillators: [WavetableOscillator; 2],
    filter: BiquadFilter,
}

impl StargateEngine {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillators: [
                WavetableOscillator::new(440.0, sample_rate),
                WavetableOscillator::new(440.0 * 1.5, sample_rate),
            ],
            filter: BiquadFilter::new(),
        }
    }

    #[inline(always)]
    pub fn process(&mut self, cutoff: f64, resonance: f64, sample_rate: f64) -> f32 {
        self.filter.set_lowpass(cutoff as f32, sample_rate as f32, resonance as f32);

        let osc_out = (self.oscillators[0].process() + self.oscillators[1].process()) * 0.5;
        self.filter.process(osc_out)
    }
}

impl Reset for StargateEngine {
    fn reset(&mut self) {
        for osc in &mut self.oscillators {
            osc.reset();
        }
        self.filter.reset();
    }
}
