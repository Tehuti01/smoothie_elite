/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa8f7933a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/plugin.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::descriptor::ClapDescriptor;
use super::params::ClapParamInfo;
use super::process::ClapProcessContext;

/// the Smoothie Elite framework.
/// # Example
/// ```rust,ignore
///
///     fn descriptor() -> ClapDescriptor {
///             "com.smoothieaudio.my-reverb",
///             "Smoothie Audio",
///             "A lush algorithmic reverb",
///         )
///     fn process(&mut self, ctx: &mut ClapProcessContext) { /* ... */ }
/// }
/// ```
pub trait SmoothieClapPlugin: Sized + Send {
    /// Static metadata describing the plugin to the host.
    fn descriptor() -> ClapDescriptor;

    /// Create a new default instance of the plugin.
    fn new() -> Self;

    /// Called by the host before the first `process()` call.
    fn init(&mut self, sample_rate: f64, _min_blocksize: u32, _max_blocksize: u32) {}

    /// Called when the host wants the plugin to process audio and events.
    fn process(&mut self, ctx: &mut ClapProcessContext);

    /// Reset internal state to initial conditions (triggered on transport rewind, etc.).
    fn reset(&mut self) {}

    /// Return a list of parameter descriptors exposed to the host.
    fn param_count() -> u32 {
        0
    }

    /// Populate `info` with descriptor for parameter at `index`.
    fn param_info(_index: u32, _info: &mut ClapParamInfo) {}

    /// Return the current normalised value [0.0, 1.0] of a parameter.
    fn param_value(_id: u32) -> f64 {
        0.0
    }

    /// Host is requesting the plugin to flush parameters that changed during the render.
    fn param_flush(&mut self) {}

    /// Return the latency in samples introduced by this plugin.
    fn latency_samples() -> u32 {
        0
    }

    /// Return the tail length in samples (reverb, delay decay) or `u32::MAX` for infinite.
    fn tail_samples() -> u32 {
        0
    }
}
