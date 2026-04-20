import os
import shutil
import re

base_path = '/Users/tehuti01/SeFi-Sam/smoothie_elite/crates'
flat_path = '/Users/tehuti01/SeFi-Sam/smoothie_elite/crates_flat'

if os.path.exists(flat_path):
    shutil.rmtree(flat_path)
os.makedirs(flat_path)

def find_crates(path):
    crates = []
    for root, dirs, files in os.walk(path):
        if 'Cargo.toml' in files:
            crates.append(root)
    return crates

all_crates = find_crates(base_path)
print(f"Found {len(all_crates)} crates.")

for crate in all_crates:
    # Read Cargo.toml to get the actual package name
    toml_path = os.path.join(crate, 'Cargo.toml')
    try:
        with open(toml_path, 'r') as f:
            content = f.read()
            match = re.search(r'name\s*=\s*"([^"]+)"', content)
            if match:
                pkg_name = match.group(1)
            else:
                pkg_name = os.path.basename(crate)
    except:
        pkg_name = os.path.basename(crate)
    
    dest = os.path.join(flat_path, pkg_name)
    if os.path.exists(dest):
        # Handle duplicates by using the folder structure name
        dest = os.path.join(flat_path, os.path.basename(crate))
        if os.path.exists(dest):
             dest = dest + "_ext"
             
    print(f"Moving {crate} -> {dest}")
    shutil.move(crate, dest)

print("Flattening complete. Cleaning up original directory...")
shutil.rmtree(base_path)
os.rename(flat_path, base_path)

# Now, sort into tiers
tier_map = {
    '01-silicon': ['smoothie-core', 'smoothie-async', 'smoothie-macros', 'smoothie-math', 'smoothie-security', 'smoothie-sync', 'smoothie-serde', 'smoothie-logging', 'core', 'async', 'sync', 'serde', 'logging', 'ironstack'],
    '02-resonance': ['smoothie-dsp', 'smoothie-dynamics', 'smoothie-granular', 'smoothie-reverb', 'smoothie-eq', 'smoothie-spectrum', 'smoothie-physics', 'smoothie-modulation', 'smoothie-midi', 'dsp', 'effects', 'synth', 'smoothie-sound-design', 'smoothie-tuning', 'smoothie-audio-format'],
    '03-cognition': ['smoothie-ai', 'smoothie-params', 'smoothie-graph', 'smoothie-preset', 'smoothie-ai-core', 'seraphic-agent', 'seraphic-multiverse', 'seraphic-prime', 'smoothie-registry'],
    '04-holography': ['smoothie-au', 'smoothie-aax', 'smoothie-clap', 'smoothie-vst3', 'smoothie-standalone', 'smoothie-frontend', 'smoothie-ui', 'smoothie-ui-render', 'smoothie-ui-vfx', 'smoothie-ui-core', 'smoothie-cli-backend', 'smoothie-cli-frontend', 'cargo-smoothie', 'cli', 'smoothie-wasm']
}

for tier in tier_map:
    os.makedirs(os.path.join(base_path, tier), exist_ok=True)

for crate_dir in os.listdir(base_path):
    crate_full_path = os.path.join(base_path, crate_dir)
    if not os.path.isdir(crate_full_path) or crate_dir.startswith('0'):
        continue
    
    target_tier = '04-holography' # Default
    for tier, members in tier_map.items():
        if crate_dir in members:
            target_tier = tier
            break
    
    dest_path = os.path.join(base_path, target_tier, crate_dir)
    print(f"Sorting {crate_dir} -> {target_tier}")
    shutil.move(crate_full_path, dest_path)

print("Ascension Complete.")
