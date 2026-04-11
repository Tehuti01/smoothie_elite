# Smoothie Elite — Framework Manifesto

> *"Elite audio plugin development shouldn't be hard. It should be smooth."*

---

## What Is Smoothie Elite?

**Smoothie Elite** is a comprehensive, open-source Rust framework for building professional-grade
audio plugins. It is the spiritual successor to JUCE (C++) and the evolutionary upgrade over
nih-plug — designed from the ground up to be:

- **Faster** than any existing framework
- **Safer** than C++ (Rust's memory safety, no undefined behavior, no use-after-free)
- **More expressive** (first-class TypeScript/TSX/JSX/Svelte/Vue/Angular UI support)
- **More complete** (VST3 + CLAP + AU + AAX in a single crate)
- **More beautiful** (GPU-accelerated animation, custom UI components, Seraphic design system)
- **More honest** (built-in plugin health checker, DAW compatibility validator, vulnerability scanner)

---

## Why We Built This

### The Problem with JUCE
JUCE is a 20-year-old C++ framework. It works, but:
- Requires an expensive commercial license for closed-source plugins
- C++ brings memory unsafety, undefined behavior, and complex build systems
- No modern UI paradigms (no TypeScript, no React, no TSX)
- Build times are slow. Compilation errors are cryptic
- No built-in safety or vulnerability checking

### The Problem with nih-plug
nih-plug is excellent and we respect it deeply, but:
- VST3 and CLAP only — no AU (macOS), no AAX (Pro Tools)
- UI options are limited (egui or vizia — no web-tech frontend)
- No built-in standalone with a proper native application shell
- No CLI tooling for plugin scaffolding
- No plugin validation / DAW compatibility engine
- No animation or GPU-accelerated graphics system
- No component library or design system

### Why Rust?
Rust gives us everything C++ promises but actually delivers:
- Zero-cost abstractions
- Real-time safe code (no garbage collector, no allocator in audio thread)
- Memory safety without runtime overhead
- First-class cross-platform compilation
- The best package manager (Cargo) of any systems language
- Growing audio ecosystem (cpal, rtrb, rustfft, dasp, biquad...)

---

## Goals

### Year 1 (v0.x — Foundation)
- [ ] Core plugin trait system better than nih-plug
- [ ] VST3, CLAP, AU, AAX format wrappers
- [ ] smoothie-params: type-safe, real-time-safe parameter system
- [ ] smoothie-dsp: complete DSP primitive library
- [ ] smoothie-midi: full MIDI 1.0 and MPE support
- [ ] smoothie-validator: plugin health check & DAW compat engine
- [ ] smoothie-graphics: wgpu-based GPU-accelerated rendering
- [ ] smoothie-ui: native component library (knobs, sliders, meters, etc.)
- [ ] smoothie_elite_standalone: Tauri-based standalone host (Seraphic branded)
- [ ] TypeScript/TSX/JSX frontend support
- [ ] cargo-smoothie: CLI for plugin scaffolding
- [ ] Prebuilt component library (publishable as NPM package)
- [ ] Animation system (spring physics, GPU keyframes)

### Year 2 (v1.x — Production)
- [ ] MIDI 2.0 support
- [ ] Plugin licensing/ownership system (DRM-optional, hardware-ID, cloud validation)
- [ ] Network-based preset sharing
- [ ] AI-assisted sound design (local inference)
- [ ] WebAssembly target (run plugins in browser)
- [ ] Plugin marketplace integration
- [ ] Automated regression testing harness

### Year 3 (v2.x — Elite)
- [ ] Full modular routing graph (like JUCE AudioProcessorGraph)
- [ ] Built-in oversampling engine
- [ ] Spectral processing toolkit
- [ ] Machine learning DSP (NAM, RAVE integration)
- [ ] Collaborative preset editing
- [ ] Plugin profiler & performance analyser
- [ ] Visual programming interface

---

## Architecture

```
smoothie-elite (workspace)
│
├── smoothie-core        → Plugin trait, AudioBuffer, ProcessContext, MIDI
├── smoothie-params      → Parameter system (type-safe, real-time-safe, automatable)
├── smoothie-dsp         → DSP primitives (filters, oscillators, dynamics, convolution)
├── smoothie-midi        → MIDI 1.0 / MPE / MIDI 2.0 support
├── smoothie-vst3        → VST3 wrapper (Steinberg ABI via vst3-sys)
├── smoothie-clap        → CLAP wrapper (C ABI via clap-sys)
├── smoothie-au          → Audio Units wrapper (macOS CoreAudio)
├── smoothie-aax         → AAX wrapper (Pro Tools — Avid SDK)
├── smoothie-graphics    → wgpu GPU rendering + animation engine
├── smoothie-ui          → Component library (knobs, sliders, meters, waveforms)
├── smoothie-presets     → Preset serialization, bank management
├── smoothie-validator   → Plugin health checker, DAW compat, vulnerability scanner
└── smoothie-cli         → cargo-smoothie CLI scaffolding tool

standalone/
└── smoothie_elite_standalone → Tauri app (Seraphic branded)
    ├── src-tauri/          → Rust backend (plugin loader, validator, audio engine)
    └── src/                → TypeScript/TSX frontend
        ├── components/     → React/TSX UI components
        └── styles/         → Seraphic design system (CSS/Tailwind)
```

---

## Supported Plugin Formats

| Format | Host | Status |
|--------|------|--------|
| VST3   | FL Studio, Ableton, Reaper, Bitwig, Cubase, Studio One... | ✅ Active |
| CLAP   | Bitwig, Reaper, FL Studio 21+... | ✅ Active |
| AU     | Logic Pro, GarageBand, MainStage (macOS only) | 🔧 In Progress |
| AAX    | Pro Tools (Avid SDK required) | 🔧 In Progress |
| Standalone | All — no host needed | ✅ Active |

---

## Supported DAWs

| DAW | VST3 | CLAP | AU | AAX |
|-----|------|------|----|-----|
| FL Studio | ✅ | ✅ (v21+) | ❌ | ❌ |
| Ableton Live | ✅ | 🔜 | ✅ (macOS) | ❌ |
| Logic Pro | ✅ | ❌ | ✅ | ❌ |
| Pro Tools | ❌ | ❌ | ❌ | ✅ |
| Reaper | ✅ | ✅ | ✅ | ❌ |
| Cubase / Nuendo | ✅ | ✅ | ❌ | ❌ |
| Studio One | ✅ | ✅ | ✅ | ❌ |
| Bitwig | ✅ | ✅ | ❌ | ❌ |
| Reason | ✅ | ❌ | ❌ | ❌ |
| GarageBand | ❌ | ❌ | ✅ | ❌ |

---

## Frontend Support

Smoothie Elite's UI system supports:
- **TypeScript / JavaScript** — first-class, always
- **TSX / JSX** — React-style component trees, compiled by Vite
- **Svelte** — reactive, zero-runtime
- **Vue 3** — composition API
- **Angular** — enterprise-ready
- **Plain HTML + CSS** — always works

All frontends compile to static assets embedded in the plugin binary.
The Tauri standalone app extends this with full native OS integration.

---

## The Smoothie_Elite_StandAlone

The standalone host (`smoothie_elite_standalone`) is a Tauri-based desktop application with:
- **Seraphic Sonic branding** (logo, colors, no Tauri watermark)
- **Plugin loader** — load any VST3/CLAP/AU/AAX built with Smoothie Elite
- **Health checker** — shows exactly which formats, DAWs, features are supported
- **Vulnerability scanner** — checks for common audio plugin security issues
- **Parameter explorer** — browse and automate all plugin parameters
- **Preset manager** — load, save, and share presets
- **Audio I/O configurator** — choose interface, sample rate, buffer size
- **MIDI monitor** — see incoming MIDI in real time
- **Latency analyser** — measure plugin latency accurately
- **Settings panel** — global framework configuration

---

## What We Learned from JUCE's 1000+ Features

Rather than just copying JUCE, we studied its architecture and rebuilt every feature
in Rust with modern idioms:

| JUCE | Smoothie Elite | Improvement |
|------|----------------|-------------|
| `AudioProcessor` | `SmoothiePlugin` trait | Cleaner, zero-overhead |
| `AudioProcessorGraph` | `PluginGraph` | Type-safe routing |
| `APVTS` | `smoothie-params` | Real-time safe, proc-macro driven |
| `ValueTree` | Serde-based state | Standard serialization |
| `AudioBuffer<float>` | `AudioBuffer<S>` | Generic sample type |
| `MidiBuffer` | `smoothie-midi` | MIDI 2.0 ready |
| `dsp::IIR` | `smoothie-dsp` filters | SIMD-optimized |
| `dsp::Convolution` | IR convolution | GPU-accelerated option |
| `dsp::Oversampling` | Oversampling engine | Phase-linear option |
| `OpenGLRenderer` | wgpu renderer | Modern GPU API |
| `AnimatedAppComponent` | Animation engine | Spring physics built in |
| `AudioThumbnail` | Waveform widget | Real-time waveform |
| `Reverb` | Reverb unit | Algorithmic + convolution |
| Projucer | `cargo-smoothie` | Just `cargo new` |
| JUCE licensing | MIT + Apache 2.0 | Completely free, forever |

---

## Philosophy

1. **Elite by default.** The framework should produce production-quality plugins out of the box.
2. **Free forever.** Open source, MIT + Apache 2.0. No commercial license needed.
3. **Bare metal where it counts.** Audio thread runs with zero allocations, zero locks, zero GC.
4. **Modern everywhere else.** TypeScript frontends, Tauri integration, NPM component publishing.
5. **Honest.** Built-in validation tells you exactly what works, what doesn't, and why.
6. **Composable.** Use one crate or all of them. Smoothie Elite is modular.

---

*Built by Seraphic Sonic. MIT / Apache-2.0 licensed.*
*This document is a living specification — updated as the framework evolves.*
