---
id: fi-167-repo-auditor.py
category: f-11-coreos
---

import os
import subprocess

# 🛠️ repo_auditor.py v0.2.0 — Strophe 19 GitHub Auditor
# Audits the local repository for structural finality and security laws.

def run_git_audit():
    print("🚀 INITIATING STROPHE 19: REPO SOVEREIGNTY AUDIT...")
    
    # 1. Essential File Integrity
    essentials = [
        "README.md", "LICENSE", ".gitignore", "SECURITY.md", 
        "CONTRIBUTING.md", "CODE_OF_CONDUCT.md"
    ]
    
    violations = 0
    for f in essentials:
        if os.path.exists(f):
            print(f"   ✓ {f:<20} | DETECTED")
        else:
            print(f"   ❌ {f:<20} | MISSING")
            violations += 1

    # 2. CI/CD Stratum Check
    if os.path.isdir(".github/workflows"):
        print(f"   ✓ GitHub Workflows   | ACTIVE")
    else:
        print(f"   ❌ GitHub Workflows   | INACTIVE")
        violations += 1

    # 3. Branch Check (Local)
    branch = subprocess.getoutput("git rev-parse --abbrev-ref HEAD")
    print(f"   - Current Branch: {branch}")
    if branch == "main" or branch == "master":
        print("   ⚠️  WARNING: Directly operating on Mainline. Violates Law 10.")

    if violations == 0:
        print("\n✅ AUDIT SUCCESS: Repository is Sovereign-compliant.")
    else:
        print(f"\n❌ AUDIT FAILURE: {violations} structural violations detected.")

if __name__ == "__main__":
    run_git_audit()
