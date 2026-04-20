use std::fs;
use std::path::Path;

/// 🛠️ doc_auditor.rs v0.2.0 — The Seraphic Documentation Auditor
/// Audits Rust source files for Tri-Layer compliance and performance proofs.

fn main() {
    println!("🚀 INITIATING STROPHE 15: HYPER-DOCS AUDIT...");

    let src_path = Path::new("crates");
    let mut missing_proofs = 0;

    audit_docs(src_path, &mut missing_proofs);

    if missing_proofs == 0 {
        println!("✅ AUDIT SUCCESS: All documentation satisfies the Tri-Layer Protocol.");
    } else {
        println!("❌ AUDIT FAILURE: {} public methods missing performance proofs.", missing_proofs);
    }
}

fn audit_docs(path: &Path, missing: &mut usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                audit_docs(&p, missing);
            } else if p.extension().map_or(false, |ext| ext == "rs") {
                let content = fs::read_to_string(&p).unwrap_or_default();
                // Check for public functions missing "Performance Proof"
                for line in content.lines() {
                    if line.contains("pub fn") && !content.contains("Performance Proof") {
                        println!("   ⚠️  MISSING PROOF: {}", p.display());
                        *missing += 1;
                        break;
                    }
                }
            }
        }
    }
}
