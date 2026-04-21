---
id: fi-2540-path-sovereignty-fixer.py
category: f-05-sysarch
---

import os
import re

# 🚀 path_sovereignty_fixer.py v0.2.1
# Mapping folder names to tier categories

TIERS = {
    "tier-0-silicon": ["core", "smoothie-math", "sync", "smoothie-core", "async", "logging", "serde", "ironstack", "smoothie-security", "smoothie-macros"],
    "tier-1-resonance": ["dsp", "smoothie-eq", "smoothie-dynamics", "smoothie-modulation", "smoothie-physics", "effects", "smoothie-reverb", "synth", "smoothie-audio-format", "smoothie-granular", "smoothie-sound-design", "smoothie-spectrum", "smoothie-tuning", "smoothie-fx"],
    "tier-2-cognition": ["smoothie-ai", "smoothie-ai-core", "seraphic-agent", "smoothie-params", "smoothie-graph", "seraphic-prime", "seraphic-multiverse", "smoothie-registry", "smoothie-preset", "smoothie-midi"],
    "tier-3-holography": ["smoothie-ui", "smoothie-ui-core", "smoothie-ui-render", "smoothie-ui-vfx", "smoothie-wasm", "smoothie-frontend", "smoothie-cli-frontend", "smoothie-cli-backend"],
    "tier-4-praxis": ["smoothie-vst3", "smoothie-au", "smoothie-clap", "smoothie-net", "smoothie-standalone", "smoothie-mastering", "cargo-smoothie", "smoothie-aax"]
}

# CRATE NAME TO FOLDER NAME mapping
NAME_TO_FOLDER = {
    "smoothie-core": "smoothie-core", # Wait, is it core or smoothie-core?
    "smoothie-sync": "sync",
    "smoothie-async": "async",
    "smoothie-logging": "logging",
    "smoothie-serde": "serde",
    "smoothie-security": "smoothie-security",
}

# Add more mappings if needed
CRATE_TO_TIER = {}
for tier, crates in TIERS.items():
    for c in crates:
        CRATE_TO_TIER[c] = tier

def fix_paths(base_dir):
    print(f"🚀 INITIATING PATH RESYNC IN {base_dir}...")
    
    for tier, crates in TIERS.items():
        for folder in crates:
            manifest_path = os.path.join(base_dir, tier, folder, "Cargo.toml")
            if not os.path.exists(manifest_path):
                continue
                
            with open(manifest_path, "r") as f:
                content = f.read()

            # Match: dep_name = { path = "../other_folder" }
            pattern = r'(\w+[-\w]*)\s*=\s*\{\s*path\s*=\s*"(\.\./[^"]+)"'
            
            def replace_path(match):
                dep_name = match.group(1)
                old_path = match.group(2)
                
                # Extract target folder name
                target_folder = old_path.replace("../", "")
                
                # We need to find which tier this folder belongs to
                target_tier = None
                for t, fs in TIERS.items():
                    if target_folder in fs:
                        target_tier = t
                        break
                
                if target_tier:
                    if target_tier == tier:
                        return f'{dep_name} = {{ path = "../{target_folder}"'
                    else:
                        return f'{dep_name} = {{ path = "../../{target_tier}/{target_folder}"'
                
                return match.group(0)

            new_content = re.sub(pattern, replace_path, content)
            if new_content != content:
                with open(manifest_path, "w") as f:
                    f.write(new_content)

if __name__ == "__main__":
    fix_paths("smoothie_elite/crates")
