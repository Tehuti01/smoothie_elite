# Synthesis Engine

The `smoothie-synth` crate provides the architecture required to build professional polyphonic software instruments.

## 1. VoiceManager

The `VoiceManager` handles polyphony, note stealing, and voice allocation.

```rust
use smoothie_synth::VoiceManager;

// Create a manager with 32 voices of polyphony
let mut voices = VoiceManager::new(32);

// Handle a MIDI Note On
voices.note_on(60, 100); // Middle C, Velocity 100

// Handle a MIDI Note Off
voices.note_off(60);
```
When polyphony is exceeded, the manager uses a least-recently-used (LRU) algorithm with envelope awareness to gracefully steal the quietest, oldest voice.

## 2. Wavetable Synthesis

Wavetable synthesis allows for sweeping through complex harmonic structures.
- **WaveTables:** Uses `2048` sample long, band-limited waveforms.
- **Interpolation:** Utilizes high-order Hermite interpolation when reading the table to prevent tuning artifacts.

## 3. FM Engine (Frequency Modulation)

A complete 4-operator FM synthesis engine.
- Implements 32 routing algorithms (mimicking the classic Yamaha DX7 structure).
- Each operator features its own dedicated ADSR envelope and ratio scaling.

## 4. Subtractive Primitives

Standard analog-modeled oscillators are available:
- **PolyBLEP Saw, Square, and Triangle.**
- **White/Pink Noise Generators.**
- Can be routed into the `ZdfFilter` (`smoothie-dsp`) for classic Moog/Roland style subtractive synthesis.
