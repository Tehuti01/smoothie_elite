---
id: fi-120-lock-heap.sh
category: f-06-dsp
---

#!/bin/bash
# 🌌 lock_heap.sh — Strophe 3 Memory Auditor
# Scans compiled binary for libc symbols related to heap allocation.

echo "🚀 Initiating Strophe 3 Allocation Audit..."

# Check for malloc/free/realloc symbols
nm target/release/libsmoothie_elite.dylib | grep -E "malloc|free|realloc" && echo "❌ ERROR: Dynamic allocation symbols found in binary!"

# Check for standard library allocations in Rust
grep -r "Vec::new" . --include="*.rs" | grep "fn process" && echo "⚠️ WARNING: Possible Vec allocation in hot path detected."

echo "✓ Audit Complete."
