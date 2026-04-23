import re
import glob

def fix():
    for path in glob.glob("crates/*/*/Cargo.toml"):
        with open(path, 'r') as f:
            lines = f.readlines()
        
        seen_keys = set()
        out_lines = []
        for line in lines:
            if line.startswith('['):
                seen_keys = set()
                out_lines.append(line)
                continue
                
            m = re.match(r'^([a-zA-Z0-9_-]+)\s*=', line)
            if m:
                key = m.group(1)
                if key in seen_keys:
                    continue
                seen_keys.add(key)
            out_lines.append(line)
            
        with open(path, 'w') as f:
            f.writelines(out_lines)

if __name__ == "__main__":
    fix()
