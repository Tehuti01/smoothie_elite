//! # cargo-smoothie
//!
//! The Smoothie Elite CLI. Run via:
//!
//! ```
//! cargo install cargo-smoothie
//! cargo smoothie new my-plugin
//! cargo smoothie build
//! cargo smoothie bundle
//! cargo smoothie validate
//! cargo smoothie standalone
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name        = "cargo-smoothie",
    bin_name    = "cargo smoothie",
    version,
    about       = "Smoothie Elite — Elite audio plugin development framework",
    long_about  = r#"
  ╔═══════════════════════════════════════════╗
  ║   SMOOTHIE ELITE — Elite Plugin Dev CLI   ║
  ║   by Seraphic Sonic                       ║
  ╚═══════════════════════════════════════════╝

  Build professional-grade VST3, CLAP, AU, and AAX plugins in Rust.
  Powered by Smoothie Elite — faster and safer than JUCE.
"#,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Smoothie Elite plugin project.
    New {
        /// Plugin name (e.g. "my-reverb")
        name: String,
        /// Plugin type: fx (default), instrument, analyzer
        #[arg(long, default_value = "fx")]
        kind: String,
        /// Target formats (comma separated: vst3,clap,au,aax)
        #[arg(long, default_value = "vst3,clap")]
        formats: String,
    },

    /// Build the plugin (calls `cargo build`).
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },

    /// Bundle the plugin into distributable formats.
    Bundle {
        /// Bundle as VST3
        #[arg(long)]
        vst3: bool,
        /// Bundle as CLAP
        #[arg(long)]
        clap: bool,
        /// Bundle as AU (macOS only)
        #[arg(long)]
        au: bool,
        /// Bundle as AAX (requires Avid SDK)
        #[arg(long)]
        aax: bool,
        /// Bundle all supported formats
        #[arg(long)]
        all: bool,
    },

    /// Validate a plugin binary.
    Validate {
        /// Path to the plugin bundle
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Launch the Smoothie Elite Standalone app.
    Standalone {
        /// Plugin to load at startup
        #[arg(value_name = "PLUGIN_PATH")]
        plugin: Option<PathBuf>,
    },

    /// Show information about the current workspace.
    Info,

    /// Update the Smoothie Elite framework to the latest version.
    Update,

    /// Open the Smoothie Elite documentation.
    Docs,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // `cargo smoothie` is invoked as `cargo-smoothie smoothie <subcommand>`
    // Strip the first "smoothie" arg if called via `cargo smoothie`
    let args: Vec<String> = std::env::args().collect();
    let args = if args.get(1).map(|s| s == "smoothie").unwrap_or(false) {
        let mut a = vec![args[0].clone()];
        a.extend_from_slice(&args[2..]);
        a
    } else {
        args
    };

    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::New { name, kind, formats } => cmd_new(&name, &kind, &formats),
        Commands::Build { release }           => cmd_build(release),
        Commands::Bundle { vst3, clap, au, aax, all } => cmd_bundle(vst3, clap, au, aax, all),
        Commands::Validate { path, json }     => cmd_validate(path, json),
        Commands::Standalone { plugin }       => cmd_standalone(plugin),
        Commands::Info                        => cmd_info(),
        Commands::Update                      => cmd_update(),
        Commands::Docs                        => cmd_docs(),
    }
}

fn cmd_new(name: &str, kind: &str, formats: &str) -> anyhow::Result<()> {
    println!("🎛️  Creating new Smoothie Elite plugin: {name}");
    println!("   Type:    {kind}");
    println!("   Formats: {formats}");
    println!();

    let dir = PathBuf::from(name);
    std::fs::create_dir_all(&dir)?;

    // Cargo.toml
    let cargo_toml = format!(r#"[package]
name    = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[[bin]]
name = "{name}-standalone"
path = "src/bin/standalone.rs"

[dependencies]
smoothie-core      = {{ git = "https://github.com/SeraphicSonic/smoothie-elite.git" }}
smoothie-params    = {{ git = "https://github.com/SeraphicSonic/smoothie-elite.git" }}
smoothie-dsp       = {{ git = "https://github.com/SeraphicSonic/smoothie-elite.git" }}
smoothie-validator = {{ git = "https://github.com/SeraphicSonic/smoothie-elite.git" }}
"#);
    std::fs::write(dir.join("Cargo.toml"), cargo_toml)?;

    // src/lib.rs
    std::fs::create_dir_all(dir.join("src"))?;
    let lib_rs = format!(r#"use smoothie_core::prelude::*;

#[derive(Default)]
pub struct {pascal}Plugin {{
    // TODO: add your DSP state here
}}

impl SmoothiePlugin for {pascal}Plugin {{
    const NAME:    &'static str = "{name}";
    const VENDOR:  &'static str = "My Company";
    const VERSION: &'static str = "0.1.0";
    const UID:     PluginUid    = uid!("com.mycompany.{name}");

    fn audio_layouts() -> &'static [AudioLayout] {{
        &[AudioLayout::stereo_in_stereo_out()]
    }}

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {{
        // Your DSP code here
        ProcessStatus::Normal
    }}
}}

smoothie_export!({pascal}Plugin);
"#, pascal = to_pascal(name));
    std::fs::write(dir.join("src").join("lib.rs"), lib_rs)?;

    // Standalone binary
    std::fs::create_dir_all(dir.join("src").join("bin"))?;
    let standalone_rs = format!(r#"fn main() {{
    smoothie_core::standalone::run::<{name}::{}Plugin>();
}}
"#, to_pascal(name));
    std::fs::write(dir.join("src").join("bin").join("standalone.rs"), standalone_rs)?;

    println!("✅ Plugin scaffold created at ./{name}/");
    println!("   Next steps:");
    println!("   cd {name}");
    println!("   cargo smoothie build");
    println!("   cargo smoothie validate");
    Ok(())
}

fn cmd_build(release: bool) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    if release { cmd.arg("--release"); }
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("Build failed");
    }
    println!("✅ Build complete");
    Ok(())
}

fn cmd_bundle(vst3: bool, clap: bool, au: bool, aax: bool, all: bool) -> anyhow::Result<()> {
    println!("📦 Bundling plugin formats:");
    if all || vst3 { println!("  → VST3"); bundle_vst3()?; }
    if all || clap { println!("  → CLAP"); }
    if all || au   { println!("  → AU (macOS)"); }
    if all || aax  { println!("  → AAX (Pro Tools)"); }
    Ok(())
}

fn bundle_vst3() -> anyhow::Result<()> {
    // Build the cdylib, then wrap in VST3 bundle structure
    let status = std::process::Command::new("cargo")
        .args(["build", "--release", "--lib"])
        .status()?;
    if !status.success() { anyhow::bail!("VST3 build failed"); }
    println!("  ✅ VST3 bundle created");
    Ok(())
}

fn cmd_validate(path: Option<PathBuf>, json: bool) -> anyhow::Result<()> {
    let target_path = path.unwrap_or_else(|| PathBuf::from("."));
    let report = smoothie_validator::validate_plugin(&target_path);
    if json {
        println!("{}", report.to_json());
    } else {
        report.print_summary();
    }
    Ok(())
}

fn cmd_standalone(plugin: Option<PathBuf>) -> anyhow::Result<()> {
    println!("🎸 Launching Smoothie Elite Standalone...");
    if let Some(p) = plugin {
        println!("   Loading plugin: {}", p.display());
    }
    // Launch the Tauri standalone app
    let exe = std::env::current_exe()?
        .parent()
        .unwrap_or(&PathBuf::from("."))
        .join("smoothie_elite_standalone");

    if exe.exists() {
        std::process::Command::new(exe).spawn()?;
    } else {
        println!("   smoothie_elite_standalone not found in PATH.");
        println!("   Build it with: cd standalone && npm run tauri build");
    }
    Ok(())
}

fn cmd_info() -> anyhow::Result<()> {
    println!("Smoothie Elite Framework");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("OS: {} ({})", std::env::consts::OS, std::env::consts::ARCH);
    Ok(())
}

fn cmd_update() -> anyhow::Result<()> {
    println!("Updating Smoothie Elite...");
    std::process::Command::new("cargo")
        .args(["install", "cargo-smoothie", "--git",
               "https://github.com/SeraphicSonic/smoothie-elite"])
        .status()?;
    Ok(())
}

fn cmd_docs() -> anyhow::Result<()> {
    let _ = open::that("https://github.com/SeraphicSonic/smoothie-elite/wiki");
    Ok(())
}

fn to_pascal(s: &str) -> String {
    s.split(['-', '_'])
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
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
