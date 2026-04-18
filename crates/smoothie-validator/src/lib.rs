//! # smoothie-validator
//!
//! The **Smoothie Elite Plugin Health Check Engine**.
//!
//! Validates a compiled plugin binary for:
//! - Format compatibility (VST3 / CLAP / AU / AAX)
//! - DAW-specific compatibility rules
//! - Common vulnerabilities (unsafe thread access, etc.)
//! - System requirements (macOS version, Windows version, etc.)
//! - Missing exports / symbol errors
//!
//! Used both by the `smoothie_elite_standalone` GUI and the `cargo-smoothie` CLI.

pub mod daw;
pub mod format;
pub mod report;
pub mod system;
pub mod vulnerability;

pub use report::{ValidationReport, CheckResult, CheckStatus, Severity};
pub use daw::DawCompatibility;
pub use format::FormatSupport;
pub use system::SystemRequirements;

use std::path::Path;

/// Run a full validation pass on a plugin binary.
///
/// # Arguments
/// * `plugin_path` — Path to the plugin bundle (.vst3, .clap, .component, .aaxplugin)
///
/// Returns a `ValidationReport` with the full results.
pub fn validate_plugin(plugin_path: &Path) -> ValidationReport {
    let mut report = ValidationReport::new(plugin_path);

    // 1. System requirements
    report.add_section("System", system::check(plugin_path));

    // 2. Format detection and validation
    report.add_section("Plugin Format", format::check(plugin_path));

    // 3. DAW compatibility
    report.add_section("DAW Compatibility", daw::check(plugin_path));

    // 4. Vulnerability scan
    report.add_section("Security", vulnerability::check(plugin_path));

    report
}

/// Run validation on the current running plugin (call from within the plugin).
pub fn self_validate() -> ValidationReport {
    let current_exe = std::env::current_exe().unwrap_or_default();
    validate_plugin(&current_exe)
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
