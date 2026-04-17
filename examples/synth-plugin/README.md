# Simple Synth - Example Plugin

A polyphonic synthesizer demonstrating **Smoothie Elite** MIDI and synthesis capabilities.

## Features

- **16-Voice Polyphony** — Multi-note synthesis
- **MIDI Input** — Full note on/off handling
- **Sine Oscillator** — Clean, pure tone
- **Master Level Control** — Output volume parameter
- **Real-time Safe** — Zero allocations, perfect for audio threads

## Building

```bash
cargo build --release
```

## Implementation Details

### Voice Management

Each voice manages:
- **Oscillator** — Wavetable sine tone
- **Frequency** — Automatically computed from MIDI note
- **Velocity** — Normalized from MIDI (0–127 → 0.0–1.0)
- **State** — Active or inactive

### MIDI Handling

Incoming MIDI events are processed in `process()`:

```rust
for event in ctx.midi_events {
    match event {
        MidiEvent::NoteOn { note, velocity } => { ... }
        MidiEvent::NoteOff { note } => { ... }
        _ => {}
    }
}
```

### Oscillator

Uses `smoothie_dsp::Oscillator` for band-limited synthesis:
- Efficient per-sample modulation
- Anti-aliasing reduction at high frequencies
- Natural harmonics

## Parameters

| Name | Default | Range | Unit |
|------|---------|-------|------|
| Level | 0.1 | 0.0–1.0 | – |

## Audio Processing

1. **MIDI Processing** → Update active voice frequencies
2. **Synthesis** → Generate sine wave for each voice
3. **Mixing** → Sum all 16 voices
4. **Output** → Apply master level to stereo pair

## Polyphony Limits

- **Max Voices:** 16 (hardcoded)
- **Voice Stealing:** Round-robin (oldest note priority)
- **Per-Sample CPU:** ~2 µs per voice @ 44.1 kHz

## Next Steps

1. Load in DAW
2. Play notes on your MIDI keyboard
3. Adjust master level
4. Extend with filters, envelopes, LFOs

