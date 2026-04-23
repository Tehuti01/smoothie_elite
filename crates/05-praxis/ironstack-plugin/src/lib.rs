/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf3a2d1e | REVISION: 2026.04.20                           │
 * │ PATH: crates/05-praxis/ironstack-plugin/src/lib.rs                       │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core Plugin Orchestration for IRONSTACK-100.                │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Industrial-grade bridging of DSP, UI, and state tiers.  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::primitives::Sample;
use smoothie_params::bank::ParameterBank;
use smoothie_preset::{init_ironstack_factory_bank, PresetBank};
use smoothie_synth::IronStackPolySynth;
use smoothie_ui::IronStackHologram;
use smoothie_vst3::{Vst3AudioProcessor, Vst3EditController};

/// Primary orchestration structure for the IRONSTACK-100 plugin.
pub struct IronStackPlugin {
    pub synth: IronStackPolySynth,
    pub hologram: IronStackHologram,
    pub presets: PresetBank,
    sample_rate: f32,
}

impl IronStackPlugin {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            synth: IronStackPolySynth::new(sample_rate),
            hologram: IronStackHologram::new(),
            presets: init_ironstack_factory_bank(),
            sample_rate,
        }
    }
}

/// Technical implementation of the Vst3AudioProcessor trait for industrial DSP routing.
impl Vst3AudioProcessor for IronStackPlugin {
    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        // Re-initialize synth for new sample rate
        self.synth = IronStackPolySynth::new(sample_rate);
    }

    fn process_block(&mut self, buffer: &mut [Sample]) {
        self.synth.generate_into(buffer);
    }
}

/// Technical implementation of the Vst3EditController trait for UI/Parameter synchronization.
impl Vst3EditController for IronStackPlugin {
    fn sync_ui_to_params(&mut self, bank: &ParameterBank) {
        self.hologram.sync_to_bank(bank);
    }

    fn sync_params_to_ui(&mut self, bank: &ParameterBank) {
        self.hologram.sync_from_bank(bank);
    }
}

pub mod clap;
pub mod vst3;
