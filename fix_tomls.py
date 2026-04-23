import os
import re

def fix_toml(path):
    with open(path, 'r') as f:
        content = f.read()
    
    # Try even broader match
    new_content = re.sub(r'smoothie-[a-zA-Z0-9-]+\s*=\s*\{[^}]*path\s*=[^}]*\}', lambda m: m.group(0).split('=')[0].strip() + ' = { workspace = true }', content)
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Fixed {path}")
    else:
        # Debug
        if 'smoothie-' in content and 'path =' in content:
             print(f"Found smoothie with path in {path} but no match")

for root, dirs, files in os.walk('.'):
    if 'target' in root: continue
    for file in files:
        if file == 'Cargo.toml' and root != '.':
            fix_toml(os.path.join(root, file))
