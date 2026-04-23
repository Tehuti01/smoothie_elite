/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf2a3d4e | REVISION: 2026.04.20                           │
 * │ PATH: crates/05-praxis/ironstack-plugin/src/vst3.rs                      │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: VST3 Entry Point Orchestration for IRONSTACK-100.           │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Industrial-grade plugin interface stabilization.         │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::IronStackPlugin;
use smoothie_vst3::{Vst3Category, Vst3ComponentFlags, Vst3PluginEntry, Vst3ProcessorInfo};

/// Initializes the VST3 entry point for the IRONSTACK-100 instrument.
pub fn init_vst3_entry() -> Vst3PluginEntry {
    let mut entry = Vst3PluginEntry::new("IRONSTACK-100", "Smoothie Audio");

    // Industrial-grade metadata
    entry = entry.with_version(1, 0, 0).with_sdk_version(3, 7, 9);

    entry
}

/// Technical implementation of the VST3 Processor configuration.
pub fn init_vst3_processor_info() -> Vst3ProcessorInfo {
    Vst3ProcessorInfo {
        flags: Vst3ComponentFlags::REQUIRES_GUI | Vst3ComponentFlags::DAW_SILENT_SWITCH,
        latency_samples: 0,
        initial_delay: 0,
        tail_samples: 0,
        silnable_tail: true,
    }
}

/// Helper for the host to identify the plugin category.
pub fn get_vst3_category() -> Vst3Category {
    Vst3Category::Instrument
}

/// 🧠 Simulation of the Vst3 instantiation logic for technical certification.
pub fn create_vst3_instance(sample_rate: f32) -> IronStackPlugin {
    IronStackPlugin::new(sample_rate)
}
