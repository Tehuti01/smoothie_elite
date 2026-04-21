/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbf41e26b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/test.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use std::process::Command;

/// Technical implementation of the execute logic.
pub fn execute() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║          Smoothie Elite — Test Suite             ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();

    let steps: Vec<(&str, Vec<&str>)> = vec![
        ("Format check", vec!["fmt", "--all", "--", "--check"]),
        (
            "Clippy lint",
            vec!["clippy", "--workspace", "--", "-D", "warnings"],
        ),
        ("Unit tests", vec!["test", "--workspace"]),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (name, args) in &steps {
        print!("  Running {}...", name);
        let status = Command::new("cargo").args(args).output();

        match status {
            Ok(output) if output.status.success() => {
                println!(" ✓");
                passed += 1;
            }
            Ok(_) => {
                println!(" ✗");
                failed += 1;
            }
            Err(e) => {
                println!(" ✗  ({})", e);
                failed += 1;
            }
        }
    }

    println!();
    println!("  ─────────────────────────────────────────");
    println!("  Results: {} passed, {} failed", passed, failed);
    println!();

    if failed > 0 {
        std::process::exit(1);
    }
}
