use smoothie_core::plugin::Reset;
/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x98A53A2C | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/filter/zdf.rs                          │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Ultra-Analog Zero-Delay Feedback State-Variable Filter.      │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Protected by Aztec Sacred Geometry Encryption.          │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;

pub struct ZdfSvf {
    s1: f64,
    s2: f64,
    g: f64,
    k: f64,
}

impl Default for ZdfSvf {
    fn default() -> Self {
        Self::new()
    }
}

impl ZdfSvf {
    pub fn new() -> Self {
        Self {
            s1: 0.0,
            s2: 0.0,
            g: 0.0,
            k: 0.0,
        }
    }

    pub fn set_params(&mut self, cutoff: f64, resonance: f64, sample_rate: f64) {
        let wd = 2.0 * 3.141592653589793 * cutoff;
        let t = 1.0 / sample_rate;
        let wa = (2.0 / t) * (wd * t / 2.0).tan();
        self.g = wa * t / 2.0;
        self.k = 2.0 - 2.0 * resonance;
    }
}

impl Reset for ZdfSvf {
        fn reset(&mut self) {
            self.s1 = 0.0;
            self.s2 = 0.0;
        }
}

impl PluginOsNode for ZdfSvf {

    #[inline(always)]
    fn process(&mut self, x: f64) -> f64 {
        let den = 1.0 + self.g * (self.g + self.k);
        let y_hp = (x - self.s1 * (self.g + self.k) - self.s2) / den;
        let v1 = self.g * y_hp;
        let y_bp = v1 + self.s1;
        self.s1 = v1 + y_bp;
        let v2 = self.g * y_bp;
        let y_lp = v2 + self.s2;
        self.s2 = v2 + y_lp;
        y_lp
    }
}
