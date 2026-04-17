//! The `SmoothiePlugin` trait — the single thing every plugin implements.

use std::sync::Arc;
use crate::{AudioLayout, ProcessContext, InitContext, ProcessStatus, PluginUid, FormatFlags};

/// The core trait for every Smoothie Elite plugin.
///
/// Implement this and call `smoothie_export!(MyPlugin)` — that's it.
/// The framework handles VST3, CLAP, AU, AAX, and standalone for you.
pub trait SmoothiePlugin: Default + Send + 'static {
    // ── Identity ──────────────────────────────────────────────────────────────

    /// Display name shown in the DAW plugin list.
    const NAME: &'static str;

    /// Company / developer name.
    const VENDOR: &'static str;

    /// Semver plugin version string.
    const VERSION: &'static str;

    /// Globally unique plugin identifier.
    const UID: PluginUid;

    /// URL to plugin product page.
    const URL: &'static str = "";

    /// Support email.
    const EMAIL: &'static str = "";

    /// Declare all supported audio I/O configurations.
    /// The first layout is the default.
    fn audio_layouts() -> &'static [AudioLayout];

    // ── Parameters ────────────────────────────────────────────────────────────

    /// Return all automatable parameters for this plugin.
    /// The host uses this to build the automation list.
    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![]
    }

    // ── Plugin formats this plugin supports ───────────────────────────────────

    /// By default, exports all formats enabled via Cargo features.
    /// Override to restrict.
    fn supported_formats() -> FormatFlags {
        FormatFlags::all()
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────────

    /// Called once when the plugin is first loaded by the host.
    /// Return `false` to signal failure.
    #[allow(unused_variables)]
    fn initialize(&mut self, ctx: &mut InitContext) -> bool { true }

    /// Called whenever sample rate or buffer size changes.
    #[allow(unused_variables)]
    fn reset(&mut self) {}

    /// The main audio processing callback. Called on the real-time audio thread.
    ///
    /// **Rules for the audio thread:**
    /// - No heap allocation
    /// - No mutex locks (use atomics or lock-free structures)
    /// - No I/O (disk, network, logging)
    /// - No panics
    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus;

    /// Called when the plugin is being deactivated / unloaded.
    fn deactivate(&mut self) {}

    // ── Optional: editor (GUI) ─────────────────────────────────────────────────

    /// Return `true` if this plugin has a GUI editor.
    fn has_editor() -> bool { false }

    /// Called by the standalone/host to create and attach the editor UI.
    /// The default implementation does nothing (headless plugin).
    #[allow(unused_variables)]
    fn create_editor(&mut self, parent: *mut std::ffi::c_void) -> bool { false }

    /// Destroy and detach the editor UI.
    fn destroy_editor(&mut self) {}

    // ── Optional: preset state ────────────────────────────────────────────────

    /// Serialize plugin state to bytes (for preset save).
    fn save_state(&self) -> Vec<u8> { vec![] }

    /// Deserialize plugin state from bytes (for preset load).
    #[allow(unused_variables)]
    fn load_state(&mut self, data: &[u8]) {}

    // ── Optional: tail ────────────────────────────────────────────────────────

    /// Number of extra samples needed after input stops (reverb tail, delay, etc.).
    /// Return 0 for no tail.
    fn tail_length_samples(&self) -> u32 { 0 }
}
