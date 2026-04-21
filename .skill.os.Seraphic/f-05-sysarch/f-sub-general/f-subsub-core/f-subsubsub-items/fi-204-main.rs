---
id: fi-204-main.rs
category: f-05-sysarch
---

use std::fs;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;

// ⚡ RS-012: CACHE-LINE ALIGNMENT AUDITOR (NATIVE RUST)
// Scans the workspace crates for potential "False Sharing" risks.
// Implementation of the Seraphic Architectural mandate in Rust.

const WORKSPACE_ROOT: &str = "../../../../../../../../smoothie_elite/crates";

fn main() {
    println!("🧐 AUDITING CACHE-LINE SOVEREIGNTY (NATIVE RUST) in {}...", WORKSPACE_ROOT);
    let mut violations = 0;

    let struct_regex = Regex::new(r"(?s)pub struct (\w+)\s*\{([^}]*)\}").unwrap();

    for entry in WalkDir::new(WORKSPACE_ROOT).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            
            for cap in struct_regex.captures_iter(&content) {
                let struct_name = &cap[1];
                let body = &cap[2];

                if body.contains("Atomic") {
                    // Check for alignment attribute in the preceding lines
                    let struct_start = cap.get(0).unwrap().start();
                    let prefix = &content[struct_start.saturating_sub(100)..struct_start];
                    
                    if !prefix.contains("#[repr(align(64))]") {
                        println!("⚠️  VIOLATION: Struct '{}' in {} contains atomics but is UNALIGNED.", 
                                 struct_name, entry.path().display());
                        violations += 1;
                    }
                }
            }
        }
    }

    if violations == 0 {
        println!("✅ SUCCESS: No alignment violations detected. Cache Sovereignty maintained.");
    } else {
        println!("❌ TOTAL: Found {} potential False Sharing risks.", violations);
    }
}
