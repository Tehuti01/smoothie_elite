import os
import glob

math_dir = 'crates/02-resonance/smoothie-sound-design/src/enterprise_math'
files = glob.glob(os.path.join(math_dir, '*.rs'))

removed_count = 0
for f in files:
    with open(f, 'r') as file:
        lines = file.readlines()
    
    new_lines = []
    for line in lines:
        if "High-precision stability bit-audit node" not in line:
            new_lines.append(line)
        else:
            removed_count += 1
            
    with open(f, 'w') as file:
        file.writelines(new_lines)

print(f"Removed {removed_count} lines of bit-audit filler across {len(files)} files.")
