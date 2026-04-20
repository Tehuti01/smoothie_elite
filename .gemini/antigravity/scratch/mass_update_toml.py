import os
import re

tiers = ['01-silicon', '02-resonance', '03-cognition', '04-holography']
base_dir = 'smoothie_elite/crates'

workspace_deps = [
    'smoothie-core', 'smoothie-dsp', 'smoothie-params', 'smoothie-midi', 
    'smoothie-ui', 'smoothie-ui-render', 'smoothie-vst3', 'smoothie-clap', 
    'smoothie-au', 'smoothie-aax', 'smoothie-presets', 'smoothie-graph', 
    'smoothie-math', 'smoothie-modulation', 'smoothie-sync', 'smoothie-ai', 
    'smoothie-macros', 'smoothie-dynamics', 'smoothie-granular', 'smoothie-reverb',
    'smoothie-eq', 'smoothie-spectrum', 'smoothie-physics', 'smoothie-security',
    'smoothie-net', 'smoothie-registry', 'smoothie-ai-core', 'smoothie-ui-core',
    'smoothie-ui-vfx', 'smoothie-standalone', 'smoothie-async', 'smoothie-logging',
    'smoothie-serde'
]

def update_toml(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
    
    # 1. Update version to use workspace
    # Also update edition, authors, license if they exist as fixed strings and root has them
    content = re.sub(r'^version\s*=\s*"[^"]*"', 'version.workspace = true', content, flags=re.MULTILINE)
    content = re.sub(r'^edition\s*=\s*"[^"]*"', 'edition.workspace = true', content, flags=re.MULTILINE)
    
    # 2. Update internal dependencies to workspace = true
    for dep in workspace_deps:
        # Match both { path = "..." } and { version = "...", path = "..." }
        # Match variations like smoothie-core = { path = "../core" }
        pattern = rf'^{dep}\s*=\s*\{{[^}}]*path\s*=\s*"[^"]*"[^}}]*\}}'
        replacement = f'{dep} = {{ workspace = true }}'
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)

    with open(file_path, 'w') as f:
        f.write(content)

for tier in tiers:
    tier_path = os.path.join(base_dir, tier)
    if not os.path.exists(tier_path):
        continue
    for crate_dir in os.listdir(tier_path):
        crate_path = os.path.join(tier_path, crate_dir)
        if not os.path.isdir(crate_path):
            continue
        toml_path = os.path.join(crate_path, 'Cargo.toml')
        if os.path.exists(toml_path):
            print(f"Updating {toml_path}")
            update_toml(toml_path)
