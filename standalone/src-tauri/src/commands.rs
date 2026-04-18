//! Tauri commands — called from TypeScript frontend via `invoke()`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub os:           String,
    pub arch:         String,
    pub os_version:   Option<String>,
    pub smoothie_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DawSupportEntry {
    pub name:   String,
    pub vst3:   bool,
    pub clap:   bool,
    pub au:     bool,
    pub aax:    bool,
    pub notes:  String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthReport {
    pub plugin_path: String,
    pub is_healthy:  bool,
    pub passed:      usize,
    pub warnings:    usize,
    pub failed:      usize,
    pub json:        String,
}

// ── Commands ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn validate_plugin(path: String) -> Result<HealthReport, String> {
    let path = PathBuf::from(&path);
    let report = smoothie_validator::validate_plugin(&path);
    Ok(HealthReport {
        plugin_path: path.display().to_string(),
        is_healthy:  report.is_healthy(),
        passed:      report.total_passed(),
        warnings:    report.total_warnings(),
        failed:      report.total_failed(),
        json:        report.to_json(),
    })
}

#[tauri::command]
pub async fn check_plugin_health() -> Result<HealthReport, String> {
    let report = smoothie_validator::self_validate();
    Ok(HealthReport {
        plugin_path: "self".to_string(),
        is_healthy:  report.is_healthy(),
        passed:      report.total_passed(),
        warnings:    report.total_warnings(),
        failed:      report.total_failed(),
        json:        report.to_json(),
    })
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let os_version = {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("sw_vers")
                .arg("-productVersion")
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        }
        #[cfg(not(target_os = "macos"))]
        None::<String>
    };

    Ok(SystemInfo {
        os:               std::env::consts::OS.to_string(),
        arch:             std::env::consts::ARCH.to_string(),
        os_version,
        smoothie_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tauri::command]
pub async fn get_daw_support() -> Result<Vec<DawSupportEntry>, String> {
    let is_mac = cfg!(target_os = "macos");

    Ok(vec![
        DawSupportEntry {
            name: "FL Studio".into(),
            vst3: true, clap: true, au: false, aax: false,
            notes: "VST3: all versions. CLAP: v21+".into(),
        },
        DawSupportEntry {
            name: "Ableton Live".into(),
            vst3: true, clap: false, au: is_mac, aax: false,
            notes: "AU only on macOS".into(),
        },
        DawSupportEntry {
            name: "Logic Pro".into(),
            vst3: false, clap: false, au: is_mac, aax: false,
            notes: "macOS only — AU required".into(),
        },
        DawSupportEntry {
            name: "Pro Tools".into(),
            vst3: false, clap: false, au: false, aax: true,
            notes: "AAX only — requires Avid SDK + signing".into(),
        },
        DawSupportEntry {
            name: "Reaper".into(),
            vst3: true, clap: true, au: is_mac, aax: false,
            notes: "Best multi-format DAW".into(),
        },
        DawSupportEntry {
            name: "Cubase / Nuendo".into(),
            vst3: true, clap: false, au: false, aax: false,
            notes: "VST3 — Steinberg's own format".into(),
        },
        DawSupportEntry {
            name: "Bitwig Studio".into(),
            vst3: true, clap: true, au: false, aax: false,
            notes: "Excellent CLAP support".into(),
        },
        DawSupportEntry {
            name: "Studio One".into(),
            vst3: true, clap: true, au: is_mac, aax: false,
            notes: "".into(),
        },
        DawSupportEntry {
            name: "GarageBand".into(),
            vst3: false, clap: false, au: is_mac, aax: false,
            notes: "macOS only — AU required".into(),
        },
    ])
}

#[tauri::command]
pub async fn get_framework_version() -> Result<String, String> {
    Ok(format!("Smoothie Elite v{}", env!("CARGO_PKG_VERSION")))
}

#[tauri::command]
pub async fn list_installed_plugins() -> Result<Vec<String>, String> {
    let mut plugins = Vec::new();

    // Check standard plugin directories
    let dirs: &[&str] = &[
        #[cfg(target_os = "macos")]
        "/Library/Audio/Plug-Ins/VST3",
        #[cfg(target_os = "macos")]
        "/Library/Audio/Plug-Ins/Components",
        #[cfg(target_os = "macos")]
        "~/Library/Audio/Plug-Ins/VST3",
        #[cfg(target_os = "windows")]
        "C:/Program Files/Common Files/VST3",
        #[cfg(target_os = "linux")]
        "/usr/lib/vst3",
    ];

    for dir in dirs {
        let path = PathBuf::from(dir);
        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries.flatten() {
                plugins.push(entry.path().display().to_string());
            }
        }
    }

    Ok(plugins)
}

#[tauri::command]
pub async fn open_plugin_folder() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("/Library/Audio/Plug-Ins/VST3")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
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
