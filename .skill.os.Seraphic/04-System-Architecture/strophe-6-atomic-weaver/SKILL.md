---
name: strophe-6-atomic-weaver
description: "The Sovereign Mandate of the Atomic Weaver. Governs the thread-safe, lock-free fabric of the framework. Enforces atomic state transitions, wait-free SPSC buffers, and acquire/release memory ordering. Use this to ensure zero-latency communication between the UI and the Audio thread."
---

# 🌌 STROPHE 6: THE ATOMIC WEAVER (ROUTER)

This skill governs the **Atomic Weaver** (Threaded Fabric). It is the absolute law of lock-free communication. Follow the sub-folder path to achieve Threaded sovereignty:

### 🏛️ MAPPING THE ATOMIC WEAVER
1.  **[01-Core/ATOMIC_INVARIANTS.md](01-Core/ATOMIC_INVARIANTS.md):** The physical principles of memory ordering, atomic primitives, and the SPSC pattern.
2.  **[02-Practices/LOCK_FREE_LAWS.md](02-Practices/LOCK_FREE_LAWS.md):** Rules for avoiding mutexes, deadlocks, and priority inversion in the audio thread.
3.  **[03-Examples/SPSC_RING_BUFFER.md](03-Examples/SPSC_RING_BUFFER.md):** Step-by-step example of implementing a wait-free SPSC ring buffer for parameter updates.
4.  **[04-Commands/check_atomics.sh](04-Commands/check_atomics.sh):** Executable script for auditing code for atomic usage and potential race conditions.
5.  **[05-Meta/VERSION](05-Meta/VERSION):** Skill Version (v0.1.0).

---
*Threaded Fabric: ACTIVE.*
