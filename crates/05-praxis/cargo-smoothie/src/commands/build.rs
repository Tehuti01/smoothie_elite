/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x848f5fa0 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/build.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use std::process::Command;

/// Technical implementation of the execute logic.
pub fn execute(release: bool, format: Option<&str>) {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          Smoothie Elite — Build                  ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let mut args = vec!["build"];

    if release {
        args.push("--release");
        println!("  Mode: Release (optimized)");
    } else {
        println!("  Mode: Debug");
    }

    if let Some(fmt) = format {
        println!("  Format: {}", fmt);
        // Set environment variable for format-specific compilation
        std::env::set_var("SMOOTHIE_TARGET_FORMAT", fmt);
    } else {
        println!("  Format: All");
    }

    println!();
    println!("  Running cargo build...");
    println!();

    let status = Command::new("cargo")
        .args(&args)
        .env("SMOOTHIE_FRAMEWORK", "1")
        .status();

    match status {
        Ok(s) if s.success() => {
            println!();
            println!("  ✓  Build completed successfully.");
        }
        Ok(s) => {
            eprintln!(
                "  ✗  Build failed with exit code: {}",
                s.code().unwrap_or(-1)
            );
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("  ✗  Failed to execute cargo: {}", e);
            std::process::exit(1);
        }
    }
}
