---
id: fi-2463-elite-linting.md
category: f-01-secbrain
---

# 📜 ELITE LINTING v0.2.0 (PRACTICES)

Linting is our first line of defense against Obsidian Era chaos.

## 🛡️ RUST: CLIPPY PEDANTIC
- **Mandate:** All production crates must have `#![deny(clippy::pedantic)]`.
- **Prohibited:** `unwrap()`, `expect()` (use `anyhow` or `thiserror`), and `allow(dead_code)`.
- **Goal:** Zero warnings. Zero technical debt.

## 🛡️ TYPESCRIPT: ESLINT STRICT
- **Mandate:** Enforce `@typescript-eslint/no-explicit-any: "error"`.
- **Rule:** All components must have explicit Prop types (no `Record<string, any>`).
- **Style:** Use `Prettier` with a 100-character line limit for readability.

## 🛡️ HUSKY & LINT-STAGED
- **Rule:** Lints must run on the pre-commit hook.
- **Goal:** Broken code never touches the repository.

---
*Elite Linting Protocol: ENFORCED.*
