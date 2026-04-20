# 🎨 generate_palette.py — Strophe 9 Aesthetic Auditor
# Generates a PHI-resonant color palette for Seraphic UIs.

PHI = 1.618033988749895

def generate_palette():
    print("🚀 Generating Seraphic PHI-Resonant Palette...")
    
    # Base Colors
    Charcoal = (11, 12, 16)
    Blue = (0, 180, 216)
    Gold = (212, 175, 55)
    
    # Generate PHI variations
    variants = [1.0, 1.0/PHI, 1.0/(PHI**2)]
    
    print("📊 Color Matrix (RGB):")
    for name, color in [("Charcoal", Charcoal), ("Blue", Blue), ("Gold", Gold)]:
        print(f"   {name}:")
        for i, v in enumerate(variants):
            v_color = [int(c * v) for c in color]
            print(f"      Tier {i}: {tuple(v_color)}")

if __name__ == "__main__":
    generate_palette()
