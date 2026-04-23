import os
import re
import glob

def run():
    # Find all Cargo.toml files in crates
    crate_tomls = glob.glob("crates/*/*/Cargo.toml")
    
    for toml_path in crate_tomls:
        with open(toml_path, 'r') as f:
            content = f.read()

        # Remove "publish = false"
        content = re.sub(r'^publish\s*=\s*false\n?', '', content, flags=re.MULTILINE)

        # Add license and description if missing
        if 'license' not in content:
            # Inject under version or edition
            content = re.sub(
                r'(^edition\s*=\s*"2021")', 
                r'\1\nlicense = "MIT"\ndescription = "A Seraphic Technologies Elite audio plugin framework crate."\nrepository = "https://github.com/seraphic/smoothie-elite"', 
                content, 
                flags=re.MULTILINE
            )
            
        with open(toml_path, 'w') as f:
            f.write(content)
            
    print(f"Updated {len(crate_tomls)} crates for publishing.")

if __name__ == "__main__":
    run()
