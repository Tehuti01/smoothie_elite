---
id: fi-2492-package-skill.sh
category: f-01-secbrain
---

#!/bin/bash
# 🛠️ package_skill.sh — Strophe 13 Skill Auditor
# Validates and packages a Seraphic skill into a .skill file.

echo "🚀 Initiating Strophe 13 Skill Audit..."

# Check for 5-tier architecture
for dir in 01-Core 02-Practices 03-Examples 04-Commands 05-Meta; do
    if [ ! -d "$dir" ]; then
        echo "❌ ERROR: Missing sub-folder $dir. Tier violation."
        exit 1
    fi
done

# Check for SKILL.md router guide
if [ ! -f "SKILL.md" ]; then
    echo "❌ ERROR: Missing SKILL.md. Router violation."
    exit 1
fi

echo "✓ Skill Structure Verified. Ready for packaging."
