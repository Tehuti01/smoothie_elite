/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3e847e6e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/cargo-smoothie/src/commands/new.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use include_dir::{include_dir, Dir};
///
/// Generates a complete Smoothie Elite plugin project using embedded industrial templates.
use std::fs;
use std::path::{Path, PathBuf};

/// Embedded template directory
static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Available plugin templates.
const VALID_TEMPLATES: &[&str] = &["effect", "instrument", "analyzer", "utility"];

/// Technical implementation of the execute logic.
pub fn execute(name: &str, template: &str, path: Option<&str>) {
    if !VALID_TEMPLATES.contains(&template) {
        eprintln!(
            "❌ ERROR: Unknown template '{}'. Valid: {}",
            template,
            VALID_TEMPLATES.join(", ")
        );
        std::process::exit(1);
    }

    let project_dir = match path {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from(format!("./{}", name)),
    };

    println!("╔══════════════════════════════════════════════════╗");
    println!(
        "║          {}              ║",
        "Smoothie Elite — Autonomous Scaffold".bold()
    );
    println!("╚══════════════════════════════════════════════════╝");
    println!();
    println!("  🚀 Initiating inception of: {}", name.bright_green());
    println!("  🎨 Using template:          {}", template.bright_cyan());
    println!();

    // 1. Extract embedded template
    let template_dir = TEMPLATES_DIR
        .get_dir(template)
        .expect("Template directory missing in binary");
    extract_dir(template_dir, &project_dir, name);

    println!("  {} Project scaffolded successfully.", "✓".green());
    println!("\n  Next steps:");
    println!("    cd {}", project_dir.display());
    println!("    cargo smoothie build --release");
    println!();
}

/// Technical implementation of the extract_dir logic.
fn extract_dir(dir: &Dir, target: &Path, project_name: &str) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(d) => {
                let new_target = target.join(d.path().file_name().unwrap());
                fs::create_dir_all(&new_target).unwrap();
                extract_dir(d, &new_target, project_name);
            }
            include_dir::DirEntry::File(f) => {
                let file_path = target.join(f.path().file_name().unwrap());
                let content = f.contents_utf8().unwrap();

                // Perform template variable replacement
                let processed = content
                    .replace("{{project_name}}", project_name)
                    .replace("{{project_name_pascal}}", &to_pascal_case(project_name));

                // If it was a .template file, remove the extension
                let final_path = if file_path.extension().map_or(false, |ext| ext == "template") {
                    file_path.with_extension("")
                } else {
                    file_path
                };

                // Ensure parent directory exists
                if let Some(parent) = final_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }

                fs::write(final_path, processed).expect("Failed to write scaffold file");
            }
        }
    }
}

/// Technical implementation of the to_pascal_case logic.
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '-' || c == '_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = first.to_uppercase().to_string();
                    result.extend(chars);
                    result
                }
            }
        })
        .collect()
}

use colored::Colorize;
