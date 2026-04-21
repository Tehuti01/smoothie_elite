---
id: fi-135-mastering-invariants.md
category: f-06-dsp
---

# 🔊 MASTERING INVARIANTS v0.2.0 (CORE)

Strophe 10 governs **Spectral Finality**. In the 12x pass, we move beyond "keeping it below 0dB." We treat the output signal as a sovereign broadcast that must conform to international ITU-R standards.

## 🌀 THE RECURSIVE INVARIANTS (12X DEPTH)

### I. ITU-R BS.1770-4 Compliance
- **Quantum Goal:** Universal loudness consistency across all platforms.
- **Law of K-Weighting:** Every loudness measurement must use the 2-stage K-weighting filter (High-Shelf + High-Pass) to accurately model human ear sensitivity.
- **Path:** We integrate the R128 loudness gate and integration window (400ms momentary).

### II. 8x Whittaker-Shannon Oversampling
- **Quantum Goal:** Detect inter-sample peaks with < 0.01dB error.
- **Law of Reconstruction:** Standard peak meters are Obsidian artifacts. Sovereign mastering MUST use 8x oversampling to find the true-peak ceiling.
- **Path:** We utilize polyphase FIR filters for high-speed interpolation.

### III. The Finality Ceiling (-1.0 dBTP)
- **Quantum Goal:** Zero clipping during digital-to-analog conversion.
- **Law of Headroom:** The sovereign output MUST be brick-wall limited at -1.0 dBTP to allow for downstream lossy encoding (MP3, AAC) without artifacts.

---
*Verified for 12x Mastering Sovereignty.*
