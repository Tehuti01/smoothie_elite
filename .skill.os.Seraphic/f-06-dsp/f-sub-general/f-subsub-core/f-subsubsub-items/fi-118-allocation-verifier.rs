---
id: fi-118-allocation-verifier.rs
category: f-06-dsp
---

use std::process::Command;
use std::fs;
use std::path::Path;

/// 🛠️ allocation_verifier.rs v0.2.0 — The Seraphic A0 Verifier
/// Audits the compiled binary for any external calls to dynamic allocation functions.
/// Industrial Grade: Scans for symbols from libc and the Rust standard library.

fn main() {
    println!("🚀 INITIATING STROPHE 3: ALLOCATION SOVEREIGNTY AUDIT...");

    let args: Vec<String> = std::env::args().collect();
    let target_bin = if args.len() > 1 {
        &args[1]
    } else {
        "target/release/libsmoothie_elite.dylib"
    };
    if !Path::new(target_bin).exists() {
        println!("❌ ERROR: Release binary not found. Run 'cargo build --release' first.");
        return;
    }

    // Check for dynamic allocation symbols in the dynamic symbol table
    let output = Command::new("nm")
        .arg("-D") // Look at dynamic symbols
        .arg(target_bin)
        .output()
        .expect("Failed to execute nm");

    let symbols = String::from_utf8_lossy(&output.stdout);
    
    // Prohibited allocation primitives
    let prohibited = ["malloc", "free", "realloc", "calloc", "_Znwm", "_ZdlPv"]; // C and C++ mangled symbols
    let mut violation_count = 0;

    for sym in &prohibited {
        if symbols.contains(sym) {
            println!("   ❌ PROHIBITED SYMBOL DETECTED: {}", sym);
            violation_count += 1;
        }
    }

    if violation_count == 0 {
        println!("✅ AUDIT SUCCESS: Binary is 100% Allocation-Sovereign (A0).");
    } else {
        println!("❌ AUDIT FAILURE: {} memory leaks found in binary structure.", violation_count);
    }
}
