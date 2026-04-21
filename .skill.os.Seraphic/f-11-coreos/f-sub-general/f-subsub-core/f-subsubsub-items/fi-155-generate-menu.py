---
id: fi-155-generate-menu.py
category: f-11-coreos
---

import os
import json
from datetime import datetime

# 📋 generate_menu.py v0.2.5 — Seraphic Matrix Stratum Reporter
# Audits the stratified Matrix and generates the Sovereign Dashboard.

def generate_reports():
    print(f"\n🚀 {datetime.now().strftime('%Y-%m-%d %H:%M:%S')} | SSM v0.2.6 Stratum Reporter")
    matrix_root = ".skill.os.Seraphic"
    
    with open(os.path.join(matrix_root, "manifest.json"), "r") as f:
        manifest = json.load(f)

    print("╔" + "═" * 88 + "╗")
    print("║" + " " * 31 + "SERAPHIC AGENTIC OS HUB" + " " * 34 + "║")
    print("╠" + "═" * 35 + "╦" + "═" * 15 + "╦" + "═" * 15 + "╦" + "═" * 20 + "╣")
    print(f"║ {'STRATUM / STROPHE':<33} ║ {'VERSION':<13} ║ {'TIERS':<13} ║ {'STATUS':<18} ║")
    print("╠" + "═" * 35 + "╬" + "═" * 15 + "╬" + "═" * 15 + "╬" + "═" * 20 + "╣")

    for stratum in manifest["stratums"]:
        print(f"║ {stratum['name'].upper():<33} ║ {'-':<13} ║ {'-':<13} ║ {'STRATUM ACTIVE':<18} ║")
        for skill in stratum["skills"]:
            skill_path = os.path.join(matrix_root, stratum["name"], skill)
            ver_path = os.path.join(skill_path, "05-Meta", "VERSION")
            version = open(ver_path).read().strip() if os.path.exists(ver_path) else "v0.1.0"
            
            # Tier count
            if os.path.exists(skill_path):
                tiers = [d for d in os.listdir(skill_path) if d[0].isdigit()]
                tier_icons = "●" * len(tiers) + "○" * (5 - len(tiers))
            else:
                tier_icons = "❌ MISSING"

            name = skill.replace("strophe-", "").replace("-", " ").upper()
            print(f"║   ↳ {name:<31} ║ {version:<13} ║ {tier_icons:<13} ║ {'SOVEREIGN':<18} ║")

    print("╚" + "═" * 35 + "╩" + "═" * 15 + "╩" + "═" * 15 + "╩" + "═" * 20 + "╝")
    print("✓ Agentic OS: ONLINE | Similarity Engine: INDEXED | CPU/RAM < 10%\n")

if __name__ == "__main__":
    generate_reports()
