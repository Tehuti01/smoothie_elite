---
id: fi-133-true-peak-limiter.md
category: f-06-dsp
---

# 🛠️ 8x TRUE PEAK LIMITER (EXAMPLE)

A 12x Quality implementation of a polyphase true-peak limiter.

### 1. FIR Interpolation Kernel
```rust
#[repr(align(64))]
pub struct PolyphaseLimiter {
    // 8 sets of 8 coefficients (8x oversampling)
    fir_kernels: [[f64; 8]; 8], 
    history: [f64; 8],
    ceiling: f64,
}
```

### 2. High-Fidelity Detection
```rust
impl PolyphaseLimiter {
    #[seraphic_mandate(LIMITER, L0)]
    pub fn process_sample(&mut self, input: f64) -> f64 {
        self.history.rotate_right(1);
        self.history[0] = input;
        
        // Find maximum peak across 8 sub-sample positions
        let mut max_p = input.abs();
        for kernel in &self.fir_kernels {
            let sub_sample = kernel.iter().zip(&self.history)
                                   .map(|(a, b)| a * b).sum::<f64>();
            max_p = max_p.max(sub_sample.abs());
        }

        // Apply gain reduction if over ceiling
        let reduction = (self.ceiling / max_p).min(1.0);
        input * reduction
    }
}
```

---
*Example 12x Mastering Implementation: CONFIRMED.*
