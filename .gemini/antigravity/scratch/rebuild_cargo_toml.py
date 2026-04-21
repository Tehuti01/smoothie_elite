import os
import toml

ROOT_PATH = "Cargo.toml"
CRATES_DIR = "smoothie_elite/crates"

def rebuild():
    # 1. Discover all crates
    crates = []
    for root, dirs, files in os.walk(CRATES_DIR):
        if "Cargo.toml" in files:
            crates.append(os.path.relpath(root, "."))
    
    # Add the sovereign-rs tool
    crates.append(".skill.os.Seraphic/07-The-Devo/06-Executive-Controller/sovereign-rs")

    # 2. Load current manifest
    with open(ROOT_PATH, "r") as f:
        manifest = toml.load(f)

    # 3. Rebuild members
    manifest["workspace"]["members"] = sorted(crates)

    # 4. Rebuild internal dependencies
    # We'll scan each crate's Cargo.toml to get its package name
    internal_deps = {}
    for crate_path in crates:
        crate_toml = os.path.join(crate_path, "Cargo.toml")
        if os.path.exists(crate_toml):
            try:
                c_data = toml.load(crate_toml)
                name = c_data["package"]["name"]
                internal_deps[name] = {"path": crate_path}
            except Exception as e:
                print(f"Skipping {crate_path}: {e}")

    # Update dependencies (preserve foreign ones)
    new_deps = {}
    for name, data in manifest["workspace"]["dependencies"].items():
        if isinstance(data, dict) and "path" in data:
            if name in internal_deps:
                new_deps[name] = internal_deps[name]
        else:
            new_deps[name] = data
            
    # Add any missing internal ones
    for name, data in internal_deps.items():
        if name not in new_deps:
            new_deps[name] = data

    manifest["workspace"]["dependencies"] = new_deps

    # 5. Save with consistent formatting
    with open(ROOT_PATH, "w") as f:
        toml.dump(manifest, f)

if __name__ == "__main__":
    rebuild()
