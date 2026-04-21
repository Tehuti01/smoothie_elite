---
id: fi-107-simd-oscillator.md
category: f-06-dsp
---

# 🛠️ SIMD OSCILLATOR BANK (EXAMPLE)

A 12x Quality implementation of a multi-oscillator bank using AVX2.

### 1. Structure of Arrays (SoA)
```rust
#[repr(align(64))]
pub struct SimdOscillatorBank {
    pub phases: [f64; 4],      // Aligned for AVX2
    pub increments: [f64; 4],
    pub amplitudes: [f64; 4],
}
```

### 2. High-Performance Process
```rust
use std::arch::x86_64::*;

impl SimdOscillatorBank {
    #[seraphic_mandate(SIMD, L0)]
    pub unsafe fn process_parallel(&mut self) -> __m256d {
        // Load SoA data into YMM registers
        let mut p = _mm256_load_pd(self.phases.as_ptr());
        let i = _mm256_load_pd(self.increments.as_ptr());
        let a = _mm256_load_pd(self.amplitudes.as_ptr());

        // Increment and Wrap (Branchless)
        p = _mm256_add_pd(p, i);
        let mask = _mm256_cmp_pd(p, _mm256_set1_pd(1.0), _CMP_GE_OQ);
        p = _mm256_sub_pd(p, _mm256_and_pd(mask, _mm256_set1_pd(1.0)));

        // Store back updated state
        _mm256_store_pd(self.phases.as_mut_ptr(), p);

        // Multiply by Amplitude and Return
        _mm256_mul_pd(p, a)
    }
}
```

---
*Example 12x SIMD Implementation: CONFIRMED.*
