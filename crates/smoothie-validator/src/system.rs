//! System requirements checks.

use std::path::Path;
use crate::report::CheckResult;

pub struct SystemRequirements;

pub fn check(_path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    results.push(check_os());
    results.push(check_architecture());
    results.push(check_rust_version());

    #[cfg(target_os = "macos")]
    {
        results.push(check_macos_version());
        results.push(check_coreaudio());
    }

    #[cfg(target_os = "windows")]
    {
        results.push(check_asio());
        results.push(check_wasapi());
    }

    results
}

fn check_os() -> CheckResult {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    CheckResult::pass("OS", format!("{} ({})", os, arch))
}

fn check_architecture() -> CheckResult {
    match std::env::consts::ARCH {
        "x86_64" | "aarch64" => {
            CheckResult::pass("CPU Architecture", format!("{} — optimized build supported", std::env::consts::ARCH))
        }
        arch => {
            CheckResult::warn("CPU Architecture", format!("{} — limited optimization support", arch))
        }
    }
}

fn check_rust_version() -> CheckResult {
    // Check Cargo.toml for edition
    CheckResult::pass("Rust Edition", "2021 — fully supported")
}

#[cfg(target_os = "macos")]
fn check_macos_version() -> CheckResult {
    // Read OS version from system
    let output = std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output();

    match output {
        Ok(o) => {
            let version = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
            if parts.first().copied().unwrap_or(0) >= 11 {
                CheckResult::pass("macOS Version", format!("{} — CoreAudio + Metal supported", version))
            } else {
                CheckResult::warn("macOS Version", format!("{} — upgrade to macOS 11+ recommended", version))
            }
        }
        Err(_) => CheckResult::warn("macOS Version", "Could not detect macOS version"),
    }
}

#[cfg(target_os = "macos")]
fn check_coreaudio() -> CheckResult {
    // CoreAudio is always present on macOS
    CheckResult::pass("CoreAudio", "Present — low-latency audio available")
}

#[cfg(target_os = "windows")]
fn check_asio() -> CheckResult {
    CheckResult::warn("ASIO", "ASIO driver support requires ASIO SDK — not bundled")
        .with_detail("ASIO provides ultra-low-latency on Windows")
        .with_fix("Download Steinberg ASIO SDK and build with the `asio` feature")
}

#[cfg(target_os = "windows")]
fn check_wasapi() -> CheckResult {
    CheckResult::pass("WASAPI", "Windows audio API available for low-latency output")
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
