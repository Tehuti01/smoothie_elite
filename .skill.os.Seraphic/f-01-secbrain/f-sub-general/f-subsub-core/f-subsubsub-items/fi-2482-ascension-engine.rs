---
id: fi-2482-ascension-engine.rs
category: f-01-secbrain
---

use std::fs;
use std::path::{Path, PathBuf};

/// 🛠️ ascension_engine.rs v0.1.0 — The Matrix Singularity Tool
/// Audits the entire .skill.os.Seraphic matrix for content density.
/// Identifies "Shallow Skills" ( < 300 lines) and triggers an expansion turn.

fn main() {
    println!("🚀 INITIATING RECURSIVE ASCENSION ENGINE...");

    let matrix_root = Path::new(".skill.os.Seraphic");
    let mut targets = Vec::new();

    if let Ok(entries) = fs::read_dir(matrix_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                audit_category(&path, &mut targets);
            }
        }
    }

    if targets.is_empty() {
        println!("✅ SUCCESS: The Matrix has reached temporary saturation.");
    } else {
        println!("\n⚠️  DETECTION: {} skills require 12x depth expansion:", targets.len());
        for t in targets {
            println!("   ↳ {}", t.display());
        }
        println!("\n💡 TRIGGER: Use Strophe 13 to expand these silos to 800+ lines.");
    }
}

fn audit_category(path: &Path, targets: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() && p.file_name().unwrap().to_str().unwrap().starts_with("strophe-") {
                if !is_deep_enough(&p) {
                    targets.push(p);
                }
            }
        }
    }
}

fn is_deep_enough(path: &Path) -> bool {
    let mut total_lines = 0;
    
    // Sum lines in SKILL.md and all 01-05 sub-folders
    if let Ok(entries) = WalkDir::new(path) {
        for entry in entries {
            let p = entry.path();
            if p.is_file() && (p.extension().map_or(false, |e| e == "md" || e == "rs")) {
                if let Ok(content) = fs::read_to_string(p) {
                    total_lines += content.lines().count();
                }
            }
        }
    }

    total_lines >= 800
}

// Minimal WalkDir implementation for single-file tool portability
struct WalkDir {
    stack: Vec<PathBuf>,
}

impl WalkDir {
    fn new(path: &Path) -> Result<Self, std::io::Error> {
        Ok(WalkDir { stack: vec![path.to_path_buf()] })
    }
}

impl Iterator for WalkDir {
    type Item = std::fs::DirEntry;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.stack.pop() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        self.stack.push(p);
                    } else {
                        return Some(entry);
                    }
                }
            }
        }
        None
    }
}
