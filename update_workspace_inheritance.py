import os
import re

def update_to_workspace_fields(path):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    
    # Replace hardcoded values with workspace inheritance
    # We target version, edition, license, authors, repository, rust-version
    
    fields = ['version', 'edition', 'license', 'authors', 'repository', 'homepage', 'rust-version', 'description']
    
    new_content = content
    for field in fields:
        # Match field = "..." or field = ["..."]
        # Use a regex that replaces the value with { workspace = true }
        pattern = r'^' + field + r'\s*=\s*.*$'
        if re.search(pattern, new_content, re.MULTILINE):
            new_content = re.sub(pattern, f'{field}.workspace = true', new_content, flags=re.MULTILINE)
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Updated {path} to workspace fields")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file == 'Cargo.toml':
            update_to_workspace_fields(os.path.join(root, file))
