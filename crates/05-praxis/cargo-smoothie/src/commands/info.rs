/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xdc4be09c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/info.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use std::fs;
use std::path::Path;

use colored::Colorize;

/// Technical implementation of the execute logic.
pub fn execute() {
    println!("╔══════════════════════════════════════════════════╗");
    println!(
        "║       {}        ║",
        "Smoothie Elite LTS — Project Info".bold().bright_green()
    );
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    // Try to read Cargo.toml
    let cargo_path = Path::new("Cargo.toml");
    if !cargo_path.exists() {
        eprintln!("  ✗  No Cargo.toml found in current directory.");
        eprintln!("     Run this command from a Smoothie Elite plugin project.");
        std::process::exit(1);
    }

    let content = match fs::read_to_string(cargo_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  ✗  Failed to read Cargo.toml: {}", e);
            std::process::exit(1);
        }
    };

    // Parse basic fields
    let name = extract_field(&content, "name").unwrap_or_else(|| "unknown".to_string());
    let version = extract_field(&content, "version").unwrap_or_else(|| "0.0.0".to_string());
    let description = extract_field(&content, "description").unwrap_or_else(|| "—".to_string());
    let license = extract_field(&content, "license").unwrap_or_else(|| "—".to_string());

    println!("  Name:        {}", name);
    println!("  Version:     {}", version);
    println!("  Description: {}", description);
    println!("  License:     {}", license);
    println!();

    // Check for Smoothie dependencies
    let has_smoothie = content.contains("smoothie-core") || content.contains("smoothie_elite");
    if has_smoothie {
        println!("  Framework:   Smoothie Elite ✓");
    } else {
        println!("  Framework:   Not detected (missing smoothie-core dependency)");
    }

    // Check crate type
    let is_cdylib = content.contains("cdylib");
    let is_lib = content.contains("[lib]");
    if is_cdylib {
        println!("  Output:      Dynamic library (plugin)");
    } else if is_lib {
        println!("  Output:      Library");
    } else {
        println!("  Output:      Binary");
    }

    // Count source files
    let src_path = Path::new("src");
    if src_path.exists() {
        let file_count = count_rs_files(src_path);
        println!("  Source files: {}", file_count);
    }

    println!();
    println!("  Supported formats:");
    println!("    • VST3  (Steinberg)");
    println!("    • CLAP  (Free Audio)");
    println!("    • AU    (Apple, macOS/iOS)");
    println!("    • AAX   (Avid, Pro Tools)");
    println!();
}

/// Simple TOML field extraction (no full parser dependency).
fn extract_field(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(key) && trimmed.contains('=') {
            let value = trimmed.split('=').nth(1)?.trim();
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

/// Recursively count .rs files in a directory.
fn count_rs_files(dir: &Path) -> usize {
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                count += count_rs_files(&path);
            } else if path.extension().map_or(false, |e| e == "rs") {
                count += 1;
            }
        }
    }
    count
}
