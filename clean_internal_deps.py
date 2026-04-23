import os
import re

def fix_all_internal_deps(path):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    
    # Target anything that looks like smoothie-*, seraphic-*, ironstack-* and has a path
    patterns = [
        r'smoothie-[a-zA-Z0-9-]+\s*=\s*\{[^}]*path\s*=[^}]*\}',
        r'seraphic-[a-zA-Z0-9-]+\s*=\s*\{[^}]*path\s*=[^}]*\}',
        r'ironstack-[a-zA-Z0-9-]+\s*=\s*\{[^}]*path\s*=[^}]*\}'
    ]
    
    new_content = content
    for pattern in patterns:
        def replacer(match):
            line = match.group(0)
            name = line.split('=')[0].strip()
            return f'{name} = {{ workspace = true }}'
        
        new_content = re.sub(pattern, replacer, new_content)
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Cleaned internal deps in {path}")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file == 'Cargo.toml':
            fix_all_internal_deps(os.path.join(root, file))
