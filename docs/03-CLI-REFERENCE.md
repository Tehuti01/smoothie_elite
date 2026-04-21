# CLI & Build System Reference

Smoothie Elite utilizes a two-tier command-line architecture:
1. `cargo smoothie` for scaffolding, analysis, and project management.
2. `cargo xtask` for complex build automation, bundling, and CI/CD pipelines.

---

## 1. Cargo Smoothie

The `cargo-smoothie` binary acts as your primary development assistant.

### Commands

*   `cargo smoothie new <name> [--template <type>]`
    Creates a new plugin.
    *   `--template effect` (default)
    *   `--template instrument`
    *   `--template analyzer`

*   `cargo smoothie build`
    Standard compilation check. Equivalent to `cargo build`, but tailored with environment variables necessary for the framework.

*   `cargo smoothie standalone`
    Compiles and launches the plugin as a Tauri desktop application. Automatically configures CPAL for the primary system audio device and `midir` for the first available MIDI keyboard.

*   `cargo smoothie validate <path_to_binary>`
    Runs the internal `smoothie-validator`. This boots a headless host, instantiates the plugin, and runs stress tests (fuzzing parameters, chaotic block sizes, NaN injection) to ensure the plugin will not crash a DAW.

*   `cargo smoothie info`
    Parses `Cargo.toml` and the `SmoothiePlugin` trait to print a summary of the plugin's capabilities, latency, format support, and parameter count.

---

## 2. Cargo Xtask

`xtask` is the build automation suite. It handles the complex directory structures required by VST3 and CLAP formats, signs macOS bundles, and creates Windows installers.

### Commands

*   `cargo xtask bundle [--release] [--vst3] [--clap] [--au] [--aax] [--standalone]`
    Compiles the dynamic library (`.dylib`, `.so`, `.dll`) and packages it into the correct directory structure required by the format specifications.
    *   Creates `Contents/MacOS`, `Contents/Resources`, and `Info.plist` for macOS VST3/AU.

*   `cargo xtask install`
    Copies the bundled plugins from `target/.../bundle/` to the OS-specific plugin directories.
    *   **macOS:** `~/Library/Audio/Plug-Ins/VST3/`
    *   **Windows:** `C:\Program Files\Common Files\VST3\`
    *   **Linux:** `~/.vst3/`

*   `cargo xtask sign`
    (macOS only) Automatically signs the bundled binaries with your Apple Developer ID and prepares them for notarization.

*   `cargo xtask test`
    Runs the entire test suite across all crates, enforcing the `#[deny(clippy::all)]` and zero-allocation policies.

*   `cargo xtask clean`
    Purges the `target` directory and removes any symlinks created in the system plugin folders.
