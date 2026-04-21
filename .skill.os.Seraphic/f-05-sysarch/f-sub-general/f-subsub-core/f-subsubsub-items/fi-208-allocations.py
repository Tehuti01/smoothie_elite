---
id: fi-208-allocations.py
category: f-05-sysarch
---

import os
import re

# ⚡ RS-014: ARENA MEMORY MONITOR
# Scans the 'Silicon' tier for illegal heap allocations.
# Enforces the A0 (Zero Allocation) Mandate in real-time paths.

SILICON_PATH = "smoothie_elite/crates/01-silicon"

def audit_allocations():
    print(f"🧐 AUDITING ALLOCATION MANDATE in {SILICON_PATH}...")
    leaks = 0
    forbidden = ["Vec::new", "Vec::with_capacity", "Box::new", "Box::pin", "HashMap::new", "String::new"]
    
    for root, dirs, files in os.walk(SILICON_PATH):
        for file in files:
            if file.endswith(".rs"):
                path = os.path.join(root, file)
                with open(path, "r") as f:
                    lines = f.readlines()
                    for i, line in enumerate(lines):
                        for f_call in forbidden:
                            if f_call in line and "//" not in line:
                                print(f"⚠️  VIOLATION: Forbidden heap call '{f_call}' found in {file}:{i+1}")
                                print(f"   Line: {line.strip()}")
                                leaks += 1

    if leaks == 0:
        print("✅ SUCCESS: Zero heap allocations detected in Silicon Tier. Inception logic confirmed.")
    else:
        print(f"❌ TOTAL: Found {leaks} violations of the Allocation Mandate.")

if __name__ == "__main__":
    audit_allocations()
