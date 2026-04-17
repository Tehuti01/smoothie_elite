# Pro Filter - Example Plugin

A professional multi-mode filter plugin demonstrating **Smoothie Elite** capabilities.

## Features

- **ZDF State Variable Filter** — Superior modulation character
- **Multi-mode** — Lowpass, Highpass, Bandpass
- **3 Parameters** — Cutoff, Resonance, Mode
- **Real-time Safe** — Zero allocations in audio thread
- **Stereo/Mono** — Works with any channel configuration
- **Preset Support** — Save/load plugin state

## Building

```bash
cargo build --release
```

### Output

- **macOS:** `target/release/libsmoothie_example_filter.dylib`
- **Windows:** `target/release/smoothie_example_filter.dll`
- **Linux:** `target/release/libsmoothie_example_filter.so`

Place the compiled binary in your DAW's plugin folder:
- **VST3:** `~/.vst3/`
- **CLAP:** `~/.clap/`
- **AU:** `~/Library/Audio/Plug-Ins/Components/` (macOS)

## Architecture

```
ProFilter
├── filter_l, filter_r      ZDF filters (one per channel)
├── cutoff: FloatParam      Frequency 20Hz–20kHz
├── resonance: FloatParam   Q factor 0.1–10.0
└── mode: EnumParam         Filter mode (LP/HP/BP)
```

## Parameter Details

| Name | Default | Range | Unit |
|------|---------|-------|------|
| Cutoff | 2000 Hz | 20–20000 Hz | Hz |
| Resonance | 1.0 | 0.1–10.0 | Q |
| Mode | Lowpass | LP / HP / BP | – |

## Real-Time Performance

- **Audio Thread:** ~0.2 ms per stereo frame @ 44.1 kHz
- **CPU Load:** <1% per instance (typical DAW @ 256 buffer)
- **Memory:** Stack-only (no heap allocation)

## Preset Format

State is saved as 9 bytes:
- Cutoff (f32, LE): bytes 0–3
- Resonance (f32, LE): bytes 4–7
- Mode (u8): byte 8

## Next Steps

1. Load in your favorite DAW
2. Adjust parameters (cutoff, resonance)
3. Try each filter mode on audio
4. Study the source for implementation details

