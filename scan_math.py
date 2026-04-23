import os
import glob
import re

math_dir = 'crates/02-resonance/smoothie-sound-design/src/enterprise_math'
files = glob.glob(os.path.join(math_dir, '*.rs'))
print(f"Found {len(files)} math files.")

for f in files:
    with open(f, 'r') as file:
        content = file.read()
        if "High-precision stability bit-audit node" in content:
            print(f"{os.path.basename(f)} contains bit-audit filler.")
        if "let omega = 2.0 * PI_F64 * self.params.frequency;" in content:
            print(f"{os.path.basename(f)} uses generic oscillator derivative.")
