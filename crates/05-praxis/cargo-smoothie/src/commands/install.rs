/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc6bc4a31 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/install.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the execute logic.
pub fn execute() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          Smoothie Elite — Install                ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let (vst3_dir, au_dir) = get_system_plugin_dirs();

    println!("  System plugin directories:");
    println!("    VST3: {}", vst3_dir);
    if !au_dir.is_empty() {
        println!("    AU:   {}", au_dir);
    }
    println!();

    let vst3_install_path = expand_home(vst3_dir);
    let au_install_path = expand_home(au_dir);
    let target_dir = std::path::Path::new("target/bundles");

    if !target_dir.exists() {
        println!("❌ Error: No bundled plugins found. Run `cargo smoothie bundle` first.");
        return;
    }

    println!("  Installing bundled plugins...");
    let mut installed_count = 0;

    // Scan target directory for .vst3 and .component folders
    if let Ok(entries) = std::fs::read_dir(target_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext == "vst3" {
                    let dest =
                        std::path::Path::new(&vst3_install_path).join(path.file_name().unwrap());
                    if copy_dir_recursive(&path, &dest) {
                        println!("    ✅ Installed VST3: {:?}", dest);
                        installed_count += 1;
                    }
                } else if ext == "component" && !au_dir.is_empty() {
                    let dest =
                        std::path::Path::new(&au_install_path).join(path.file_name().unwrap());
                    if copy_dir_recursive(&path, &dest) {
                        println!("    ✅ Installed AU: {:?}", dest);
                        installed_count += 1;
                    }
                }
            }
        }
    }

    if installed_count > 0 {
        println!(
            "\n  🎉 Successfully installed {} plugin(s).",
            installed_count
        );
    } else {
        println!("  ⚠️ No matching plugin bundles found in `target/bundles/`.");
    }
}

/// Recursively copy a directory natively (Handles `.vst3` and `.component` macOS bundles).
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> bool {
    if !dst.exists() {
        if let Err(e) = std::fs::create_dir_all(dst) {
            println!("❌ Failed to create directory {:?}: {}", dst, e);
            return false;
        }
    }

    if let Ok(entries) = std::fs::read_dir(src) {
        for entry in entries.flatten() {
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                if !copy_dir_recursive(&src_path, &dst_path) {
                    return false;
                }
            } else {
                if let Err(e) = std::fs::copy(&src_path, &dst_path) {
                    println!("❌ Failed to copy file {:?}: {}", src_path, e);
                    return false;
                }
            }
        }
        true
    } else {
        false
    }
}

/// Technical implementation of the expand_home logic.
fn expand_home(path: &str) -> std::string::String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}

/// Technical implementation of the get_system_plugin_dirs logic.
fn get_system_plugin_dirs() -> (&'static str, &'static str) {
    if cfg!(target_os = "macos") {
        (
            "~/Library/Audio/Plug-Ins/VST3/",
            "~/Library/Audio/Plug-Ins/Components/",
        )
    } else if cfg!(target_os = "windows") {
        ("C:\\Program Files\\Common Files\\VST3\\", "")
    } else {
        ("~/.vst3/", "")
    }
}
