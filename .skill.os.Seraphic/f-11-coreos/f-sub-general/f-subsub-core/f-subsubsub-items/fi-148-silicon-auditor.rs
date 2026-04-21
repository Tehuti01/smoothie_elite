---
id: fi-148-silicon-auditor.rs
category: f-11-coreos
---

use std::fs;
use std::path::Path;
use std::process::exit;

/// 🛡️ silicon_auditor.rs v0.2.0 — The Seraphic Silicon Auditor
/// Analyzes Rust source code for memory alignment and allocation violations.
/// Industrial Grade: Scans for struct alignment and prohibited standard library types.

fn main() {
    println!("🚀 INITIATING STROPHE 1: SILICON ALIGNMENT AUDIT...");

    let args: Vec<String> = std::env::args().collect();
    let src_path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new("crates")
    };
    if !src_path.exists() {
        println!("❌ ERROR: 'crates' directory not found.");
        exit(1);
    }

    let mut issues = 0;
    audit_directory(src_path, &mut issues);

    if issues == 0 {
        println!("✅ AUDIT SUCCESS: All crates align with the Seraphic Mandate.");
    } else {
        println!("❌ AUDIT FAILURE: {} violations detected.", issues);
        exit(1);
    }
}

fn audit_directory(path: &Path, issues: &mut usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                audit_directory(&p, issues);
            } else if p.extension().map_or(false, |ext| ext == "rs") {
                audit_file(&p, issues);
            }
        }
    }
}

fn audit_file(path: &Path, issues: &mut usize) {
    let content = fs::read_to_string(path).unwrap_or_default();
    
    // Remove comments for auditing
    let mut clean_content = String::new();
    let mut in_comment = false;
    let mut in_block_comment = false;
    
    let lines: Vec<&str> = content.lines().collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("///") || trimmed.starts_with("*") {
            continue;
        }
        clean_content.push_str(line);
        clean_content.push('\n');
    }

    // 1. Check for Struct Alignment (64-byte)
    if clean_content.contains("struct ") {
        if !clean_content.contains("#[repr(align(64))]") && !clean_content.contains("#[repr(C)]") {
            println!("   ⚠️  UNALIGNED STRUCT: {} (Missing align(64))", path.display());
            *issues += 1;
        }
    }

    // 2. Check for Heap Allocations in process loops
    if let Some(start) = content.find("fn process") {
        let block_end = content[start..].find('}').unwrap_or(content.len() - start) + start;
        let hot_path = &content[start..block_end];
        
        let prohibited = ["Vec::new", "Box::new", "HashMap::new", "String::from", "vec!"];
        for term in &prohibited {
            if hot_path.contains(term) {
                println!("   ❌ ALLOCATION IN HOT PATH: {} (Found {})", path.display(), term);
                *issues += 1;
            }
        }
    }
}
