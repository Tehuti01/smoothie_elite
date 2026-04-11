//! Plugin format detection and validation.

use std::path::Path;
use crate::report::CheckResult;

pub struct FormatSupport {
    pub vst3: bool,
    pub clap: bool,
    pub au:   bool,
    pub aax:  bool,
}

/// Detect what format a plugin binary is, then validate its structure.
pub fn check(path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

    match ext.as_str() {
        "vst3" => {
            results.push(check_vst3(path));
            results.push(CheckResult::pass("CLAP", "N/A for .vst3 bundle").not_applicable());
            results.push(CheckResult::pass("AU",   "N/A for .vst3 bundle").not_applicable());
        }
        "clap" => {
            results.push(check_clap(path));
        }
        "component" => {
            results.push(check_au(path));
        }
        "aaxplugin" => {
            results.push(check_aax(path));
        }
        _ => {
            // Standalone executable — check all
            results.push(check_vst3(path));
            results.push(check_clap(path));
        }
    }

    results
}

fn check_vst3(path: &Path) -> CheckResult {
    // Check for VST3 bundle structure: Contents/MacOS/PluginName (macOS)
    // or Contents/x86_64-win/PluginName.vst3 (Windows)
    let bundle = if path.extension().map(|e| e == "vst3").unwrap_or(false) {
        path.to_path_buf()
    } else {
        path.with_extension("vst3")
    };

    if bundle.exists() {
        let contents = bundle.join("Contents");
        if contents.exists() {
            CheckResult::pass("VST3", "VST3 bundle structure found")
        } else {
            CheckResult::fail("VST3", "VST3 bundle missing Contents/ directory")
                .with_fix("Run `cargo smoothie bundle --vst3` to create the bundle")
        }
    } else {
        CheckResult::warn("VST3", "VST3 bundle not found — not yet built")
            .with_fix("Run `cargo smoothie bundle --vst3`")
    }
}

fn check_clap(path: &Path) -> CheckResult {
    let bundle = if path.extension().map(|e| e == "clap").unwrap_or(false) {
        path.to_path_buf()
    } else {
        path.with_extension("clap")
    };

    if bundle.exists() {
        CheckResult::pass("CLAP", "CLAP plugin file found")
    } else {
        CheckResult::warn("CLAP", "CLAP plugin not found — not yet built")
            .with_fix("Run `cargo smoothie bundle --clap`")
    }
}

fn check_au(path: &Path) -> CheckResult {
    #[cfg(target_os = "macos")]
    {
        let plist = path.join("Contents").join("Info.plist");
        if plist.exists() {
            return CheckResult::pass("AU", "Audio Unit bundle structure valid");
        }
        CheckResult::fail("AU", "AU bundle missing Info.plist")
    }
    #[cfg(not(target_os = "macos"))]
    CheckResult::pass("AU", "AU — only supported on macOS").not_applicable()
}

fn check_aax(_path: &Path) -> CheckResult {
    CheckResult::warn("AAX", "AAX requires Avid Developer SDK — not open source")
        .with_detail("Pro Tools AAX requires signing with Avid's certificate")
        .with_fix("See: https://developer.avid.com/aax")
}

// Extension for not-applicable checks
trait NotApplicable {
    fn not_applicable(self) -> Self;
}

impl NotApplicable for CheckResult {
    fn not_applicable(mut self) -> Self {
        self.status = crate::report::CheckStatus::NotApplicable;
        self
    }
}
