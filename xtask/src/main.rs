//! # cargo xtask — Smoothie Elite build automation
//!
//! Usage:
//! ```
//! cargo xtask bundle [crate]          # Bundle all formats (VST3, CLAP, standalone)
//! cargo xtask bundle --vst3           # VST3 only
//! cargo xtask bundle --clap           # CLAP only
//! cargo xtask bundle --standalone     # Standalone binary
//! cargo xtask validate [crate]        # Run plugin health checks
//! cargo xtask install                 # Install into system plugin folders
//! cargo xtask clippy                  # Run clippy on all crates
//! cargo xtask test                    # Run all tests
//! cargo xtask clean                   # Clean all build artifacts
//! ```

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(name = "xtask", about = "Smoothie Elite build system")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Bundle plugin into VST3/CLAP/standalone formats.
    Bundle {
        /// Name of the crate to bundle (defaults to workspace root)
        #[arg(value_name = "CRATE")]
        package: Option<String>,
        #[arg(long)] vst3:       bool,
        #[arg(long)] clap:       bool,
        #[arg(long)] standalone: bool,
        #[arg(long)] au:         bool,
        #[arg(long, short = 'r')] release: bool,
    },
    /// Validate a plugin binary with the health checker.
    Validate {
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
    },
    /// Install bundles to system plugin directories.
    Install {
        #[arg(value_name = "BUNDLE")] bundle: Option<PathBuf>,
    },
    /// Run clippy on all workspace crates.
    Clippy,
    /// Run all workspace tests.
    Test,
    /// Clean build artifacts.
    Clean,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sh = Shell::new()?;

    // Find workspace root
    let workspace_root = workspace_root()?;
    sh.change_dir(&workspace_root);

    match args.command {
        Command::Bundle { package, vst3, clap, standalone, au, release } => {
            let profile = if release { "release" } else { "dev" };
            let all = !vst3 && !clap && !standalone && !au;

            if all || vst3       { bundle_vst3(&sh, &package, profile)?; }
            if all || clap       { bundle_clap(&sh, &package, profile)?; }
            if all || standalone { bundle_standalone(&sh, &package, profile)?; }
            if all || au         { bundle_au(&sh, &package, profile)?; }

            println!("\n✅ All bundles complete.");
        }
        Command::Validate { path } => {
            let p = path.unwrap_or(workspace_root.join("target").join("debug"));
            println!("Running validation on: {}", p.display());
            cmd!(sh, "cargo run -p cargo-smoothie -- validate {p}").run()?;
        }
        Command::Install { bundle } => {
            install_plugins(&sh, bundle)?;
        }
        Command::Clippy => {
            cmd!(sh, "cargo clippy --workspace --all-targets -- -D warnings").run()?;
        }
        Command::Test => {
            cmd!(sh, "cargo test --workspace").run()?;
        }
        Command::Clean => {
            cmd!(sh, "cargo clean").run()?;
            println!("✅ Cleaned.");
        }
    }

    Ok(())
}

// ── Bundle targets ────────────────────────────────────────────────────────────

fn bundle_vst3(sh: &Shell, package: &Option<String>, profile: &str) -> Result<()> {
    println!("📦 Bundling VST3...");

    let pkg_args = package_args(package);
    if profile == "release" {
        cmd!(sh, "cargo build --release --lib {pkg_args...}").run()?;
    } else {
        cmd!(sh, "cargo build --lib {pkg_args...}").run()?;
    }

    let out_dir = target_dir(profile);
    let lib = find_cdylib(&out_dir)?;
    let bundle_name = lib.file_stem().unwrap_or_default().to_string_lossy().replace("lib", "");
    let bundle_path = out_dir.join(format!("{}.vst3", bundle_name));

    // macOS VST3 bundle structure
    #[cfg(target_os = "macos")]
    {
        let contents = bundle_path.join("Contents").join("MacOS");
        std::fs::create_dir_all(&contents)?;
        std::fs::copy(&lib, contents.join(&bundle_name))?;
        write_vst3_plist(&bundle_path.join("Contents"), &bundle_name)?;
        println!("  ✅ VST3: {}", bundle_path.display());
    }

    #[cfg(target_os = "windows")]
    {
        let arch_dir = bundle_path.join("Contents").join("x86_64-win");
        std::fs::create_dir_all(&arch_dir)?;
        std::fs::copy(&lib, arch_dir.join(format!("{}.vst3", bundle_name)))?;
        println!("  ✅ VST3: {}", bundle_path.display());
    }

    #[cfg(target_os = "linux")]
    {
        let arch_dir = bundle_path.join("Contents").join("x86_64-linux");
        std::fs::create_dir_all(&arch_dir)?;
        std::fs::copy(&lib, arch_dir.join(format!("{}.so", bundle_name)))?;
        println!("  ✅ VST3: {}", bundle_path.display());
    }

    Ok(())
}

fn bundle_clap(sh: &Shell, package: &Option<String>, profile: &str) -> Result<()> {
    println!("📦 Bundling CLAP...");

    let pkg_args = package_args(package);
    if profile == "release" {
        cmd!(sh, "cargo build --release --lib {pkg_args...}").run()?;
    } else {
        cmd!(sh, "cargo build --lib {pkg_args...}").run()?;
    }

    let out_dir = target_dir(profile);
    let lib = find_cdylib(&out_dir)?;
    let bundle_name = lib.file_stem().unwrap_or_default().to_string_lossy().replace("lib", "");

    #[cfg(unix)]
    {
        let clap_path = out_dir.join(format!("{}.clap", bundle_name));
        std::fs::copy(&lib, &clap_path)?;
        println!("  ✅ CLAP: {}", clap_path.display());
    }
    #[cfg(windows)]
    {
        let clap_path = out_dir.join(format!("{}.clap", bundle_name));
        std::fs::copy(&lib, &clap_path)?;
        println!("  ✅ CLAP: {}", clap_path.display());
    }

    Ok(())
}

fn bundle_standalone(sh: &Shell, package: &Option<String>, profile: &str) -> Result<()> {
    println!("📦 Bundling Standalone...");
    let pkg_args = package_args(package);
    if profile == "release" {
        cmd!(sh, "cargo build --release --bins {pkg_args...}").run()?;
    } else {
        cmd!(sh, "cargo build --bins {pkg_args...}").run()?;
    }
    println!("  ✅ Standalone binary built in target/{}/", profile);
    Ok(())
}

fn bundle_au(sh: &Shell, _package: &Option<String>, _profile: &str) -> Result<()> {
    #[cfg(not(target_os = "macos"))]
    {
        println!("  — AU: macOS only, skipping on this platform.");
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        println!("📦 Bundling AU... (stub — full AU wrapper coming in v0.2)");
        Ok(())
    }
}

fn install_plugins(sh: &Shell, bundle: Option<PathBuf>) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let vst3_dir = PathBuf::from("/Library/Audio/Plug-Ins/VST3");
        if let Some(b) = bundle {
            println!("Installing {} → {}", b.display(), vst3_dir.display());
            cmd!(sh, "cp -r {b} {vst3_dir}").run()?;
        } else {
            let out = target_dir("release");
            for entry in std::fs::read_dir(&out)? {
                let entry = entry?;
                let path  = entry.path();
                if path.extension().map(|e| e == "vst3").unwrap_or(false) {
                    cmd!(sh, "cp -r {path} {vst3_dir}").run()?;
                    println!("  ✅ Installed: {}", path.display());
                }
            }
        }
    }
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn workspace_root() -> Result<PathBuf> {
    let output = std::process::Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version=1"])
        .output()
        .context("cargo metadata failed")?;
    let meta: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let root = meta["workspace_root"].as_str().context("no workspace_root")?;
    Ok(PathBuf::from(root))
}

fn target_dir(profile: &str) -> PathBuf {
    let profile = if profile == "dev" { "debug" } else { profile };
    PathBuf::from("target").join(profile)
}

fn find_cdylib(dir: &Path) -> Result<PathBuf> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path  = entry.path();
        let ext   = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if matches!(ext, "dylib" | "dll" | "so") {
            return Ok(path);
        }
    }
    bail!("No cdylib found in {}", dir.display())
}

fn package_args(package: &Option<String>) -> Vec<String> {
    match package {
        Some(p) => vec!["--package".to_string(), p.clone()],
        None    => vec![],
    }
}

#[cfg(target_os = "macos")]
fn write_vst3_plist(contents_dir: &Path, name: &str) -> Result<()> {
    let plist = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>    <string>{name}</string>
    <key>CFBundleIdentifier</key>    <string>com.seraphicsonic.{name}</string>
    <key>CFBundleName</key>          <string>{name}</string>
    <key>CFBundlePackageType</key>   <string>BNDL</string>
    <key>CFBundleShortVersionString</key> <string>0.1.0</string>
    <key>CFBundleVersion</key>       <string>1</string>
</dict>
</plist>
"#);
    std::fs::write(contents_dir.join("Info.plist"), plist)?;
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
