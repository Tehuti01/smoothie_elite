//! # smoothie-core
//!
//! The foundation of **Smoothie Elite** — the elite Rust audio plugin framework.
//!
//! Every Smoothie Elite plugin starts here:
//!
//! ```rust,no_run
//! use smoothie_core::prelude::*;
//!
//! #[derive(Default)]
//! struct MyPlugin { gain: f32 }
//!
//! impl SmoothiePlugin for MyPlugin {
//!     const NAME:    &'static str = "My Plugin";
//!     const VENDOR:  &'static str = "My Company";
//!     const VERSION: &'static str = "1.0.0";
//!     const UID:     PluginUid    = uid!("com.mycompany.myplugin");
//!
//!     fn audio_layouts() -> &'static [AudioLayout] {
//!         &[AudioLayout::stereo_in_stereo_out()]
//!     }
//!
//!     fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
//!         ctx.buffer_mut().apply_gain(self.gain);
//!         ProcessStatus::Normal
//!     }
//! }
//!
//! smoothie_export!(MyPlugin);
//! ```

#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

pub mod buffer;
pub mod context;
pub mod format;
pub mod layout;
pub mod plugin;
pub mod uid;

pub use plugin::SmoothiePlugin;
pub use buffer::AudioBuffer;
pub use context::{ProcessContext, InitContext};
pub use layout::{AudioLayout, ChannelCount};
pub use format::{PluginFormat, FormatFlags};
pub use uid::PluginUid;

/// Re-export everything needed to implement a plugin.
pub mod prelude {
    pub use crate::{
        SmoothiePlugin, AudioBuffer, ProcessContext, InitContext,
        AudioLayout, ChannelCount, PluginFormat, FormatFlags, PluginUid,
        ProcessStatus,
    };
    pub use crate::uid::uid;
    pub use smoothie_params::prelude::*;
    pub use smoothie_midi::prelude::*;
}

/// What the plugin wants the host to do after `process()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// Everything normal — keep calling `process`.
    Normal,
    /// Plugin is outputting silence and can be suspended.
    Tail(u32),
    /// Plugin is done, do not call `process` again until reset.
    KeepAlive,
    /// An unrecoverable error occurred.
    Error(&'static str),
}

/// Export macro — wires up all enabled plugin format exports in one line.
///
/// ```rust,no_run
/// smoothie_export!(MyPlugin);
/// // Expands to VST3 + CLAP + AU + AAX exports based on enabled features.
/// ```
#[macro_export]
macro_rules! smoothie_export {
    ($plugin:ty) => {
        #[cfg(feature = "vst3")]
        smoothie_vst3::export!($plugin);
        #[cfg(feature = "clap")]
        smoothie_clap::export!($plugin);
        #[cfg(feature = "au")]
        smoothie_au::export!($plugin);
        #[cfg(feature = "aax")]
        smoothie_aax::export!($plugin);
    };
}
