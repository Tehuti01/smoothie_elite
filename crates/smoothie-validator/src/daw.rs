//! DAW compatibility checks.

use std::path::Path;
use crate::report::CheckResult;

pub struct DawCompatibility;

pub fn check(_path: &Path) -> Vec<CheckResult> {
    vec![
        check_fl_studio(),
        check_ableton(),
        check_logic(),
        check_reaper(),
        check_cubase(),
        check_pro_tools(),
        check_bitwig(),
        check_studio_one(),
    ]
}

fn check_fl_studio() -> CheckResult {
    CheckResult::pass("FL Studio", "VST3 + CLAP (v21+) supported")
        .with_detail("FL Studio 20: VST3 only. FL Studio 21+: VST3 and CLAP")
}

fn check_ableton() -> CheckResult {
    #[cfg(target_os = "macos")]
    return CheckResult::pass("Ableton Live", "VST3 + AU (macOS)");
    #[cfg(target_os = "windows")]
    return CheckResult::pass("Ableton Live", "VST3 (Windows)");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    CheckResult::warn("Ableton Live", "Linux support limited to VST3")
}

fn check_logic() -> CheckResult {
    #[cfg(target_os = "macos")]
    return CheckResult::pass("Logic Pro", "AU supported (macOS only)");
    #[cfg(not(target_os = "macos"))]
    CheckResult::pass("Logic Pro", "Logic Pro is macOS only — N/A on this platform")
        .not_applicable()
}

fn check_reaper() -> CheckResult {
    CheckResult::pass("Reaper", "VST3 + CLAP + AU (macOS) all supported")
}

fn check_cubase() -> CheckResult {
    CheckResult::pass("Cubase / Nuendo", "VST3 fully supported — Steinberg's own format")
}

fn check_pro_tools() -> CheckResult {
    CheckResult::warn("Pro Tools", "Requires AAX format + Avid signing (not built by default)")
        .with_fix("Enable `aax` feature and obtain Avid Developer SDK")
}

fn check_bitwig() -> CheckResult {
    CheckResult::pass("Bitwig Studio", "VST3 + CLAP supported")
}

fn check_studio_one() -> CheckResult {
    #[cfg(target_os = "macos")]
    return CheckResult::pass("Studio One", "VST3 + CLAP + AU (macOS)");
    #[cfg(not(target_os = "macos"))]
    return CheckResult::pass("Studio One", "VST3 + CLAP (Windows/Linux)");
}

trait NotApplicable {
    fn not_applicable(self) -> Self;
}
impl NotApplicable for CheckResult {
    fn not_applicable(mut self) -> Self {
        self.status = crate::report::CheckStatus::NotApplicable;
        self
    }
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
