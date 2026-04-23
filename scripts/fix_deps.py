import re
import glob
import json

# Fix dependencies to include versions
def fix():
    paths = glob.glob("crates/*/*/Cargo.toml")
    for path in paths:
        with open(path, 'r') as f:
            content = f.read()

        # Find internal dependencies that only have `path = ` or `workspace = ` and add a dummy version requirement
        # E.g. smoothie-macros = { path = "../smoothie-macros" } -> smoothie-macros = { path = "../smoothie-macros", version = "1.0.0" }
        
        def replacer(match):
            dep_name = match.group(1)
            inside = match.group(2)
            if 'version' not in inside:
                return f'{dep_name} = {{{inside}, version = "1.0.0"}}'
            return match.group(0)

        content = re.sub(r'^([a-zA-Z0-9_-]+)\s*=\s*\{([^}]*)\}', replacer, content, flags=re.MULTILINE)
        
        with open(path, 'w') as f:
            f.write(content)

if __name__ == "__main__":
    fix()
