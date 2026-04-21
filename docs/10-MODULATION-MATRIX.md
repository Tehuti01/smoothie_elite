# Modulation Matrix

The `smoothie-modulation` crate provides a "modular synthesizer" approach to routing control signals within a plugin.

## 1. The ModMatrix

The `ModMatrix` allows any generic source to drive any generic destination, scaled by an amount.

```rust
use smoothie_modulation::{ModMatrix, ModSource, ModDest};

let mut matrix = ModMatrix::new();

// Route LFO 1 to the Filter Cutoff with an intensity of 0.85
matrix.connect(ModSource::Lfo(1), ModDest::FilterCutoff, 0.85);

// Route MIDI Velocity to VCA Volume
matrix.connect(ModSource::MidiVelocity, ModDest::Vca, 1.0);
```

## 2. Sources

### The Elite LFO (Low Frequency Oscillator)
A dedicated sub-audio rate oscillator.
- **Shapes:** Sine, Triangle, Square, Saw Up, Saw Down, Sample & Hold, Smooth Random.
- **Sync:** Can be free-running (Hz) or synced to the DAW's host tempo (e.g., 1/4 note, 1/8 dotted).

### Envelopes (ADSR)
The classic Attack-Decay-Sustain-Release generator.
- Triggered by MIDI Note On events.
- Attack and Decay curves are exponential, mimicking analog capacitor discharge rates.

### External Sources
- MIDI Control Change (CC) messages.
- MPE (MIDI Polyphonic Expression) slide and pressure data.
- Macro Knobs from the UI.

## 3. Execution

In the `process()` loop, the matrix is evaluated once per block (or per sample for audio-rate modulation).

```rust
// 1. Tick all sources
lfo.tick();
env.tick();

// 2. Evaluate the matrix routing
matrix.evaluate();

// 3. Apply the results to the DSP
let target_cutoff = base_cutoff + matrix.get_value(ModDest::FilterCutoff);
filter.set_frequency(target_cutoff);
```
