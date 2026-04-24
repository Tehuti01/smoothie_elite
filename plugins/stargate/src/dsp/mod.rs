/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4453504d | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/dsp/mod.rs                                    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Master DSP Processing Module.                               │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

pub mod engine;
pub mod fx;
pub mod routing;

pub use engine::StargateEngine;
pub use fx::StargateEffects;
