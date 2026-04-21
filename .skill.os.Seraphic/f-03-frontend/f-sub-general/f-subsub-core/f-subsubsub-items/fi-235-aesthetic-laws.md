---
id: fi-235-aesthetic-laws.md
category: f-03-frontend
---

# 📜 AESTHETIC LAWS v0.2.0 (PRACTICES)

To maintain UI sovereignty, the following laws must be strictly enforced:

### 1. The SDF-Only Mandate
Pre-rendered bitmaps (PNG, JPG) are prohibited for core UI components.
- **Requirement:** Generate all knobs, sliders, and buttons using WGSL/GLSL Signed Distance Functions.
- **Goal:** Infinite scalability and zero texture-memory pressure.

### 2. Glassmorphism Refraction
Layered components must utilize backdrop blurring with a PHI-derived blur radius.
- **Law:** `BlurRadius = BaseSize / PHI^n`.

### 3. Sub-Pixel Precision
All coordinates must be handled in floating-point space before the final fragment shader step.
- **Constraint:** Zero "pixel-snapping" artifacts during high-speed animation.

### 4. Color-Resonance Tokens
Use only the Seraphic Palette (Abyssal Charcoal, Seraphic Blue, Valeon Gold) with PHI-resonant opacity steps.

---
*Aesthetic Pipeline Protocol: ENFORCED.*
