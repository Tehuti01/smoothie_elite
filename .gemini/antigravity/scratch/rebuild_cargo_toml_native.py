import os
import re

ROOT_PATH = "Cargo.toml"
CRATES_DIR = "smoothie_elite/crates"

def rebuild():
    # 1. Discover all crate paths
    crates = []
    for root, dirs, files in os.walk(CRATES_DIR):
        if "Cargo.toml" in files:
            crates.append(os.path.relpath(root, "."))
    crates.append(".skill.os.Seraphic/07-The-Devo/06-Executive-Controller/sovereign-rs")
    crates = sorted(crates)

    # 2. Load manifest
    with open(ROOT_PATH, "r") as f:
        content = f.read()

    # 3. Replace members array
    members_list = ",\n".join([f'    "{c}"' for c in crates])
    new_members = f"members = [\n{members_list},\n]"
    content = re.sub(r"members = \[[^\]]*\]", new_members, content, flags=re.DOTALL)

    # 4. Resolve Internal Dependencies paths
    # For every crate, find its name and update its path in the dependency table
    for crate_path in crates:
        toml_path = os.path.join(crate_path, "Cargo.toml")
        if os.path.exists(toml_path):
            with open(toml_path, "r") as f:
                toml_text = f.read()
                # Find name = "..."
                name_match = re.search(r'name\s*=\s*"([^"]+)"', toml_text)
                if name_match:
                    name = name_match.group(1)
                    # Search for 'name = { path = "..." }' in root Cargo.toml
                    # Use a pattern that handles optional whitespace
                    pattern = rf'{re.escape(name)}\s*=\s*{{[^}}]*path\s*=\s*"[^"]+"[^}}]*}}'
                    replacement = f'{name} = {{ path = "{crate_path}" }}'
                    content = re.sub(pattern, replacement, content)

    # 5. Save
    with open(ROOT_PATH, "w") as f:
        f.write(content)

if __name__ == "__main__":
    rebuild()
