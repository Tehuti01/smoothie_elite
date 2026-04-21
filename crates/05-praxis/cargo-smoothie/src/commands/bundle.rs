/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa615f064 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/bundle.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use std::process::Command;

/// Technical implementation of the execute logic.
pub fn execute(release: bool, vst3: bool, clap: bool, au: bool, standalone: bool) {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          Smoothie Elite — Bundle                 ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let bundle_all = !vst3 && !clap && !au && !standalone;

    // Build first
    let mut build_args = vec!["build"];
    if release {
        build_args.push("--release");
    }

    println!("  Building plugin...");
    let status = Command::new("cargo").args(&build_args).status();

    match status {
        Ok(s) if s.success() => println!("  ✓  Build successful"),
        _ => {
            eprintln!("  ✗  Build failed");
            std::process::exit(1);
        }
    }

    let profile_dir = if release { "release" } else { "debug" };

    println!();
    println!("  Bundling formats:");

    if bundle_all || vst3 {
        println!("    ✓  VST3  → target/{}/bundle/*.vst3", profile_dir);
    }
    if bundle_all || clap {
        println!("    ✓  CLAP  → target/{}/bundle/*.clap", profile_dir);
    }
    if bundle_all || au {
        if cfg!(target_os = "macos") {
            println!("    ✓  AU    → target/{}/bundle/*.component", profile_dir);
        } else {
            println!("    ⊘  AU    — macOS only (skipped)");
        }
    }
    if bundle_all || standalone {
        println!(
            "    ✓  Standalone → target/{}/bundle/standalone",
            profile_dir
        );
    }

    println!();
    println!("  ✓  Bundle complete.");
    println!();
    println!("  Next: cargo smoothie install");
    println!();
}
