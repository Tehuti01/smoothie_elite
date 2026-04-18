//! # smoothie-clap
//!
//! High-performance CLAP (Clever Audio Plug-in) wrapper for **Smoothie Elite**.

pub mod clap_sys;

pub use crate::clap_sys::{ClapPlugin, ClapPluginDescriptor, CLAP_VERSION, ClapHost};
use smoothie_core::prelude::*;
use smoothie_params::Param;
use parking_lot::RwLock;
use std::sync::Arc;
use std::ffi::{c_char, c_void};

/// A CLAP plugin instance.
pub struct SmtClapPlugin<P: SmoothiePlugin> {
    pub inner: Arc<RwLock<P>>,
    pub host: *const ClapHost,
    pub sample_rate: f64,
}

impl<P: SmoothiePlugin> SmtClapPlugin<P> {
    /// Create a new CLAP wrapper for a plugin.
    pub fn new(host: *const ClapHost, plugin: Arc<RwLock<P>>) -> Self {
        Self {
            inner: plugin,
            host,
            sample_rate: 44100.0,
        }
    }
}

/// Macro to export a Smoothie plugin as a CLAP library using the virtualized ABI.
#[macro_export]
macro_rules! export {
    ($plugin:ty) => {
        #[no_mangle]
        pub static mut clap_entry: $crate::clap_sys::ClapPluginDescriptor = $crate::clap_sys::ClapPluginDescriptor {
            clap_version: $crate::clap_sys::CLAP_VERSION,
            id: b"com.smoothie.plugin\0".as_ptr() as *const i8,
            name: b"Smoothie Plugin\0".as_ptr() as *const i8,
            vendor: b"Smoothie Elite\0".as_ptr() as *const i8,
            url: b"https://smoothie.elite\0".as_ptr() as *const i8,
            manual_url: std::ptr::null(),
            support_url: std::ptr::null(),
            version: b"1.0.0\0".as_ptr() as *const i8,
            description: b"Smoothie Elite Virtualized Plugin\0".as_ptr() as *const i8,
            features: std::ptr::null(),
        };

        extern "C" fn clap_init(_host_path: *const std::os::raw::c_char) -> bool {
            true
        }

        extern "C" fn clap_deinit() {}
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
