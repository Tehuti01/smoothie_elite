# Presets, Licensing & Network

## 1. Presets (`smoothie-presets`)

Presets are stored in a human-readable, version-control friendly format: **TOML**.

```rust
use smoothie_presets::{PresetBank, Preset};

// Serialize the current plugin state to a Preset
let current_state = Preset::capture("My Heavy Bass", &plugin);

// Load a preset
let preset = bank.get("My Heavy Bass")?;
preset.apply_to(&mut plugin);
```
The framework automatically handles exposing these presets to the host DAW (e.g., the VST3 preset dropdown) and saving the state within DAW project files.

## 2. Licensing (`smoothie-licensing`)

A robust, hardware-locked licensing system that requires no internet connection for validation.

- **Hardware Fingerprint:** Generates a unique machine ID based on CPU ID, motherboard UUID, and OS identifiers.
- **HMAC Validation:** License keys are cryptographically signed.
- **Trial Mode:** Built-in logic for "14-Day Trial" or "Intermittent Noise" restrictions for unauthorized users.

```rust
let mgr = LicenseManager::new("com.yourname.plugin");
match mgr.check() {
    LicenseStatus::Licensed => { /* Full execution */ },
    LicenseStatus::Trial { days } => { /* Warning UI */ },
}
```

## 3. Telemetry (`smoothie-network`)

Optional, opt-in network features:
- **Update Checker:** Pings a remote server (e.g., GitHub Releases) to notify the user of a new version.
- **Crash Reporter:** If a panic occurs outside of the `universal_panic_handler`, the network module can generate a stack trace and send it to the developer.
