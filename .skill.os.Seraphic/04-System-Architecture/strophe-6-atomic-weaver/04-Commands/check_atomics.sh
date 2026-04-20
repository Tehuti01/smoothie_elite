#!/bin/bash
# ⚛️ check_atomics.sh — Strophe 6 Atomic Auditor
# Scans code for dangerous blocking primitives in the audio path.

echo "🚀 Initiating Strophe 6 Atomic Audit..."

# Check for Mutex or RwLock in hot-path crates
grep -r "Mutex::new" crates/dsp crates/synth crates/effects && echo "❌ ERROR: Mutex found in audio crates. Terminal violation."

# Check for Sequential Consistency (may be too slow)
grep -r "Ordering::SeqCst" . --include="*.rs" && echo "⚠️ WARNING: Sequential Consistency (SeqCst) detected. Performance impact possible."

# Check for Atomic usage
grep -r "Atomic" . --include="*.rs" && echo "✓ Atomic primitives detected."

echo "✓ Audit Complete."
