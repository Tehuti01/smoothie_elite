import re
import glob

def cleanup():
    for path in glob.glob("crates/*/*/Cargo.toml") + ["Cargo.toml"]:
        with open(path, 'r') as f:
            content = f.read()

        def dep_fixer(match):
            dep_name = match.group(1)
            inside = match.group(2)
            if 'workspace = true' in inside and 'version =' in inside:
                new_inside = re.sub(r',\s*version\s*=\s*"[^"]*"', '', inside)
                new_inside = re.sub(r'version\s*=\s*"[^"]*"[,\s]*', '', new_inside)
                return dep_name + ' = {' + new_inside.strip().strip(",") + '}'
            return match.group(0)

        content = re.sub(r'^([a-zA-Z0-9_-]+)\s*=\s*\{([^}]*)\}', dep_fixer, content, flags=re.MULTILINE)
        
        with open(path, 'w') as f:
            f.write(content)

if __name__ == "__main__":
    cleanup()
