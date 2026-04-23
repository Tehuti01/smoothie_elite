/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8f3c2d4e | REVISION: 2026.04.20                           │
 * │ PATH: crates/03-cognition/smoothie-preset/src/ironstack_presets.rs       │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Factory Preset definitions for the IRONSTACK-100 instrument.│
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Industrial-grade state initialization.                  │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::{PresetBank, PresetEntry, PresetSnapshot};
use alloc::vec;

/// Initializes the factory preset library for the IRONSTACK-100 instrument.
pub fn init_ironstack_factory_bank() -> PresetBank {
    let mut bank = PresetBank::new();

    // 1. "Industrial Crunch" - High Tube Drive, Aggressive Bias
    let mut crunch = PresetSnapshot::new(3);
    crunch.set(0, 1.8); // Tube Drive
    crunch.set(1, -5.0); // Plate Bias
    crunch.set(2, 0.7); // Master Out

    let mut entry_crunch = PresetEntry::new("Industrial Crunch", crunch);
    entry_crunch.category = "Lead".into();
    entry_crunch.tags = vec!["aggressive".into(), "high-gain".into(), "crunch".into()];
    bank.add(entry_crunch);

    // 2. "Glassy Clean" - Low Drive, Moderate Bias
    let mut clean = PresetSnapshot::new(3);
    clean.set(0, 0.3); // Tube Drive
    clean.set(1, -1.5); // Plate Bias
    clean.set(2, 0.9); // Master Out

    let mut entry_clean = PresetEntry::new("Glassy Clean", clean);
    entry_clean.category = "Clean".into();
    entry_clean.tags = vec!["glassy".into(), "clear".into(), "stable".into()];
    bank.add(entry_clean);

    // 3. "Default" - Balanced PHI-aligned parameters
    let mut default = PresetSnapshot::new(3);
    default.set(0, 0.5); // Tube Drive
    default.set(1, -2.0); // Plate Bias
    default.set(2, 0.8); // Master Out

    let entry_default = PresetEntry::new("IronStack Default", default);
    bank.add(entry_default);

    bank
}
