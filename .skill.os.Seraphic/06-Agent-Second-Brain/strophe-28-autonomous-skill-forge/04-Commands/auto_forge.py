import os
import sys
import json
from datetime import datetime

# 🚀 auto_forge.py v0.1.0 — Sovereign Skill Inception Engine
# Automatically scaffolds a 5-tier skill folder within .skill.os.Seraphic.

def forge_skill(name, description, category, taxonomy_level):
    print(f"🚀 {datetime.now().strftime('%H:%M:%S')} | INITIATING SKILL FORGE...")
    print(f"   - Target: {name} ({taxonomy_level})")
    print(f"   - Category: {category}")

    matrix_root = ".skill.os.Seraphic"
    skill_path = os.path.join(matrix_root, category, name)

    # 1. Create Structure
    tiers = ["01-Core", "02-Practices", "03-Examples", "04-Commands", "05-Meta"]
    for tier in tiers:
        os.makedirs(os.path.join(skill_path, tier), exist_ok=True)
    
    # 2. Write SKILL.md (Router)
    with open(os.path.join(skill_path, "SKILL.md"), "w") as f:
        f.write(f"---\nname: {name}\ndescription: \"{description}\"\n---\n\n")
        f.write(f"# 🌌 {name.upper().replace('-', ' ')} (ROUTER)\n\n")
        f.write("Welcome to the Sovereign Knowledge Silo. Follow the sub-folder path:\n\n")
        for i, tier in enumerate(tiers):
            f.write(f"{i+1}. [{tier}/] - Mandatory Layer.\n")

    # 3. Write VERSION
    with open(os.path.join(skill_path, "05-Meta", "VERSION"), "w") as f:
        f.write("v0.1.0\n")

    # 4. Generate Code Trigger (Python Template)
    with open(os.path.join(skill_path, "04-Commands", f"trigger_{name.replace('-', '_')}.py"), "w") as f:
        f.write(f"import sys\n\ndef check_relevance():\n    print(\"🚀 Checking relevance for {name}...\")\n    # Logic to ask agent if this is needed\n    print(\"✅ RELEVANT: Skill engaged.\")\n\nif __name__ == '__main__':\n    check_relevance()")

    # 5. Update Manifest
    manifest_path = os.path.join(matrix_root, "manifest.json")
    if os.path.exists(manifest_path):
        with open(manifest_path, "r") as f:
            manifest = json.load(f)
        
        found = False
        for stratum in manifest["stratums"]:
            if stratum["name"] == category:
                if name not in stratum["skills"]:
                    stratum["skills"].append(name)
                found = True
                break
        
        if not found:
            manifest["stratums"].append({"name": category, "skills": [name]})

        with open(manifest_path, "w") as f:
            json.dump(manifest, f, indent=2)

    print(f"\n✅ FORGE COMPLETE. Skill '{name}' is now active in {category}.")

if __name__ == "__main__":
    if len(sys.argv) < 5:
        print("❌ ERROR: Usage: auto_forge.py <NAME> <DESC> <CATEGORY> <LEVEL>")
    else:
        forge_skill(sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4])
