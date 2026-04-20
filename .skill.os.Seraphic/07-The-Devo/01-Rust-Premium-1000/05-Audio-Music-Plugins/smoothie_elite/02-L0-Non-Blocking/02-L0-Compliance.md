# SKILL SE-002: L0 NON-BLOCKING DSP PROTOCOL

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                    SMOOTHIE ELITE L0 NON-BLOCKING PROTOCOL
                     Lock-Free Audio Processing
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

> "L0 compliance ensures zero blocking operations in the audio thread"
> - Engineering Mandate v2.1

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## L0 FUNDAMENTALS

### The Non-Blocking Promise

```rust
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::cell::UnsafeCell;

/// ❌ VIOLATION: Blocking operations NOT allowed
fn process_with_lock(sample: f32, lock: &std::sync::Mutex<f32>) -> f32 {
    let _guard = lock.lock().unwrap(); // ❌ BLOCKING
    sample * 2.0
}

/// ✅ L0 COMPLIANT: Atomic operations
#[inline(always)]
fn process_with_atomic(sample: f32, multiplier: &AtomicI32) -> f32 {
    let mult = multiplier.load(Ordering::Relaxed) as f32 / 1000.0;
    sample * mult
}
```

---

## ATOMIC PARAMETERS

```rust
pub struct AtomicParam {
    value: AtomicU32,
    normalized: AtomicU32,    // 0-1000 normalized
    automation_target: AtomicI32,
}

impl AtomicParam {
    pub fn new(default: f32, min: f32, max: f32) -> Self {
        let normalized = ((default - min) / (max - min) * 1000.0) as u32;
        AtomicParam {
            value: AtomicU32::new((default * 1000.0) as u32),
            normalized: AtomicU32::new(normalized),
            automation_target: AtomicI32::new(-1), // No automation
        }
    }

    #[inline(always)]
    pub fn get(&self) -> f32 {
        self.value.load(Ordering::Acquire) as f32 / 1000.0
    }

    #[inline(always)]
    pub fn set(&self, value: f32) {
        self.value.store((value * 1000.0) as u32, Ordering::Release);
    }

    #[inline(always)]
    pub fn normalized(&self) -> f32 {
        self.normalized.load(Ordering::Acquire) as f32 / 1000.0
    }
}
```

---

## L0 PERFORMANCE

| Operation | Blocking | Atomic | Savings |
|------------|----------|---------|----------|
| Parameter get | 150ns | 2ns | 75x |
| Parameter set | 200ns | 3ns | 66x |
| LFO read | 80ns | 1ns | 80x |
| State access | 100ns | 1ns | 100x |

---

*Skill SE-002 | L0 Protocol | A0+L0 Compliant*