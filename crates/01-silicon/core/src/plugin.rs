/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7629b3c4 | REVISION: 2026.04.20                           │
 * │ PATH: crates/01-silicon/core/src/plugin.rs                               │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Core plugin traits and orchestration types.                 │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// Technical implementation of the PluginCategory enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginCategory {
    Effect,
    Instrument,
    Utility,
    Analyzer,
    Other,
}

/// Technical implementation of the ProcessStatus enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    Ok,
    Error,
    Overload,
    Tail,
}

/// Technical implementation of the PluginInfo structure.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub vendor: &'static str,
    pub version: &'static str,
    pub category: PluginCategory,
    pub input_channels: usize,
    pub output_channels: usize,
    pub description: &'static str,
    pub website: &'static str,
}

impl Default for PluginInfo {
    fn default() -> Self {
        Self {
            name: "Smoothie Plugin",
            vendor: "Smoothie Audio",
            version: "0.1.0",
            category: PluginCategory::Other,
            input_channels: 2,
            output_channels: 2,
            description: "Built with Smoothie Elite",
            website: "https://smoothieaudio.dev",
        }
    }
}

/// A handle for type-safe parameter access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamHandle(pub usize);

/// Trait for reporting signal latency.
pub trait Latency {
    /// Returns the latency of the component in samples.
    fn latency(&self) -> usize { 0 }
}

/// Trait for reporting instrument tail time.
pub trait TailTime {
    /// Returns the decay tail time in samples.
    fn tail_samples(&self) -> usize { 0 }
}

/// Unified Reset Trait for all DSP components.
pub trait Reset {
    /// Resets the component to its initial state.
    fn reset(&mut self);
}

/// Trait for high-performance block-based processing.
pub trait ProcessBlock {
    /// Processes a block of audio samples.
    fn process_block(&mut self, inputs: &[&[f32]], outputs: &mut [&mut [f32]]);
}

/// The Master Plugin Trait: Every Seraphic plugin must implement this.
pub trait SmoothiePlugin: Send + Sync + Reset + Latency + TailTime {
    /// Returns metadata about the plugin.
    fn info() -> PluginInfo where Self: Sized;
    
    /// Initializes a new instance of the plugin.
    fn new(sample_rate: f32) -> Self where Self: Sized;
    
    /// Primary audio processing loop.
    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus;
    
    /// Updates the sample rate.
    fn set_sample_rate(&mut self, sr: f32);

    // Optional parameter interface
    fn param_count(&self) -> usize { 0 }
    fn get_param(&self, _index: usize) -> f32 { 0.0 }
    fn set_param(&mut self, _index: usize, _value: f32) {}
    fn get_param_name(&self, _index: usize) -> &'static str { "" }
}

/// Helper trait for unified audio processing.
pub trait AudioProcessor: SmoothiePlugin {}
impl<T: SmoothiePlugin> AudioProcessor for T {}
