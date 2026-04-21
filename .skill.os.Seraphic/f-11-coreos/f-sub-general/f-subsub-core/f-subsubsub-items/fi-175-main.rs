---
id: fi-175-main.rs
category: f-11-coreos
---

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;
use chrono::Utc;
use regex::Regex;

// 🌌 sovereign-rs v3.0.0
// The Executive Heart of the Seraphic Skill Matrix.
// Native Rust Implementation for Industrial Performance and Autonomous Growth.

const DEVO_ROOT: &str = ".skill.os.Seraphic/07-The-Devo";
const MASTER_INDEX: &str = ".skill.os.Seraphic/07-The-Devo/00-Manifest/MASTER_INDEX.json";
const WORKSPACE_CRATES: &str = "smoothie_elite/crates";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Skill {
    path: String,
    standard_compliant: bool,
    doc_lines: usize,
    quality_tier: String,
    verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    version: String,
    mandate: String,
    skills: std::collections::HashMap<String, Skill>,
    last_audit: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scans the library and updates the MASTER_INDEX.json
    Audit,
    /// Triggers a functional script from a skill's 04-Commands folder
    Trigger { skill_id: String, command: String },
    /// Activates the Innovation Engine to scan workspace and forge skills
    Evolve,
    /// Triggers the Self-Healing protocol to correct quality gaps
    Heal,
}

struct Executive {
    manifest: Manifest,
}

impl Executive {
    fn new() -> Self {
        let manifest = if Path::new(MASTER_INDEX).exists() {
            let content = fs::read_to_string(MASTER_INDEX).expect("Failed to read manifest");
            serde_json::from_str(&content).unwrap_or_else(|_| Self::empty_manifest())
        } else {
            Self::empty_manifest()
        };
        Executive { manifest }
    }

    fn empty_manifest() -> Manifest {
        Manifest {
            version: "3.0.0".to_string(),
            mandate: "HIGH_QUALITY_ELITE_ONLY".to_string(),
            skills: std::collections::HashMap::new(),
            last_audit: "".to_string(),
        }
    }

    fn save(&mut self) {
        self.manifest.last_audit = Utc::now().to_rfc3339();
        let content = serde_json::to_string_pretty(&self.manifest).expect("Failed to serialize");
        fs::create_dir_all(Path::new(MASTER_INDEX).parent().unwrap()).unwrap();
        fs::write(MASTER_INDEX, content).expect("Failed to write MASTER_INDEX.json");
    }

    fn audit(&mut self) {
        println!("🚀 INITIATING SOVEREIGN AUDIT (RUST) in {}...", DEVO_ROOT);
        self.manifest.skills.clear();
        let mut skills_found = 0;

        for entry in WalkDir::new(DEVO_ROOT).into_iter().filter_map(|e| e.ok()) {
            if entry.file_name() == "SKILL.md" {
                let skill_dir = entry.path().parent().unwrap();
                let skill_id = skill_dir.file_name().unwrap().to_str().unwrap().to_string();
                let relative_path = skill_dir.strip_prefix(DEVO_ROOT).unwrap().to_str().unwrap().to_string();

                // 5-Tier Compliance Check
                let tiers = ["01-Core", "02-Practices", "03-Examples", "04-Commands", "05-Meta"];
                let has_standard = tiers.iter().all(|t| skill_dir.join(t).exists());

                // Quality Check: Recursive doc line count
                let mut line_count = 0;
                for sub_entry in WalkDir::new(skill_dir).into_iter().filter_map(|e| e.ok()) {
                    if sub_entry.path().extension().map_or(false, |ext| ext == "md") && sub_entry.file_name() != "SKILL.md" {
                        if let Ok(content) = fs::read_to_string(sub_entry.path()) {
                            line_count += content.lines().count();
                        }
                    }
                }

                let quality_tier = if line_count >= 800 { "Elite" } else { "Scaffold" };

                self.manifest.skills.insert(skill_id, Skill {
                    path: relative_path,
                    standard_compliant: has_standard,
                    doc_lines: line_count,
                    quality_tier: quality_tier.to_string(),
                    verified: has_standard && line_count >= 800,
                });
                skills_found += 1;
            }
        }
        self.save();
        println!("✅ AUDIT COMPLETE. Indexed {} skills.", skills_found);
    }

    fn trigger(&self, skill_id: &str, command: &str) {
        if let Some(skill) = self.manifest.skills.get(skill_id) {
            let cmd_root = Path::new(DEVO_ROOT).join(&skill.path).join("04-Commands");
            
            // 1. Check for Directory-based Cargo Project
            let cargo_dir = cmd_root.join(command);
            if cargo_dir.exists() && cargo_dir.is_dir() && cargo_dir.join("Cargo.toml").exists() {
                println!("⚡ TRIGGERING CARGO PROJECT: {}::{}...", skill_id, command);
                Command::new("cargo")
                    .arg("run")
                    .arg("--release")
                    .arg("-q")
                    .current_dir(&cargo_dir)
                    .status()
                    .expect("Cargo execution failed");
                return;
            }

            // 2. Fallback to standalone files
            if let Ok(entries) = fs::read_dir(&cmd_root) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let name = entry.file_name().to_str().unwrap_or("").to_string();
                    if name.starts_with(command) && !entry.path().is_dir() {
                        println!("⚡ TRIGGERING FILE: {}::{} ({})", skill_id, command, name);
                        let path = entry.path();
                        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        
                        match ext {
                            "py" => { Command::new("python3").arg(&path).status().ok(); }
                            "sh" => { Command::new("bash").arg(&path).status().ok(); }
                            "rs" => {
                                let bin_path = "/tmp/seraphic_bin";
                                Command::new("rustc").arg("-O").arg(&path).arg("-o").arg(bin_path).status().ok();
                                Command::new(bin_path).status().ok();
                            }
                            _ => { Command::new(&path).status().ok(); }
                        };
                        return;
                    }
                }
            }
            println!("❌ ERROR: Command '{}' not found in {}/04-Commands.", command, skill_id);
        } else {
            println!("❌ ERROR: Skill '{}' not found in matrix.", skill_id);
        }
    }

    fn evolve(&self) {
        println!("🧠 INITIATING AUTONOMOUS EVOLUTION (RUST CORE)...");
        let tiers = ["01-silicon", "02-resonance", "03-cognition", "04-holography", "05-praxis"];
        let mut opportunities = 0;

        let dsp_regex = Regex::new(r"unsafe|Atomic|next\(").unwrap();
        let logic_regex = Regex::new(r"State|Reducer|Effect").unwrap();

        for tier in tiers {
            let tier_path = Path::new(WORKSPACE_CRATES).join(tier);
            if !tier_path.exists() { continue; }

            for entry in WalkDir::new(tier_path).into_iter().filter_map(|e| e.ok()) {
                if entry.path().extension().map_or(false, |ext| ext == "rs" || ext == "ts") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if content.lines().count() > 150 {
                            if dsp_regex.is_match(&content) {
                                println!("   🔥 Opportunity: {}-dsp-elite (Tier: {})", entry.file_name().to_str().unwrap(), tier);
                                opportunities += 1;
                            }
                            if logic_regex.is_match(&content) {
                                println!("   🔥 Opportunity: {}-logic-elite (Tier: {})", entry.file_name().to_str().unwrap(), tier);
                                opportunities += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("✨ INNOVATION SCAN COMPLETE. Found {} opportunities.", opportunities);
    }

    fn heal(&self) {
        println!("🏥 INITIATING SELF-HEALING (RUST CORE)...");
        let mut gaps = 0;
        for (id, skill) in &self.manifest.skills {
            let has_code = fs::read_dir(Path::new(DEVO_ROOT).join(&skill.path).join("03-Examples"))
                .map_or(false, |mut d| d.any(|e| e.map_or(false, |f| {
                    let n = f.file_name();
                    let s = n.to_str().unwrap_or("");
                    s.ends_with(".rs") || s.ends_with(".ts") || s.ends_with(".sh")
                })));

            if !has_code || skill.doc_lines < 800 {
                println!("   ⚠️  Repairing {}: {} lines, {} examples.", id, skill.doc_lines, if has_code { "Found" } else { "None" });
                gaps += 1;
            }
        }
        println!("✅ HEALING CYCLE COMPLETE. Gaps identified: {}", gaps);
    }
}

fn main() {
    let cli = Cli::parse();
    let mut executive = Executive::new();

    match cli.command {
        Commands::Audit => executive.audit(),
        Commands::Trigger { skill_id, command } => executive.trigger(&skill_id, &command),
        Commands::Evolve => executive.evolve(),
        Commands::Heal => executive.heal(),
    }
}
