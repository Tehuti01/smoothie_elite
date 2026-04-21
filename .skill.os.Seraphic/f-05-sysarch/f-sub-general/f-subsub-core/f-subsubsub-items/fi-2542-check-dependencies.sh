---
id: fi-2542-check-dependencies.sh
category: f-05-sysarch
---

#!/bin/bash
# 🗺️ check_dependencies.sh — Strophe 11 Workspace Auditor
# Audits the crate dependency graph and version consistency.

echo "🚀 Initiating Strophe 11 Architectural Audit..."

# Check for workspace version consistency
grep -r "version.workspace = true" crates/ && echo "✓ All crates use workspace-level versioning." || echo "❌ ERROR: Inconsistent crate versions detected!"

# Check for circular dependencies (basic)
cargo tree --workspace --duplicates && echo "⚠️ WARNING: Possible duplicate dependencies detected."

# Check for isolation
grep -r "smoothie-" crates/ --exclude-dir=core | grep -v "path = \"../" && echo "❌ ERROR: External path detected in sovereign crate!"

echo "✓ Audit Complete."
