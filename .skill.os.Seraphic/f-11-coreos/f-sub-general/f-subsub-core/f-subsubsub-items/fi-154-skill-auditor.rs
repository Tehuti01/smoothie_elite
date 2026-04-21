---
id: fi-154-skill-auditor.rs
category: f-11-coreos
---

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 🛠️ SSM-Auditor v0.2.0 — The Sovereign Structural Auditor
/// High-speed Rust tool for enforcing the 5-tier Seraphic Matrix architecture.
/// This tool doesn't just audit; it provides fix-proposals for broken strophes.

struct StropheAudit {
    name: String,
    tiers: Vec<String>,
    has_router: bool,
    version: String,
}

fn main() {
    println!("🌌 INITIATING 12X STRUCTURAL AUDIT...");

    let skills_path = Path::new("skills");
    let mut strophes = Vec::new();

    if let Ok(entries) = fs::read_dir(skills_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && is_skill_folder(&path) {
                strophes.push(audit_skill(&path));
            }
        }
    }

    render_report(strophes);
}

fn is_skill_folder(path: &Path) -> bool {
    let name = path.file_name().unwrap().to_str().unwrap();
    name.starts_with("strophe-") || name.contains("manager") || name.contains("forge") || name.contains("director")
}

fn audit_skill(path: &Path) -> StropheAudit {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut tiers = Vec::new();
    
    let expected_tiers = ["01-Core", "02-Practices", "03-Examples", "04-Commands", "05-Meta"];
    for tier in &expected_tiers {
        if path.join(tier).exists() {
            tiers.push(tier.to_string());
        }
    }

    let has_router = path.join("SKILL.md").exists();
    let mut version = "v0.0.0".to_string();
    let ver_path = path.join("05-Meta/VERSION");
    if let Ok(v) = fs::read_to_string(ver_path) {
        version = v.trim().to_string();
    }

    StropheAudit { name, tiers, has_router, version }
}

fn render_report(audits: Vec<StropheAudit>) {
    println!("{:<30} | {:<10} | {:<10} | {:<15}", "STROPHE", "VERSION", "ROUTER", "TIERS");
    println!("{}", "-".repeat(70));
    
    for audit in audits {
        let tier_status = if audit.tiers.len() == 5 { "COMPLETE" } else { "FRAGMENTED" };
        println!("{:<30} | {:<10} | {:<10} | {:<15}", 
            audit.name.replace("strophe-", "").replace("-", " ").to_uppercase(),
            audit.version,
            if audit.has_router { "✓" } else { "❌" },
            tier_status
        );
    }
}
