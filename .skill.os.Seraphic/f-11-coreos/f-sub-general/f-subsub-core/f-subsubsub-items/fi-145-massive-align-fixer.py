---
id: fi-145-massive-align-fixer.py
category: f-11-coreos
---

import os

# 🚀 massive_align_fixer.py v0.2.1 — Strophe 1 Alignment Tool
# Automatically injects #[repr(align(64))] into Rust structs.

def fix_alignment(directory):
    print(f"🚀 INITIATING MASSIVE ALIGNMENT FIX IN {directory}...")
    
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):
                path = os.path.join(root, file)
                with open(path, "r") as f:
                    lines = f.readlines()

                new_lines = []
                changed = False
                for i, line in enumerate(lines):
                    # Find struct definitions
                    if ("struct " in line) and not line.strip().startswith("//") and not line.strip().startswith("*"):
                        # Check if previous line has repr
                        has_repr = False
                        if i > 0 and "#[repr(" in lines[i-1]:
                            has_repr = True
                        
                        if not has_repr:
                            new_lines.append("#[repr(align(64))]\n")
                            changed = True
                    
                    new_lines.append(line)
                
                if changed:
                    print(f"   ✓ Fixed alignment in {path}")
                    with open(path, "w") as f:
                        f.writelines(new_lines)

if __name__ == "__main__":
    fix_alignment("smoothie_elite/crates")
