# 🏛️ SKILL 007-ELITE: DSP FUNDAMENTALS - MASTER EDITION

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                    🏛️ SKILL 007-ELITE: DSP FUNDAMENTALS 🏛️
                     Digital Signal Processing Core
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## 📋 30-STEP IMPLEMENTATION ROADMAP

### PHASE 1: FOUNDATION (Steps 1-5)

---

#### 🦦 STEP 1: DSP THEORY

**Research Commands:**
```bash
websearch "DSP fundamentals audio 2025"
websearch "discrete signal processing theory"
websearch "Z-transform audio"
```

**Source Links:**
- [x] Julius Smith: https://ccrma.stanford.edu/~jos/

---

#### 🦦 STEP 2: SAMPLING

**Implementation:**
```rust
// Sample rate conversion
pub fn resample_linear(input: &[f32], ratio: f32) -> Vec<f32> {
    // Linear interpolation
}
```

---

#### 🦦 STEP 3: FILTERS

**Implementation:**
```rust
// Biquad filter implementation
pub struct Biquad {
    pub b: [f32; 3],
    pub a: [f32; 3],
    pub x: [f32; 2],
    pub y: [f32; 2],
}
```

---

#### 🦦 STEP 4: FFT

**Research Commands:**
```bash
websearch "Rust FFT implementation audio"
websearch "FFT optimized audio"
```

---

#### 🦦 STEP 5: CONVOLUTION

**Implementation:**
```rust
// Convolution reverb
pub fn convolve(input: &[f32], ir: &[f32]) -> Vec<f32> {
    // Fast convolution
}
```

---

### PHASE 2: STEPS 6-15

- Oscillators
- Envelopes
- Modulation
- Synthesis
- Effects
- Analysis
- Quantization
- Dithering
- Noise Shaping
- Optimization

---

*Skill ID: 007-ELITE | Category: DSP | Complexity: Foundation*