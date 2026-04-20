#!/bin/bash
# ⚛️ atomic_auditor.sh v0.2.0 — Strophe 6 Thread-Safety Auditor
# Scans Rust code for dangerous atomic patterns and Sequential Consistency.

echo "🚀 INITIATING STROPHE 6: ATOMIC INTEGRITY AUDIT..."

# 1. Scan for SeqCst (Sequential Consistency)
grep -r "Ordering::SeqCst" . --include="*.rs" | grep -v "test" && echo "❌ ERROR: SeqCst detected in production code. Memory barrier violation."

# 2. Scan for missing padding in atomic structs
grep -r "struct" . --include="*.rs" -A 5 | grep "Atomic" | grep -v "align(64)" && echo "⚠️  WARNING: Possible False Sharing detected. Atomic counters missing alignment."

# 3. Scan for Mutex/Lock in Audio paths
grep -r "Mutex" smoothie_elite/crates/smoothie-dsp smoothie_elite/crates/smoothie-synth && echo "❌ ERROR: Mutex found in realtime crates. Deadlock risk."

echo "✓ Audit Complete."
