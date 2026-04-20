import math

# 🌀 calculate_phi.py — Strophe 4 Spectral Auditor
# Generates a table of PHI-resonant frequencies.

PHI = (1.0 + 5.0**0.5) / 2.0
INV_PHI = 1.0 / PHI

def generate_phi_table(base_freq=440.0, octaves=4):
    print(f"🚀 Generating PHI-resonant frequency table starting at {base_freq}Hz...")
    
    results = []
    # Find PHI nodes above and below
    for n in range(-octaves, octaves + 1):
        # f = base * PHI^n
        freq = base_freq * (PHI ** n)
        results.append((n, freq))
        
    print(f"📊 Results:")
    for n, freq in sorted(results):
        print(f"   Node {n:2}: {freq:8.2f} Hz")

if __name__ == "__main__":
    generate_phi_table()
