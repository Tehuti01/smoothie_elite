//! smoothie_elite_standalone — Seraphic Sonic's elite plugin host & development tool.
//!
//! This is NOT "Tauri". This is **Smoothie Elite Standalone**.
//! Built on Tauri's engine, but fully branded as Seraphic Sonic.

use tauri::Manager;

pub mod commands;

pub fn run() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::validate_plugin,
            commands::get_system_info,
            commands::get_daw_support,
            commands::get_framework_version,
            commands::check_plugin_health,
            commands::list_installed_plugins,
            commands::open_plugin_folder,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // Override default Tauri title — this is Smoothie Elite, not Tauri
            window.set_title("Smoothie Elite Standalone — by Seraphic Sonic").ok();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Smoothie Elite Standalone");
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
