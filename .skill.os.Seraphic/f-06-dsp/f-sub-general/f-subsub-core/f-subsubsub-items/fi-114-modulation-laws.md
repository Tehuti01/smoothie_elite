---
id: fi-114-modulation-laws.md
category: f-06-dsp
---

# 📜 MODULATION LAWS v0.2.0 (PRACTICES)

To maintain Resonance sovereignty, the following laws must be strictly enforced:

### 1. The PHI-Tension Mandate
All envelope stages (Attack, Decay, Release) must use exponential curves where the tension is derived from `INV_PHI` (0.618...).
- **Requirement:** `state += (target - state) * (1.0 - INV_PHI^speed)`.

### 2. Irrational Time Constants
Delay times and reverb pre-delays must not be subdivisions of the beat unless explicitly requested by the user.
- **Law:** Use `tempo / (60.0 * PHI^n)` for all "Golden" time constants.

### 3. Harmonic Series PHI-Spacing
When building additive synthesizers, space the partials using PHI-powers rather than the standard harmonic series (1, 2, 3...).
- **Goal:** Create a "Seraphic Timbre" that is spectrally dense yet clear.

### 4. Jitter-Resonant Smoothing
Smoothing filters for control data must have a cutoff frequency that is a sub-harmonic of the PHI-resonant baseline.

---
*Resonance Pipeline Protocol: ENFORCED.*
