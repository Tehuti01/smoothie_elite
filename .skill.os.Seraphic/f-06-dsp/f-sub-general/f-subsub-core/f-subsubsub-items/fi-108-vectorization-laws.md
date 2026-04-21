---
id: fi-108-vectorization-laws.md
category: f-06-dsp
---

# 📜 VECTORIZATION LAWS v0.2.0 (PRACTICES)

To maintain Parallel sovereignty, the following laws must be strictly enforced:

### 1. Explicit Vector Intrinsic Mandate
Do not rely on the compiler's auto-vectorizer for critical DSP loops.
- **Requirement:** Use explicit `std::arch` intrinsics or high-level SIMD libraries (e.g., `packed_simd`, `faster`).
- **Goal:** Deterministic instruction selection.

### 2. Software Prefetching
For large datasets (Wavetables, Impulse Responses), use software prefetch instructions.
- **Law:** Warm the cache lines 2-3 iterations ahead of the current vector load.

### 3. Loop Unrolling (The Fibonacci Factor)
Unroll loops by PHI-derived factors (e.g., 2, 3, 5, 8) to reduce the overhead of branch counters.
- **Guideline:** Balance unrolling with instruction cache pressure.

### 4. Denormal Flush Protocol
All SIMD-driven crates must initialize the CPU state with `FTZ` (Flush-to-Zero) and `DAZ` (Denormals-Are-Zero).
- **Constraint:** Zero CPU stalls for near-zero signal values.

---
*Vectorization Pipeline Protocol: ENFORCED.*
