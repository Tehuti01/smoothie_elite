---
id: fi-127-zero-latency-delay.md
category: f-06-dsp
---

# 🛠️ ZERO LATENCY DELAY (EXAMPLE)

A sample-accurate delay implementation that satisfies the L0 invariant.

### 1. The Pre-Allocated Ring Buffer
```rust
#[repr(align(64))]
pub struct L0Delay {
    buffer: [f64; 1024],
    index: usize,
}
```

### 2. Sample-Accurate Process
```rust
impl L0Delay {
    #[seraphic_mandate(L0)]
    pub fn process(&mut self, input: f64) -> f64 {
        // [Strophe 2]: Write to current index, read from previous
        let output = self.buffer[self.index];
        self.buffer[self.index] = input;
        
        // Wrap index using bitwise AND (efficient for power-of-2 sizes)
        self.index = (self.index + 1) & 1023;
        
        output
    }
}
```

### 3. Verification
- **Latency:** Exactly 0 samples of processing latency. The signal delay is purely the buffer distance.
- **Complexity:** O(1).
- **Branchless:** No `if` statements for index wrapping.

---
*Example L0 Implementation: CONFIRMED.*
