/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5903af74 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/main.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;

mod commands;

#[derive(Parser)]
#[command(name = "cargo-smoothie")]
#[command(author = "Smoothie Audio <smoothie@smoothieaudio.dev>")]
#[command(version = "1.0.0-LTS")]
#[command(about = "The Autonomous CLI for the Smoothie Elite Audio Plugin Framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Smoothie Elite plugin project
    New {
        /// Name of the project
        name: String,
        /// Template to use (effect, instrument, analyzer, utility)
        #[arg(short, long, default_value = "effect")]
        template: String,
        /// Output directory (optional)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Build the plugin in release mode with Seraphic optimizations
    Build {
        /// Build for a specific format (vst3, au, aax, clap)
        #[arg(short, long)]
        format: Option<String>,
        /// Enable release mode
        #[arg(short, long, default_value = "true")]
        release: bool,
    },
    /// Validate the plugin against the Seraphic Specification (L0, A0, PHI)
    Validate {
        /// Path to the plugin binary or project
        path: Option<PathBuf>,
    },
    /// Display information about the current Smoothie environment
    Info,
    /// Bundle the plugin into distributable installers
    Bundle {
        /// Enable release mode
        #[arg(short, long, default_value = "true")]
        release: bool,
        /// Platform to bundle for (windows, macos, linux)
        #[arg(short, long)]
        platform: Option<String>,
    },
    /// Install the cargo-smoothie extension system-wide
    Install,
    /// Run high-fidelity tests on the plugin DSP
    Test,
    /// Generate the Seraphic Documentation Hub
    Doc {
        /// Output directory for the documentation
        #[arg(short, long, default_value = "target/doc_hub")]
        output: PathBuf,
        /// Open the documentation in a browser after generation
        #[arg(long)]
        open: bool,
    },
}

/// Technical implementation of the main logic.
fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            template,
            path,
        } => {
            commands::new::execute(&name, &template, path.as_deref());
        }
        Commands::Build { format, release } => {
            commands::build::execute(release, format.as_deref());
        }
        Commands::Validate { path } => {
            commands::validate::execute(path);
        }
        Commands::Info => {
            commands::info::execute();
        }
        Commands::Bundle { release, platform } => {
            let is_macos = platform.as_deref() == Some("macos");
            commands::bundle::execute(release, true, true, is_macos, true);
        }
        Commands::Install => {
            commands::install::execute();
        }
        Commands::Test => {
            commands::test::execute();
        }
        Commands::Doc { output, open } => {
            println!(
                "{} Generating Seraphic Documentation Hub...",
                "🚀".bright_cyan()
            );

            // Execute the Python Static Site Generator (Internal path for now)
            let status = Command::new("python3")
                .arg(".skill.os.Seraphic/05-Holographic-Aesthetics/section-16-nexus-forge/04-Commands/forge_nexus.py")
                .status()
                .expect("Failed to execute Nexus Forge");

            if status.success() {
                println!(
                    "{} Documentation forged successfully at: {}",
                    "✓".green(),
                    output.display()
                );
                if open {
                    #[cfg(target_os = "macos")]
                    let _ = Command::new("open").arg(output.join("index.html")).status();
                }
            } else {
                eprintln!("{} Error forging the Nexus Hub.", "❌".red());
            }
        }
    }
}
