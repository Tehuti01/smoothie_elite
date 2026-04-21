---
id: fi-2544-isolation-laws.md
category: f-05-sysarch
---

# 📜 ISOLATION LAWS v0.2.0 (PRACTICES)

To maintain Architectural sovereignty, the following laws must be strictly enforced:

### 1. The tier-stratification Audit
Before adding a new dependency, check its Tier.
- **Requirement:** `smoothie-core` (Tier 0) must never depend on `smoothie-ui` (Tier 3).
- **Goal:** Enable the "Silicon-Direct" path to compile on any target without UI bloat.

### 2. Mandatory Feature Gating
All platform-specific code (Windows, Mac, Linux) must be strictly feature-gated.
- **Law:** Use `#[cfg(feature = "vst3")]` or platform-specific gates to ensure only required symbols enter the binary.

### 3. Workspace-Level Dependency Management
Avoid version mismatches.
- **Requirement:** All `Cargo.toml` files in the workspace must use `dependency-name.workspace = true`.
- **Constraint:** All version strings live in the root `Cargo.toml` ONLY.

### 4. Zero Circular Dependencies
Detect and eliminate circular logic before the first build.
- **Tool:** Run `cargo tree -d` to ensure the DAG (Directed Acyclic Graph) is pure.

---
*Architectural Pipeline Protocol: ENFORCED.*
