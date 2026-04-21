/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9e8d7c6b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/seraphic-orchestrator/src/injector.rs                                                     │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Global documentation and branding injection engine.         │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use anyhow::Result;
use regex::Regex;
use std::fs;
use walkdir::WalkDir;
use colored::*;

/// Technical implementation of the Injector structure.
pub struct Injector {
    doc_patterns: Vec<(Regex, String)>,
}

impl Injector {
    pub fn new() -> Result<Self> {
        let doc_patterns = vec![
             (Regex::new(r"(?m)^pub struct (\w+)")?, "/// Technical implementation of the $1 structure.\npub struct $1".to_string()),
             (Regex::new(r"(?m)^pub enum (\w+)")?, "/// Technical implementation of the $1 enumeration.\npub enum $1".to_string()),
             (Regex::new(r"(?m)^pub fn (\w+)")?, "/// Technical implementation of the $1 logic.\npub fn $1".to_string()),
        ];
        
        Ok(Self { doc_patterns })
    }

    pub async fn run(&mut self, _force: bool) -> Result<()> {
        println!("💉 Injecting technical documentation and branding headers...");
        
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(path)?;
                let mut new_content = content.clone();

                // 1. Technical Documentation Injection
                for (regex, replacement) in &self.doc_patterns {
                    new_content = regex.replace_all(&new_content, replacement).to_string();
                }

                // 2. Formatting Cleanup (Prevent duplicate docs)
                let cleanup_regex = Regex::new(r"(?m)^///.*\n///")?;
                new_content = cleanup_regex.replace_all(&new_content, "///").to_string();

                if new_content != content {
                    fs::write(path, new_content)?;
                    println!("   {} {}", "INJECTED:".bright_blue(), path.display());
                }
            }
        }

        println!("{}", "✅ INJECTION SEQUENCE COMPLETE.".bright_blue().bold());
        Ok(())
    }
}
