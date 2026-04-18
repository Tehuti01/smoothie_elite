//! # smoothie-core
//!
//! The foundation of **Smoothie Elite** — the elite Rust audio plugin framework.

// #![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]

pub mod buffer;
pub mod context;
pub mod format;
pub mod silicon;
pub mod layout;
pub mod plugin;
pub mod uid;

pub use buffer::AudioBuffer;
pub use context::{InitContext, ProcessContext};
pub use format::{FormatFlags, PluginFormat};
pub use layout::{AudioLayout, ChannelCount};
pub use plugin::SmoothiePlugin;
pub use uid::PluginUid;

/// Re-export everything needed to implement a plugin.
pub mod prelude {
    pub use crate::silicon::*;
    pub use crate::uid::uid;
    pub use crate::{
        AudioBuffer, AudioLayout, ChannelCount, FormatFlags, InitContext, PluginFormat, PluginUid,
        ProcessContext, ProcessStatus, SmoothiePlugin,
    };
    pub use ::smoothie_params::prelude::*;
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


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
