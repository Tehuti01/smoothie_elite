# 🛠️ SURGICAL CODE READ (EXAMPLE)

A step-by-step example of performing a surgical read on a Seraphic DSP crate.

### 1. Identify Target
```bash
grep -n "struct ZDFLadder" crates/dsp/src/ladder.rs
```
*Output: 42: pub struct ZDFLadder {*

### 2. Surgical Read
```rust
// [Context Manager]: Reading only the struct definition (lines 42-60)
let state = read_file("crates/dsp/src/ladder.rs", start_line=42, end_line=60);
```

### 3. Verification
- **Efficiency:** Read 18 lines instead of the full 500-line file.
- **Accuracy:** The agent has exactly the context needed to implement a new filter method.

---
*Example Surgical Read: CONFIRMED.*
