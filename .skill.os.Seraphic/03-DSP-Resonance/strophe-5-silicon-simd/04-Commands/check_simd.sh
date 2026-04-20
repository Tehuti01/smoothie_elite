#!/bin/bash
# 🏎️ check_simd.sh — Strophe 5 SIMD Auditor
# Scans compiled binary for SIMD instruction residency (AVX2, AVX-512).

echo "🚀 Initiating Strophe 5 Parallelism Audit..."

# Check for AVX2 instructions (vaddpd, vmulpd, etc.)
objdump -d target/release/libsmoothie_elite.dylib | grep -E "vaddpd|vmulpd|vaddps|vmulps" && echo "✓ AVX2/AVX-512 Instructions detected in binary." || echo "❌ ERROR: No SIMD instructions found. Crate is scalar-limited."

# Check for unaligned memory loads
grep -r "_mm256_loadu_pd" . --include="*.rs" && echo "⚠️ WARNING: Unaligned memory loads (loadu) detected. Performance impact possible."

echo "✓ Audit Complete."
