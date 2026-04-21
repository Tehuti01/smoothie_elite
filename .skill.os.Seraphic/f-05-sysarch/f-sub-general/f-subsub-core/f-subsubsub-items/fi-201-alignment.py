---
id: fi-201-alignment.py
category: f-05-sysarch
---

import os
import re

# ⚡ RS-012: CACHE-LINE ALIGNMENT AUDITOR
# Scans the workspace crates for potential "False Sharing" risks.
# Detects Atomic variables in structs lacking #[repr(align(64))].

WORKSPACE_ROOT = "smoothie_elite/crates"

def audit_alignment():
    print(f"🧐 AUDITING CACHE-LINE SOVEREIGNTY in {WORKSPACE_ROOT}...")
    violations = 0
    
    for root, dirs, files in os.walk(WORKSPACE_ROOT):
        for file in files:
            if file.endswith(".rs"):
                path = os.path.join(root, file)
                with open(path, "r") as f:
                    content = f.read()
                    
                    # Pattern: Struct with Atomic fields
                    structs = re.finditer(r"pub struct (\w+)\s*{([^}]*)}", content, re.MULTILINE | re.DOTALL)
                    for match in structs:
                        struct_name = match.group(1)
                        body = match.group(2)
                        
                        if "Atomic" in body:
                            # Check if the line before or the struct itself has alignment
                            # We search for the struct start in the original content to check attributes
                            struct_start = match.start()
                            prefix = content[max(0, struct_start-100):struct_start]
                            
                            if "#[repr(align(64))]" not in prefix:
                                print(f"⚠️  VIOLATION: Struct '{struct_name}' in {file} contains atomics but is UNALIGNED.")
                                print(f"   Path: {path}")
                                violations += 1

    if violations == 0:
        print("✅ SUCCESS: No alignment violations detected. Cache Sovereignty maintained.")
    else:
        print(f"❌ TOTAL: Found {violations} potential False Sharing risks.")

if __name__ == "__main__":
    audit_alignment()
