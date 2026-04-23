import re
import glob

def fix():
    paths = glob.glob("crates/*/*/Cargo.toml")
    # Some crates are 1.0.0-LTS and some are 1.0.0. 
    # We will just normalize ALL crates to version = "1.0.0" everywhere to make them compatible with each other and crates.io.
    for path in paths:
        with open(path, 'r') as f:
            content = f.read()

        content = content.replace('"1.0.0-LTS"', '"1.0.0"')
        
        with open(path, 'w') as f:
            f.write(content)

if __name__ == "__main__":
    fix()
