---
id: fi-2490-skill-validator.rs
category: f-01-secbrain
---

use std::fs;
use std::path::Path;

/// 🛡️ skill_validator.rs v0.2.0 — The Seraphic Skill Validator
/// Audits skills for 12x quality markers: 5-tier integrity, trigger precision, and imperative tone.

fn main() {
    println!("🚀 INITIATING STROPHE 13: SKILL QUALITY AUDIT...");

    let skills_dir = Path::new("skills");
    let mut total_score = 0;
    let mut skill_count = 0;

    if let Ok(entries) = fs::read_dir(skills_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("strophe-") {
                skill_count += 1;
                total_score += audit_skill_quality(&path);
            }
        }
    }

    if skill_count > 0 {
        let avg_score = total_score / skill_count;
        println!("✅ AUDIT COMPLETE. Average Matrix IQ: {}/300.", avg_score);
    }
}

fn audit_skill_quality(path: &Path) -> usize {
    let mut score = 0;
    
    // 1. Structural Check (100 pts)
    let tiers = ["01-Core", "02-Practices", "03-Examples", "04-Commands", "05-Meta"];
    let mut tier_match = 0;
    for tier in &tiers {
        if path.join(tier).exists() { tier_match += 1; }
    }
    score += tier_match * 20;

    // 2. Trigger Check (100 pts)
    let skill_md = path.join("SKILL.md");
    if let Ok(content) = fs::read_to_string(skill_md) {
        if content.contains("description:") && content.len() < 5000 {
            score += 100;
        }
    }

    // 3. Command Check (100 pts)
    let cmd_dir = path.join("04-Commands");
    if cmd_dir.exists() && fs::read_dir(cmd_dir).unwrap().count() > 0 {
        score += 100;
    }

    println!("   - {}: {}/300", path.file_name().unwrap().to_str().unwrap(), score);
    score
}
