---
id: fi-122-memory-management-laws.md
category: f-06-dsp
---

# 📜 MEMORY MANAGEMENT LAWS (PRACTICES)

To maintain A0, following laws are mandated for all Hot-Path crates:

### 1. No Dynamic Resizing
Using `Vec::push`, `String::push_str`, or `HashMap::insert` is strictly prohibited during `process()`.
- **Alternative:** Use fixed-capacity buffers or `ArrayVec`.

### 2. Pre-Allocate Everything
Calculate the maximum possible memory requirement during the **Era of Inception**.
- **Requirement:** All buffers (Delay, Reverb, Wavetable) must be allocated at startup.

### 3. Smart Pointer Rejection
Avoid `Box<T>`, `Rc<T>`, and `Arc<T>` in the `process()` cycle unless they were created during Inception and are now read-only.
- **Guideline:** Use references `&T` or raw pointers `*const T` for high-speed access.

### 4. Lock-Free State Transitions
When the UI needs to update plugin state, use **Atomic Pointers** to swap between pre-allocated state objects.
- **Constraint:** Never allocate the new state on the fly.

---
*A0 Memory Protocol: ENFORCED.*
