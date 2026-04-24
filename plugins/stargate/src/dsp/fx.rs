/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x45464658 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/dsp/fx.rs                                     │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Effects Chain (Chorus, Delay, Reverb, Clipper).             │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_core::plugin::Reset;
use smoothie_dsp::utils::SoftClipper;
use smoothie_effects::{Chorus, DelayEffect, ReverbEffect};

pub struct StargateEffects {
    chorus: Chorus,
    delay: DelayEffect,
    reverb: ReverbEffect,
    clipper: SoftClipper,
}

impl StargateEffects {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            chorus: Chorus::new(sample_rate),
            delay: DelayEffect::default(),
            reverb: ReverbEffect::new(sample_rate),
            clipper: SoftClipper::new(0.1),
        }
    }

    #[inline(always)]
    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        let post_chorus = self.chorus.process(input, sample_rate);
        let post_delay = self.delay.process(post_chorus);
        let post_reverb = self.reverb.process(post_delay);
        self.clipper.process(post_reverb)
    }
}

impl Reset for StargateEffects {
    fn reset(&mut self) {
        // self.chorus.reset(); // Assuming chorus implements reset, if not, skip
        self.reverb.reset();
    }
}
