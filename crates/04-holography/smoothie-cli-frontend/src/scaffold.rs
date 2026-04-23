/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x27a247ef | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-cli-frontend/src/scaffold.rs         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: UI layer for project scaffolding and template extraction.   │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: High-visual feedback for project inception.             │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::{print_seraphic_header, print_step, print_success};
use colored::Colorize;

/// Technical implementation of the show_scaffold_start logic.
pub fn show_scaffold_start(project_name: &str, template: &str) {
    print_seraphic_header("Autonomous Scaffold Engine");

    print_step(&format!(
        "Project Identity: {}",
        project_name.bright_green()
    ));
    print_step(&format!("Strophe Template: {}", template.bright_cyan()));
    println!();

    println!(
        "  {} Initiating structural resonance...",
        "🚀".bright_yellow()
    );
}

/// Technical implementation of the show_scaffold_complete logic.
pub fn show_scaffold_complete(project_name: &str, path: &str) {
    println!();
    print_success(&format!(
        "Project '{}' successfully manifested at: {}",
        project_name, path
    ));
    println!();
    println!("  {} Next steps:", "🔗".bright_blue());
    println!("    cd {}", path.bright_white());
    println!("    cargo smoothie build --release");
    println!();
}
