import re

path = "Cargo.toml"
with open(path, "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    # Fix member paths
    line = line.replace("smoothie_elite/crates/04-holography/smoothie-net", "smoothie_elite/crates/05-praxis/smoothie-net")
    line = line.replace("smoothie_elite/crates/04-holography/smoothie-au", "smoothie_elite/crates/05-praxis/smoothie-au")
    line = line.replace("smoothie_elite/crates/04-holography/smoothie-clap", "smoothie_elite/crates/05-praxis/smoothie-clap")
    
    # Fix dependency paths
    if "smoothie-net" in line and "04-holography" in line:
        line = line.replace("04-holography", "05-praxis")
    if "smoothie-clap" in line and "04-holography" in line:
        line = line.replace("04-holography", "05-praxis")
    if "smoothie-aax" in line and "smoothie-au" not in line:
         # Insert smoothie-au before aax if missing
         new_lines.append('    smoothie-au   = { path = "smoothie_elite/crates/05-praxis/smoothie-au" }\n')
    
    new_lines.append(line)

with open(path, "w") as f:
    f.writelines(new_lines)
