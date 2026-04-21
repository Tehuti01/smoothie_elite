---
id: fi-2531-lock-free-laws.md
category: f-05-sysarch
---

# 📜 LOCK-FREE LAWS v0.2.0 (PRACTICES)

To maintain Atomic sovereignty, the following laws must be strictly enforced:

### 1. Sequential Consistency Prohibition
Audit every atomic operation for its memory ordering.
- **Requirement:** Change all `Ordering::SeqCst` to `Ordering::Relaxed`, `Acquire`, or `Release` unless global total ordering is physically required for external state.

### 2. Atomic Pointer Double-Buffering
When updating large parameter sets (e.g., Wavetables, Impulse Responses), use a double-buffering pattern with an `AtomicPtr`.
- **Law:** Swap the pointer in a single instruction. Never copy data in the audio thread.

### 3. Cache-Padded Counters
Any atomic variable used as a "Head" or "Tail" in a ring buffer must be padded to 64 bytes.
- **Goal:** Eliminate L1 cache thrashing during high-speed IPC.

### 4. No Busy-Waiting
Audio threads must never use `while !flag.load() {}`.
- **Constraint:** Use event-driven updates or lock-free polling that doesn't waste CPU cycles if no data is present.

---
*Atomic Pipeline Protocol: ENFORCED.*
