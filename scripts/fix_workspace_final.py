import re
import glob

def fix_workspace_deps():
    with open('Cargo.toml', 'r') as f:
        content = f.read()

    # Find [workspace.dependencies] block and add version = "1.0.0" to any { path = "..." } that lacks it
    def replacer(match):
        dep_name = match.group(1)
        inside = match.group(2)
        if 'version' not in inside:
            return f'{dep_name} = {{{inside}, version = "1.0.0"}}'
        return match.group(0)

    # We only want to apply this in [workspace.dependencies], but applying globally is fine for the root
    content = re.sub(r'^([a-zA-Z0-9_-]+)\s*=\s*\{([^}]*path\s*=[^}]*)\}', replacer, content, flags=re.MULTILINE)
    
    with open('Cargo.toml', 'w') as f:
        f.write(content)

def fix_crates():
    for path in glob.glob("crates/*/*/Cargo.toml"):
        with open(path, 'r') as f:
            content = f.read()

        # Fix version fields in crates that are something else
        content = re.sub(r'^version\s*=\s*"[^"]*"', 'version = "1.0.0"', content, flags=re.MULTILINE)
        
        # Remove unused manifest keys for workspace = true that also specify version = "1.0.0" incorrectly 
        # from our previous bad script
        content = re.sub(r'workspace\s*=\s*true,\s*version\s*=\s*"[^"]*"', 'workspace = true', content)
        
        with open(path, 'w') as f:
            f.write(content)

if __name__ == "__main__":
    fix_workspace_deps()
    fix_crates()