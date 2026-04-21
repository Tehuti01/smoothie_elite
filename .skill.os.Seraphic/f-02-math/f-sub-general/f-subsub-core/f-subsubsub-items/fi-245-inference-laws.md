---
id: fi-245-inference-laws.md
category: f-02-math
---

# 📜 INFERENCE LAWS v0.2.0 (PRACTICES)

To maintain Neural sovereignty, the following laws must be strictly enforced:

### 1. Weight Quantization Mandate
All weights must be quantized to `f32` or `f16` (if hardware supports it) for the inference path.
- **Requirement:** Keep `f64` only for the final accumulation before the audio output.
- **Goal:** 2x-4x throughput increase via SIMD lane packing.

### 2. The LSTM Unrolling Rule
Recurrent kernels (LSTM, GRU) must be manually unrolled by the PHI-constant factor.
- **Law:** Process 5 or 8 steps per inner loop to minimize the state-swapping overhead.

### 3. MatMul Vectorization
Directly utilize the SIMD intrinsics from Strophe 5 for all matrix multiplications.
- **Constraint:** Use `_mm256_fmadd_ps` (Fused Multiply-Add) to perform dot products in a single clock cycle.

### 4. Zero-Initialization Protocol
Hidden states must be zero-initialized during the **Era of Inception** and never re-allocated.

---
*Neural Inference Protocol: ENFORCED.*
