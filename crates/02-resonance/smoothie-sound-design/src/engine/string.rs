/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf9ea078a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/engine/string.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;
use smoothie_dsp::prelude::DelayLine;

#[repr(align(64))]
/// Technical implementation of the EnterpriseString structure.
pub struct EnterpriseString {
    delay: DelayLine,
    filter_state: f64,
    feedback: f64,
}

impl EnterpriseString {
    /// Initializes a new instance of the associated type.
    pub fn new(freq: f64, sample_rate: f64) -> Self {
        let length = (sample_rate / freq) as usize;
        Self {
            delay: DelayLine::new(length),
            filter_state: 0.0,
            feedback: 0.999, // [Engineering Phase 14]: Stable equilibrium
        }
    }
}

impl PluginOsNode for EnterpriseString {
    #[seraphic_specification(L0, A0, PHI)]
    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process(&mut self, input: f64) -> f64 {
        // [Engineering Phase 21]: Waveguide scattering
        let delayed = self.delay.read() as f64;
        
        // PHI-resonant low-pass smoothing
        self.filter_state = (delayed * 0.618) + (self.filter_state * 0.382);
        
        let feedback_sample = (input + self.filter_state) * self.feedback;
        self.delay.write(feedback_sample as f32);
        
        feedback_sample
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.delay.reset();
        self.filter_state = 0.0;
    }
}
