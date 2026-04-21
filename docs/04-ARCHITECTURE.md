# High-Level Architecture

The Smoothie Elite framework is built on a modular, decoupled architecture that strictly isolates the high-priority, real-time audio thread from the non-deterministic UI and network threads.

## Core Layers

### 1. The Hardware Interface (Silicon Primitives)
Located in `smoothie-core::silicon`. This layer directly interfaces with CPU registers, cache lines, and SIMD vectors. It bypasses OS scheduling where possible.
- **Distributed Environment & Autonomousty:** Handles catastrophic hardware failures via custom panic handlers that safely reboot the DSP state without crashing the host DAW.
- **Convergence:** Manages legacy memory bridges.

### 2. The DSP Engine
Located in `smoothie-dsp`, `smoothie-fx`, `smoothie-synth`, and `smoothie-math`.
- Operates entirely on pre-allocated contiguous arrays (Slabs).
- All math is executed via `fast_math` approximations or direct SIMD intrinsics.
- Trigonometry and delay calculations utilize Industrial Geometry constants (`__PHI`).

### 3. The Parameter Registry
Located in `smoothie-params`.
- **Lock-Free:** Parameters are updated via `AtomicF32`, `AtomicI32`, etc.
- **Smoothing:** Every parameter contains a dedicated `Smoother` (Linear, Logarithmic, or Spring) to prevent audio clicks (zipper noise) when the user abruptly moves a knob.
- **Modulation Matrix:** Parameters can be bound to LFOs or Envelopes (`smoothie-modulation`) via zero-cost pointer indirection.

### 4. The Holographic UI
Located in `smoothie-ui` (egui) and `smoothie-graphics` (wgpu), extending into `smoothie-holographic` for GPU-side physics.
- The UI runs at 60-144Hz, completely isolated from the audio thread.
- State communication happens via bounded lock-free queues (`crossbeam_queue`) or atomic reads.
- Visuals are generated using Signed Distance Fields (SDFs) calculated entirely in fragment shaders.

### 5. The Neural Hive (AI)
Located in `smoothie-ai` and `smoothie-distributed-ai`.
- Capable of running `.onnx` models via the `tract` inference engine.
- Neural synthesis allows for mapping acoustic intent directly to DSP voltage control.
- P2P (Peer-to-Peer) networking via `libp2p` allows multiple instances of the plugin (even on different machines) to share DSP workloads.

### 6. The Format Wrappers
Located in `smoothie-vst3`, `smoothie-clap`, `smoothie-au`.
- These are thin ABI (Application Binary Interface) translation layers.
- They take the host DAW's C/C++ function calls, convert them to Rust safety boundaries, and pass them to the `SmoothiePlugin` trait.

## Thread Model

1. **Audio Thread (Real-Time):** Uninterruptible. No locks, no allocation, no blocking syscalls. Runs the `process()` function.
2. **Main Thread (GUI):** Handles window creation, mouse/keyboard input, and rendering commands to the GPU.
3. **Worker Threads (Async):** Background tasks powered by `tokio`. Handles preset loading, disk I/O, network telemetry, and neural weight streaming.
