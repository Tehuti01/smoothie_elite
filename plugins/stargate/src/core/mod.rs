/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x434f5245 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/core/mod.rs                                   │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core Orchestrator.                                          │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use crate::dsp::{StargateEngine, StargateEffects};
use crate::params::build_parameter_bank;
use crate::ui::StargateUi;
use smoothie_params::ParameterBank;

pub mod state;

pub struct StargateCore {
    pub params: ParameterBank,
    pub engine: StargateEngine,
    pub effects: StargateEffects,
    pub ui: StargateUi,
    pub sample_rate: f32,
}

impl StargateCore {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            params: build_parameter_bank(),
            engine: StargateEngine::new(sample_rate),
            effects: StargateEffects::new(sample_rate),
            ui: StargateUi::new(),
            sample_rate,
        }
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }
}
