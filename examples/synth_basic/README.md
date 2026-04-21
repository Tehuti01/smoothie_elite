# Synth Basic — Monophonic MIDI Synthesizer

> A monophonic synthesizer responding to MIDI note-on/off events with a PolyBLEP oscillator and ADSR envelope.

This example demonstrates how to build a software instrument (not just an effect) with Smoothie Elite.

## What It Demonstrates

- Implementing `SmoothiePlugin` as an **instrument** (`PluginCategory::Instrument`)
- Handling MIDI `NoteOn` and `NoteOff` events
- PolyBLEP oscillators — anti-aliased without a wavetable
- ADSR envelope controlling amplitude
- `midi_to_hz()` pitch conversion

## Parameters

| Index | Name | Range | Default | Unit |
|---|---|---|---|---|
| 0 | Attack | 0.001 – 5.0 | 0.01 | s |
| 1 | Decay | 0.001 – 5.0 | 0.1 | s |
| 2 | Sustain | 0.0 – 1.0 | 0.7 | linear |
| 3 | Release | 0.001 – 10.0 | 0.3 | s |
| 4 | Wave Shape | 0 – 2 | 0 | Saw/Square/Triangle |

## Building

```bash
cd examples/synth_basic
cargo build
cargo test
```

## Synthesizer Architecture

```
MIDI Note-On (pitch, velocity)
        │
        ▼
  ┌─────────────┐
  │  Oscillator  │  ← PolyBLEP Saw/Square/Triangle at note frequency
  └──────┬───────┘
         │
  ┌──────▼───────┐
  │ ADSR Envelope│  ← Amplitude shaping (Attack→Decay→Sustain→Release)
  └──────┬───────┘
         │
      Output
```

## Code Walkthrough

### 1. Handling MIDI Events

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    // Process MIDI events first (before audio)
    for event in &self.pending_midi {
        match event {
            MidiEvent::NoteOn { note, velocity } => {
                self.frequency = midi_to_hz(*note);
                self.velocity  = *velocity as f32 / 127.0;
                self.envelope.trigger();
            }
            MidiEvent::NoteOff { note, .. } => {
                if self.current_note == Some(*note) {
                    self.envelope.release();
                }
            }
        }
    }
    // ... audio processing
}
```

MIDI handling always happens before audio generation within the same block.

### 2. PolyBLEP Anti-Aliasing

A standard digital sawtooth wave has hard discontinuities that generate aliasing — high-frequency noise that folds back into the audible range. PolyBLEP mathematically subtracts these discontinuities:

```rust
fn poly_blep(t: f32, dt: f32) -> f32 {
    if t < dt {
        let t = t / dt;
        2.0 * t - t * t - 1.0       // Leading edge correction
    } else if t > 1.0 - dt {
        let t = (t - 1.0) / dt;
        t * t + 2.0 * t + 1.0       // Trailing edge correction
    } else {
        0.0
    }
}
```

The result is a bandlimited waveform that sounds clean at any pitch.

### 3. ADSR Envelope

```rust
// Attack: ramp from 0 → 1 over `attack` seconds
// Decay:  ramp from 1 → sustain over `decay` seconds
// Sustain: hold at `sustain` level until note-off
// Release: ramp from sustain → 0 over `release` seconds

let amplitude = self.envelope.process() * self.velocity;
*sample = self.oscillator.next_sample() * amplitude;
```

The envelope multiplies the oscillator output, shaping how the sound evolves over time.

### 4. `midi_to_hz()` — Converting MIDI notes to frequency

```rust
// MIDI note 69 = A4 = 440 Hz
// Each semitone is a factor of 2^(1/12)
fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
```

Available as `smoothie_core::math::midi_to_hz()`.

## What to Try Next

- Add a `LadderFilter` or `StateVariableFilter` between the oscillator and output for subtractive synthesis
- Implement 4-voice polyphony using a `VoiceManager` from `smoothie-synth`
- Modulate the filter cutoff with an additional ADSR envelope for classic synth sounds
- Add an LFO from `smoothie-modulation` for vibrato and tremolo
