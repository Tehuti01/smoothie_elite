# AI & Neural Hive

The `smoothie-ai` and `smoothie-distributed-ai` crates introduce machine learning and distributed computing directly into the DSP environment.

## 1. Neural DSP Synthesis (`smoothie-ai`)

Using the `tract` inference engine, Smoothie Elite can run pre-trained ONNX models directly on the audio thread (provided the model is small enough for real-time inference, such as WaveNet derivatives or LSTM amp models).

```rust
// 1. Load an ONNX model representing an analog guitar amplifier
let amp_model = load_onnx_model("assets/ironstack_jcm800.onnx");

// 2. Stream audio through the neural net
let processed_block = amp_model.infer(audio_block);
```

## 2. Direct Hardware Inference (Phase XVII)

For supported platforms (NPU, Apple Neural Engine), the framework can bypass traditional CPU logic gates and manifest audio voltage using direct intent mapping.

```rust
// `smoothie-core::silicon::optimization::NeuralSpecificationor`
unsafe {
    manifestor.manifest_voltage(target_bus);
}
```

## 3. The Neural Hive (`smoothie-distributed-ai`)

This crate implements a Peer-to-Peer (P2P) network using `libp2p`.

- **Concept:** When running multiple instances of a heavy Neural DSP plugin (e.g., 20 instances of an AI Reverb in a mix), a single CPU may choke.
- **The Hive:** The plugin instances automatically discover each other via mDNS or local gossip sub-protocols. They form a "Hive".
- **Distributed Computing:** Workloads are balanced across the Hive. If you have two computers running the same DAW session (or connected via LAN), the Hive will distribute the DSP processing across both machines seamlessly.
