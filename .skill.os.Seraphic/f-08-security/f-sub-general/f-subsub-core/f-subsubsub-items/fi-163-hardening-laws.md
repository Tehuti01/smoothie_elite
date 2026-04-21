---
id: fi-163-hardening-laws.md
category: f-08-security
---

# 📜 HARDENING LAWS v0.2.0 (PRACTICES)

To maintain Citadel sovereignty, the following laws must be strictly enforced:

### 1. Mandatory Ed25519 Signing
All production assets (Binary, JSON, CSV) must be passed through the `sign_asset` tool before distribution.
- **Requirement:** Distribute the `.sig` file alongside the asset.

### 2. The Ouroboros Seal (Runtime)
Insert anti-tamper canaries into the hot path that check the integrity of the function pointers.
- **Law:** If a pointer is modified, trigger a controlled crash (Panic with Zero context).

### 3. Constant-Time Verification
All cryptographic operations must be constant-time to prevent side-channel (timing) attacks.
- **Goal:** Processing time remains identical whether the verification succeeds or fails.

### 4. No Environment-Variable Trust
Reject any configuration passed via `std::env` unless it is explicitly whitelisted in the **Era of Inception**.

---
*Citadel Hardening Protocol: ENFORCED.*
