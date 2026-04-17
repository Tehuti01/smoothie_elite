//! # smoothie-clap
//!
//! High-performance CLAP (Clever Audio Plug-in) wrapper for **Smoothie Elite**.

use smoothie_core::prelude::*;
use clap_sys::plugin::*;
use clap_sys::host::clap_host;
use clap_sys::id::*;
use parking_lot::RwLock;
use std::sync::Arc;

/// A CLAP plugin instance.
pub struct SmtClapPlugin<P: SmoothiePlugin> {
    pub inner: Arc<RwLock<P>>,
    pub host: *const clap_host,
    pub sample_rate: f64,
}

impl<P: SmoothiePlugin> SmtClapPlugin<P> {
    pub fn new(host: *const clap_host, plugin: Arc<RwLock<P>>) -> Self {
        Self {
            inner: plugin,
            host,
            sample_rate: 44100.0,
        }
    }
}

/// Macro to export a Smoothie plugin as a CLAP library.
#[macro_export]
macro_rules! export_clap {
    ($plugin:ty) => {
        use clap_sys::entry::*;
        use clap_sys::plugin::*;
        use clap_sys::host::*;

        #[no_mangle]
        pub static mut clap_entry: clap_plugin_entry = clap_plugin_entry {
            clap_version: CLAP_VERSION,
            init: Some(clap_init),
            deinit: Some(clap_deinit),
            get_factory: Some(clap_get_factory),
        };

        extern "C" fn clap_init(_host_path: *const std::os::raw::c_char) -> bool {
            true
        }

        extern "C" fn clap_deinit() {}

        extern "C" fn clap_get_factory(factory_id: *const std::os::raw::c_char) -> *const std::ffi::c_void {
            // Implementation of clap_plugin_factory for $plugin
            std::ptr::null()
        }
    };
}
