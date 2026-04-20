---
name: strophe-26-error-sovereignty
description: "The Sovereign Codex for Error Handling. Governs the global error handling system of the Smoothie Elite framework. Enforces zero-allocation errors in the hot-path, PHI-aligned error codes, and high-performance result propagation. Essential for 12x industrial quality."
---

# 🌌 STROPHE 26: ERROR SOVEREIGNTY (ROUTER)

This strophe governs the **Logic Integrity** of the framework. We reject the ambiguity of panics and the bloat of dynamic error strings. We treat errors as high-speed data that must be propagated with zero latency.

### 🏛️ MAPPING THE ERROR SYSTEM
1.  **[01-Core/ERROR_INVARIANTS.md](01-Core/ERROR_INVARIANTS.md):** The physical principles of no-allocation errors, static strings, and error-enum stratification.
2.  **[02-Practices/PROPAGATION_LAWS.md](02-Practices/PROPAGATION_LAWS.md):** Rules for using `Result<T, SovereignError>`, avoiding `unwrap()`, and implementing `From` traits.
3.  **[03-Examples/DSP_ERROR_HANDLING.md](03-Examples/DSP_ERROR_HANDLING.md):** A step-by-step example of handling a filter stability error in the hot-path without heap access.
4.  **[04-Commands/error_auditor.rs](04-Commands/error_auditor.rs):** Rust tool that scans for `panic!`, `unwrap()`, and `expect()` violations.
5.  **[05-Meta/VERSION](05-Meta/VERSION):** Skill Version (v0.1.0).

---
*Logic Integrity: ACTIVE.*
*Zero-Allocation Errors: ENFORCED.*
