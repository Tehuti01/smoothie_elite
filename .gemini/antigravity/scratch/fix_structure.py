import os
import shutil

tiers = {
    '01-silicon': [],
    '02-resonance': [],
    '03-cognition': [],
    '04-holography': []
}

# The goal is to move every folder that contains a Cargo.toml (a crate)
# into one of the 4 tiers, and remove any intermediate "tier-" or nested folders.

base_dir = 'smoothie_elite/crates'

# 1. Scan everything for Cargo.toml
crates_found = []
for root, dirs, files in os.walk(base_dir):
    if 'Cargo.toml' in files:
        crates_found.append(root)

print(f"Found {len(crates_found)} crates.")

# 2. Determine target tier for each crate (simplified mapping)
for crate_path in crates_found:
    crate_name = os.path.basename(crate_path)
    
    # Heuristic mapping based on my plan
    if any(k in crate_path for k in ['silicon', 'core', 'async', 'macros', 'math', 'security', 'sync', 'serde', 'logging']):
        target = '01-silicon'
    elif any(k in crate_path for k in ['resonance', 'dsp', 'dynamics', 'granular', 'reverb', 'eq', 'spectrum', 'physics', 'mastering', 'synth', 'effects', 'sound-design', 'tuning', 'audio-format', 'ironstack']):
        target = '02-resonance'
    elif any(k in crate_path for k in ['cognition', 'ai', 'params', 'midi', 'graph', 'modulation', 'preset', 'agent', 'multiverse', 'prime', 'registry']):
        target = '03-cognition'
    else:
        target = '04-holography'
    
    tiers[target].append(crate_path)

# 3. Create temp directory to move things to avoid collision
temp_dir = 'smoothie_elite/crates_temp'
os.makedirs(temp_dir, exist_ok=True)

for tier, paths in tiers.items():
    tier_temp = os.path.join(temp_dir, tier)
    os.makedirs(tier_temp, exist_ok=True)
    for p in paths:
        dest = os.path.join(tier_temp, os.path.basename(p))
        print(f"Staging {p} -> {dest}")
        if os.path.exists(dest):
             # Collision? Append suffix
             dest += "_dup"
        shutil.move(p, dest)

# 4. Clean up the original crates folder and move temp back
shutil.rmtree(base_dir)
os.rename(temp_dir, base_dir)

print("Restructuring complete.")
