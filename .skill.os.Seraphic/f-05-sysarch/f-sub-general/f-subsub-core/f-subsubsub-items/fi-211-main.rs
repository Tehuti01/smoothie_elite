---
id: fi-211-main.rs
category: f-05-sysarch
---

use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// ⚡ RS-014: ARENA MEMORY MONITOR (NATIVE RUST)
// Scans the 'Silicon' tier for illegal heap allocations.
// Enforces the A0 (Zero Allocation) Mandate in real-time paths.

const SILICON_PATH: &str = "../../../../../../../../smoothie_elite/crates/01-silicon";

fn main() {
    println!("🧐 AUDITING ALLOCATION MANDATE (NATIVE RUST) in {}...", SILICON_PATH);
    let mut leaks = 0;
    let forbidden = ["Vec::new", "Vec::with_capacity", "Box::new", "Box::pin", "HashMap::new", "String::new"];

    for entry in WalkDir::new(SILICON_PATH).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (i, line) in content.lines().enumerate() {
                for f_call in forbidden.iter() {
                    if line.contains(f_call) && !line.trim().starts_with("//") {
                        println!("⚠️  VIOLATION: Forbidden heap call '{}' found in {}:{}", 
                                 f_call, entry.path().display(), i + 1);
                        println!("   Line: {}", line.trim());
                        leaks += 1;
                    }
                }
            }
        }
    }

    if leaks == 0 {
        println!("✅ SUCCESS: Zero heap allocations detected in Silicon Tier. Inception logic confirmed.");
    } else {
        println!("❌ TOTAL: Found {} violations of the Allocation Mandate.", leaks);
    }
}
