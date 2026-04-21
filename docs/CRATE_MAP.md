# Crate Map — Smoothie Elite Workspace

Complete reference for all 51 crates in the workspace. Crates are organized by layer.  
**Key**: `[path]` → crate directory relative to `smoothie_elite/`.

---

## Foundation Layer

These crates have no internal dependencies. They are the bedrock of the framework.

| Crate | Path | Description | Key Exports |
|---|---|---|---|
| `smoothie-core` | `crates/core/` | The foundational crate. All plugins depend on this. Provides the `SmoothiePlugin` trait, audio types, math primitives, ring buffers, atomics, and `no_std` collections. | `SmoothiePlugin`, `AudioProcessor`, `ProcessStatus`, `PluginInfo`, `AudioBuffer`, `RingBuffer`, `AtomicF32` |
| `smoothie-math` | `crates/smoothie-math/` | Fast DSP math functions. Polynomial approximations are 4–8× faster than `std` equivalents. | `fast_sin`, `fast_cos`, `fast_tanh`, `exp_approx`, `db_to_linear`, `linear_to_db`, `midi_to_hz` |
| `smoothie-params` | `crates/smoothie-params/` | Host-automatable parameter system. Lock-free atomic backing with built-in smoothing. | `FloatParam`, `IntParam`, `BoolParam`, `Smoother`, `SmoothingStyle` |
| `smoothie-midi` | `crates/smoothie-midi/` | MIDI event types and parsing. `no_std` compatible. | `MidiEvent`, `NoteOn`, `NoteOff`, `ControlChange`, `PitchBend` |
| `logging` | `crates/logging/` | Real-time safe logging macros. Never allocates on the audio thread. | `rt_log!`, `warn!`, `debug!`, `trace!` |
| `serde` | `crates/serde/` | Binary serialization for plugin preset state. Compact wire format. | `Serialize`, `Deserialize`, `PresetEncoder`, `PresetDecoder` |
| `sync` | `crates/sync/` | Lightweight synchronization primitives (`SpinLock`, `Barrier`, `Once`). | `SpinLock`, `Barrier`, `CondVar`, `SmoothieMutex` |
| `async` | `crates/async/` | Minimal async runtime for background tasks (preset loading, network I/O). | `AsyncRuntime`, `Task`, `JoinHandle` |

---

## DSP Engine Layer

Audio signal processing — the computational heart of every plugin.

| Crate | Path | Description | Key Exports |
|---|---|---|---|
| `smoothie-dsp` | `crates/dsp/` | Core DSP building blocks: filters, oscillators, FFT, envelopes, wavetables. | `BiquadFilter`, `StateVariableFilter`, `LadderFilter`, `Oscillator`, `WavetableOscillator`, `FFT`, `AdsrEnvelope`, `ParametricEq` |
| `smoothie-effects` / `effects` | `crates/effects/` | Production-ready audio effects. | `ReverbEffect`, `DelayEffect`, `Compressor`, `Limiter`, `Gate`, `Saturator`, `Chorus`, `Phaser`, `Distortion` |
| `smoothie-dynamics` | `crates/smoothie-dynamics/` | Specialized dynamics processing with look-ahead and side-chain. | `Compressor`, `Limiter`, `Gate`, `Expander`, `Compander` |
| `smoothie-eq` | `crates/smoothie-eq/` | Full-featured equalizer engine. | `ParametricEq`, `GraphicEq`, `DynamicEq`, `EqBand` |
| `smoothie-reverb` | `crates/smoothie-reverb/` | Algorithmic and convolution reverb. | `FdnReverb`, `ConvolutionReverb`, `SchroederReverb` |
| `smoothie-spectrum` | `crates/smoothie-spectrum/` | FFT-based spectral analysis for display and processing. | `SpectrumAnalyzer`, `Spectrogram`, `MagnitudeSpectrum` |
| `smoothie-modulation` | `crates/smoothie-modulation/` | Modulation sources and the mod matrix. | `Lfo`, `AdsrEnvelope`, `ModMatrix`, `ModSource`, `ModDest` |
| `smoothie-tuning` | `crates/smoothie-tuning/` | Micro-tuning and MTS (MIDI Tuning Standard) support. | `MicroTuning`, `MtsTable`, `note_to_hz` |
| `smoothie-granular` | `crates/smoothie-granular/` | Granular synthesis engine. | `GranularSampler`, `GrainCloud`, `GrainEnvelope` |
| `smoothie-mastering` | `crates/smoothie-mastering/` | Chief Engineer bus processing (EBU R128, True-Peak limiting). | `LoudnessMeter`, `TruePeakLimiter`, `Chief EngineerBus` |
| `smoothie-sound-design` | `crates/smoothie-sound-design/` | Advanced sound design modules (vocoder, resonator, wavefield). | `Vocoder`, `Resonator`, `Wavefield` |
| `smoothie-physics` | `crates/smoothie-physics/` | Wave Digital Filter (WDF) circuit models. | `WdfResistor`, `WdfCapacitor`, `MoogLadderWdf` |
| `ironstack` | `crates/ironstack/` | High-performance integrated processing hub. IronStack assembles DSP chains from the engine crates. | `IronStackEngine`, `ProcessorChain`, `AmplifierModel` |
| `synth` | `crates/synth/` | Synthesis engines: FM, wavetable, granular, polyphonic voice management. | `FmEngine`, `WavetableSynth`, `VoiceManager`, `PolyBlepOscillator` |

---

## Plugin Format Layer

Thin ABI translation layers — converts host DAW C calls into safe Rust.

| Crate | Path | Description | Tested DAWs |
|---|---|---|---|
| `smoothie-vst3` | `crates/smoothie-vst3/` | VST3 format wrapper. Full parameter automation and MIDI CC. | Ableton 11+, FL Studio 21+, Cubase 12+, Studio One 6+, Reaper |
| `smoothie-clap` | `crates/smoothie-clap/` | CLAP format. Polyphonic expression, non-destructive automation. | Bitwig Studio, Reaper |
| `smoothie-au` | `crates/smoothie-au/` | Audio Units (AUv2 + experimental AUv3). | Logic Pro X, GarageBand |
| `smoothie-aax` | `crates/smoothie-aax/` | AAX for Pro Tools. Requires Avid AAX SDK at build time. | Pro Tools 2023+ |
| `smoothie-standalone` | `crates/smoothie-standalone/` | Desktop app shell. CPAL for system audio, midir for MIDI. | macOS/Windows/Linux |
| `smoothie-wasm` | `crates/smoothie-wasm/` | Browser-native via AudioWorklet + WASM-SIMD. | Chrome 102+, Firefox 101+ |

---

## UI & Graphics Layer

Immediate-mode GPU-accelerated UI — no DOM, no garbage collection.

| Crate | Path | Description | Key Exports |
|---|---|---|---|
| `smoothie-ui` | `crates/smoothie-ui/` | High-level UI definitions using egui. Provides knobs, sliders, meters, spectrum displays. | `UiContext`, `Knob`, `Slider`, `Fader`, `SpectrumDisplay` |
| `smoothie-ui-core` | `crates/smoothie-ui-core/` | Core widget primitives and layout engine. | `Widget`, `Layout`, `Color`, `Rect`, `Font` |
| `smoothie-ui-render` | `crates/smoothie-ui-render/` | wgpu rendering backend. Compiles to Metal, DX12, Vulkan. | `Renderer`, `RenderPass`, `GpuBuffer` |
| `smoothie-ui-vfx` | `crates/smoothie-ui-vfx/` | Visual effects: SDF-rendered knobs, glassmorphic panels, glow effects. | `SdfKnob`, `GlassPanel`, `GlowEffect`, `SeraphicTheme` |
| `smoothie-frontend` | `crates/smoothie-frontend/` | Tauri-based frontend for the Standalone app. | `StandaloneApp`, `AppConfig` |

---

## AI & Neural Layer

Machine learning inference running inside the plugin process.

| Crate | Path | Description | Key Exports |
|---|---|---|---|
| `smoothie-ai` | `crates/smoothie-ai/` | Runs ONNX models via the `tract` inference engine. Supports amp modeling and neural synthesis. | `NeuralModel`, `OnnxInference`, `AmpModel` |
| `smoothie-ai-core` | `crates/smoothie-ai-core/` | Neural DSP primitives: weight tensors, SIMD-aligned inference buffers. | `WeightTensor`, `InferenceBuffer`, `LstmCell` |

---

## Plugin-OS Ecosystem

Hot-swappable DSP and UI nodes. Each crate is an independent node library.

### DSP Nodes

| Crate | Description |
|---|---|
| `plugin-os-nodes-filter` | 20 filter nodes: ZDF ladder, SVF, Moog, FIR |
| `plugin-os-nodes-synth` | Wavetable, FM, additive oscillator nodes |
| `plugin-os-nodes-dyn` | Compressor, limiter, gate, soft clipper nodes |
| `plugin-os-nodes-fx-time` | Delay, reverb, echo nodes |
| `plugin-os-nodes-fx-mod` | Chorus, flanger, phaser nodes |
| `plugin-os-nodes-fx-dist` | Saturation, distortion, bitcrusher nodes |
| `plugin-os-nodes-osc` | Oscillator nodes: PolyBLEP, wavetable, noise |
| `plugin-os-nodes-env` | Envelope nodes: ADSR, AR, multi-stage |
| `plugin-os-nodes-lfo` | LFO nodes: sine, triangle, S&H, random |
| `plugin-os-nodes-math` | Math utility nodes: gain, mix, clamp, pan |

### UI Nodes

| Crate | Description |
|---|---|
| `plugin-os-ui-widgets-basic` | Knobs, sliders, buttons, toggles |
| `plugin-os-ui-widgets-visualizer` | Spectrum analyzer, oscilloscope, waveform |
| `plugin-os-ui-widgets-meter` | VU meters, RMS meters, gain reduction |
| `plugin-os-ui-widgets-3d` | 3D holographic controls |
| `plugin-os-ui-widgets-effect` | Effect-specific controls (EQ curve, compressor shape) |
| `plugin-os-ui-widgets-automation` | Automation lane display |
| `plugin-os-ui-widgets-graph` | Node graph editor |
| `plugin-os-ui-widgets-menu` | Context menus, dropdowns |
| `plugin-os-ui-widgets-input` | Text input, number fields |
| `plugin-os-ui-widgets-container` | Panels, tabs, scroll views |
| `plugin-os-ui-2-5d` | 2.5D layered UI components |

### Infrastructure

| Crate | Description |
|---|---|
| `plugin-os-core` | Node trait, IPC, state serialization |
| `plugin-os-bridge` | Zero-latency binary marshaling (node ↔ host) |
| `plugin-os-registry` | Runtime node discovery and registration |
| `plugin-os-graph` | Node graph scheduling and topology |
| `plugin-os-modulation` | Cross-node modulation routing |
| `plugin-os-preset` | Per-patch preset system |
| `plugin-os-script` | Scripting interface for node logic |
| `plugin-os-bundle` | Packaging nodes for distribution |
| `plugin-os-dev-tools` | Development utilities and node inspector |
| `plugin-os-bonus-premium` | Premium bonus nodes |
| `plugin-os-node` | Base node implementation helpers |

---

## System Model & Distributed Environment Layer

Autonomous monitoring and self-healing systems.

| Crate | Path | Description | Key Exports |
|---|---|---|---|
| `seraphic-prime` | `crates/seraphic-prime/` | Autonomous agent engine. Monitors DSP state and heals resonance breaches. | `Orchestrator`, `SkillRegistry`, `WorkingMemory`, `SemanticMemory` |
| `seraphic-agent` | `crates/seraphic-agent/` | Silicon-level introspection. Monitors instruction retirement and branch prediction. | `SiliconAuditor`, `BranchMonitor` |
| `seraphic-multiverse` | `crates/seraphic-multiverse/` | Multi-instance state synchronization across DAW sessions. | `Distributed EnvironmentSync`, `StateAnchor` |

---

## Tooling Layer

| Crate | Path | Description |
|---|---|---|
| `cli` | `crates/cli/` | `cargo smoothie` — scaffold, build, validate, info commands |
| `smoothie-cli-frontend` | `crates/smoothie-cli-frontend/` | CLI UI layer (terminal rendering) |
| `smoothie-cli-backend` | `crates/smoothie-cli-backend/` | CLI backend (project generation, template engine) |
| `smoothie-security` | `crates/smoothie-security/` | Hardware-locked licensing + HMAC key validation |
| `smoothie-audio-format` | `crates/smoothie-audio-format/` | Audio file I/O (WAV, AIFF, FLAC) for sample loading |
| `smoothie-preset` | `crates/smoothie-preset/` | TOML-based preset bank management |
| `smoothie-graph` | `crates/smoothie-graph/` | Processing graph scheduler (Kahn's algorithm) |
| `smoothie-net` | `crates/smoothie-net/` | Update checker and crash reporter |

---

## Dependency Graph (Simplified)

```
smoothie-core           (no internal deps)
    ↑
smoothie-dsp            (← core, math)
    ↑
effects / synth         (← dsp, core)
    ↑
ironstack               (← effects, dynamics, eq)
    ↑
smoothie-vst3 / clap    (← core, params, ironstack)
    ↑
Your Plugin             (← SmoothiePlugin trait)
```

---

*For detailed API docs, run `cargo doc --workspace --open`.*
