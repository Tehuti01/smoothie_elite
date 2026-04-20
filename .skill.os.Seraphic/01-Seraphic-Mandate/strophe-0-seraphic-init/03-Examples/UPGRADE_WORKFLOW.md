# 🛠️ UPGRADE WORKFLOW (EXAMPLE)

A step-by-step example of how the agent identifies and implements a skill upgrade.

### 1. The Trigger
The agent is coding a new reverb algorithm and notices that `strophe-4-resonance-phi` doesn't cover "Multi-Tap PHI Spacing."

### 2. Reflection Turn
"Reflection: Current PHI skill is insufficient for multi-tap delay. Initiating Upgrade."

### 3. Action
- The agent updates `02-Practices/MODULATION_LAWS.md` in `strophe-4`.
- The agent adds a new Tap-Spacing example in `03-Examples`.
- The agent bumps the version in `05-Meta/VERSION`.

### 4. Ouroboros Seal
Run `skill_auditor` to ensure the matrix remains structure-sound.

---
*Example Reactive Upgrade: CONFIRMED.*
