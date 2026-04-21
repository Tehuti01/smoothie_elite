---
id: fi-134-loudness-laws.md
category: f-06-dsp
---

# 📜 LOUDNESS LAWS v0.2.0 (PRACTICES)

To maintain Mastering sovereignty, the following laws must be strictly enforced:

### 1. Integrated LUFS Target
All plugins must have a "Sovereign Target" mode for Integrated Loudness.
- **Requirement:** Default target = -14.0 LUFS (Standard for Spotify/Apple Music).
- **Goal:** Phase-coherent dynamic range management.

### 2. Mandatory True-Peak Limiter
A true-peak limiter is the final node in every sovereign DSP graph.
- **Law:** Disable standard hard-clippers in production builds. Use only Whittaker-Shannon limited outputs.

### 3. Crest-Factor Audit
Analyze the difference between Peak and LUFS (Crest Factor).
- **Guideline:** Aim for a Crest Factor of 8-12dB for "Seraphic Timbre" (Dynamic yet powerful).

### 4. DC-Offset Elimination
All mastering chains must include a high-pass filter at 10Hz to eliminate DC-offset and preserve headroom.

---
*Loudness Pipeline Protocol: ENFORCED.*
