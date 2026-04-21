# Quick Endpointt Guide

This guide covers how to scaffold, build, and deploy a professional audio plugin using Smoothie Elite.

## 1. Prerequisites

You need the latest stable Rust compiler, plus the Nightly toolchain for advanced silicon features (like inline assembly and SIMD).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
```

## 2. Install the CLI and Build Tools

Smoothie Elite is powered by two primary workflow tools: `cargo smoothie` (project generation and management) and `xtask` (build automation).

```bash
# Install the CLI
cargo install --path crates/smoothie-cli

# Install the xtask alias (optional but recommended)
cargo install --path xtask
```

## 3. Scaffold a New Plugin

Use the CLI to generate a new plugin template. You can choose from `effect`, `instrument`, `analyzer`, `utility`, or `midi`.

```bash
cargo smoothie new my-elite-reverb --template effect
cd my-elite-reverb
```

## 4. The Plugin Structure

Your generated plugin will look like this:

```
my-elite-reverb/
├── Cargo.toml
├── src/
│   ├── lib.rs          # The core DSP and parameter logic
│   ├── editor.rs       # The GPU-accelerated Bevy/egui UI
│   └── bin/
│       └── standalone.rs # Standalone app runner
└── assets/
    └── UI_assets...
```

## 5. The Process Loop (Zero-Allocation)

Open `src/lib.rs`. Your DSP logic lives inside the `process()` function.

```rust
impl SmoothiePlugin for MyEliteReverb {
    // ... metadata ...

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        // 1. Harmonize buffer (Phase XVII requirement)
        let mut buffer = ctx.as_omni_buffer();
        buffer.harmonize();

        // 2. Fetch smoothed parameters
        let mix = self.params.mix.smoothed.next();

        // 3. Process DSP (lock-free, zero-alloc)
        for (l, r) in buffer.iter_stereo_mut() {
            let (wet_l, wet_r) = self.reverb.process(*l, *r);
            *l = (*l * (1.0 - mix)) + (wet_l * mix);
            *r = (*r * (1.0 - mix)) + (wet_r * mix);
        }

        ProcessStatus::Ok
    }
}
```

## 6. Build and Deploy

Bundle the plugin for your target DAW:

```bash
# Build all formats (VST3, CLAP, Standalone)
cargo xtask bundle --release

# Or specific formats
cargo xtask bundle --vst3 --release
```

The resulting binaries will be located in the `target/release/bundle/` directory.

To automatically install them into your system's plugin folders (e.g., `/Library/Audio/Plug-Ins/VST3` on macOS):

```bash
cargo xtask install
```
