import os
import shutil

base = '/Users/tehuti01/SeFi-Sam/smoothie_elite/crates'

mapping = {
    'tier-0-silicon': '01-silicon',
    'tier-1-resonance': '02-resonance',
    'tier-2-cognition': '03-cognition',
    'tier-3-holography': '04-holography',
    'tier-4-praxis': '04-holography'
}

for src_name, dst_name in mapping.items():
    src = os.path.join(base, src_name)
    dst = os.path.join(base, dst_name)
    
    if os.path.exists(src):
        if not os.path.exists(dst):
            os.makedirs(dst)
        
        for item in os.listdir(src):
            s = os.path.join(src, item)
            d = os.path.join(dst, item)
            if os.path.exists(d):
                # If it's a directory, merge it? Or just rename?
                if os.path.isdir(s):
                    # For simplicity, move sub-contents or rename with suffix
                    d_new = d + "_dup"
                    print(f"Collision: {s} -> {d_new}")
                    shutil.move(s, d_new)
                else:
                    os.remove(d) # Overwrite files
                    shutil.move(s, d)
            else:
                shutil.move(s, d)
        
        shutil.rmtree(src)
        print(f"Merged {src_name} into {dst_name}")

print("Consolidation complete.")
