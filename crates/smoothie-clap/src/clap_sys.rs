//! smoothie-clap — Virtualized CLAP ABI.
//! Native implementation of the CLAP C-ABI to bypass dependency blockers.

use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClapVersion {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

pub const CLAP_VERSION: ClapVersion = ClapVersion { major: 1, minor: 1, revision: 10 };

pub type ClapId = u32;

#[repr(C)]
pub struct ClapPluginDescriptor {
    pub clap_version: ClapVersion,
    pub id: *const c_char,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub url: *const c_char,
    pub manual_url: *const c_char,
    pub support_url: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub features: *const *const c_char,
}

#[repr(C)]
pub struct ClapHost {
    pub clap_version: ClapVersion,
    pub host_data: *mut std::ffi::c_void,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub url: *const c_char,
    pub version: *const c_char,
    pub get_extension: unsafe extern "C" fn(host: *const ClapHost, extension_id: *const c_char) -> *const std::ffi::c_void,
    pub request_restart: unsafe extern "C" fn(host: *const ClapHost),
    pub request_process: unsafe extern "C" fn(host: *const ClapHost),
    pub request_callback: unsafe extern "C" fn(host: *const ClapHost),
}

#[repr(C)]
pub struct ClapPlugin {
    pub desc: *const ClapPluginDescriptor,
    pub plugin_data: *mut std::ffi::c_void,
    pub init: unsafe extern "C" fn(plugin: *const ClapPlugin) -> bool,
    pub destroy: unsafe extern "C" fn(plugin: *const ClapPlugin),
    pub activate: unsafe extern "C" fn(plugin: *const ClapPlugin, sample_rate: f64, min_frames_count: u32, max_frames_count: u32) -> bool,
    pub deactivate: unsafe extern "C" fn(plugin: *const ClapPlugin),
    pub start_processing: unsafe extern "C" fn(plugin: *const ClapPlugin) -> bool,
    pub stop_processing: unsafe extern "C" fn(plugin: *const ClapPlugin),
    pub process: unsafe extern "C" fn(plugin: *const ClapPlugin, process: *const std::ffi::c_void) -> i32,
    pub get_extension: unsafe extern "C" fn(plugin: *const ClapPlugin, id: *const c_char) -> *const std::ffi::c_void,
    pub on_main_thread: unsafe extern "C" fn(plugin: *const ClapPlugin),
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
