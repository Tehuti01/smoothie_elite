use std::fs;
use std::path::Path;
use toml::Value;

/// 🗺️ workspace_auditor.rs v0.2.0 — The Seraphic Crate Orchestrator
/// High-speed tool to audit the workspace dependency graph and tier-consistency.
/// Verifies that all crates correctly inherit from the root workspace.

fn main() {
    println!("🚀 INITIATING STROPHE 11: WORKSPACE ARCHITECTURAL AUDIT...");

    let root_cargo = "Cargo.toml";
    if !Path::new(root_cargo).exists() {
        println!("❌ ERROR: Root Cargo.toml not found.");
        return;
    }

    let mut issues = 0;
    audit_workspace_inheritance(&mut issues);

    if issues == 0 {
        println!("✅ AUDIT SUCCESS: Workspace topology is Sovereign-compliant.");
    } else {
        println!("❌ AUDIT FAILURE: {} architectural violations detected.", issues);
    }
}

fn audit_workspace_inheritance(issues: &mut usize) {
    let crates_dir = Path::new("crates");
    if let Ok(entries) = fs::read_dir(crates_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("Cargo.toml").exists() {
                let toml_path = path.join("Cargo.toml");
                let content = fs::read_to_string(&toml_path).unwrap_or_default();
                
                // 1. Check for workspace version inheritance
                if !content.contains("version.workspace = true") {
                    println!("   ❌ INHERITANCE ERROR: {} (Missing version.workspace)", path.display());
                    *issues += 1;
                }

                // 2. Check for tier violations (Simple check)
                if path.file_name().unwrap().to_str().unwrap() == "core" {
                    if content.contains("smoothie-ui") || content.contains("smoothie-vst3") {
                        println!("   ❌ TIER VIOLATION: Core tier cannot depend on higher-tier UI components.");
                        *issues += 1;
                    }
                }
            }
        }
    }
}
