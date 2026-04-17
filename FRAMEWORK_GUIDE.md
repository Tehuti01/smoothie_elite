# Smoothie Elite Plugin Framework Guide

**Smoothie Elite** is a high-performance, zero-allocation Rust audio plugin framework supporting VST3, CLAP, AU, AAX, and standalone formats.

## Quick Start

### 1. Define Your Plugin

Implement the `SmoothiePlugin` trait:

```rust
use smoothie_core::prelude::*;
use smoothie_dsp::filters::ZdfFilter;
use std::sync::Arc;

pub struct MyPlugin {
    zdf: ZdfFilter,
    cutoff: f32,
    resonance: f32,
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            zdf: ZdfFilter::new(),
            cutoff: 1000.0,
            resonance: 1.0,
        }
    }
}

impl SmoothiePlugin for MyPlugin {
    const NAME: &'static str = "My Filter";
    const VENDOR: &'static str = "My Company";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid(*b"MYFLT001");
    
    fn audio_layouts() -> &'static [AudioLayout] {
        &[AudioLayout::stereo()]
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        for frame in ctx.frames_mut() {
            // Update filter on parameter changes
            self.zdf.set_params(self.cutoff, self.resonance, ctx.sample_rate as f32);
            
            // Process stereo samples
            frame[0] = self.zdf.process_lowpass(frame[0]);
            frame[1] = self.zdf.process_lowpass(frame[1]);
        }
        ProcessStatus::Ok
    }
}
```

### 2. Export the Plugin

Add this at the top of your plugin crate:

```rust
smoothie_export!(MyPlugin);
```

This automatically generates VST3, CLAP, AU, and AAX exports.

---

## Core Concepts

### The SmoothiePlugin Trait

Every plugin implements `SmoothiePlugin`, which defines:

| Section | Methods | Purpose |
|---------|---------|---------|
| **Identity** | `NAME`, `VENDOR`, `VERSION`, `UID` | Plugin metadata |
| **Audio I/O** | `audio_layouts()` | Supported channel configs |
| **Parameters** | `parameters()` | Automatable parameters |
| **Lifecycle** | `initialize()`, `reset()`, `process()`, `deactivate()` | Plugin state |
| **GUI** | `has_editor()`, `create_editor()`, `destroy_editor()` | Optional editor |
| **State** | `save_state()`, `load_state()` | Preset serialization |
| **Tail** | `tail_length_samples()` | Reverb/delay tail length |

#### ProcessContext

The `process()` method receives a `ProcessContext` containing:

```rust
pub struct ProcessContext<'a> {
    pub frames: &'a mut [f32],      // Interleaved audio (L,R,L,R...)
    pub sample_rate: u32,
    pub tempo: f64,                 // BPM
    pub time_signature: (u32, u32),
    pub frame_number: u64,
    pub midi_events: &'a [MidiEvent],  // Incoming MIDI
}

impl ProcessContext<'_> {
    pub fn frames_mut(&mut self) -> impl Iterator<Item = &mut [f32]> { ... }
    pub fn frames(&self) -> impl Iterator<Item = &[f32]> { ... }
}
```

---

## Parameter System

Smoothie Elite's parameter system is **zero-allocation** and **real-time safe**.

### Parameter Types

#### FloatParam

Continuous parameters with smoothing:

```rust
pub struct FloatParam {
    value: Arc<AtomicF32>,
    default: f32,
    min: f32,
    max: f32,
    name: &'static str,
    // ... smoothing options
}
```

Usage:
```rust
let cutoff = FloatParam::new("Cutoff", 1000.0, 20.0, 20000.0);
let value = cutoff.value();  // Get current value
cutoff.set(2000.0);          // Set (safe from audio thread)
```

#### IntParam / EnumParam

Integer and enumerated parameters:

```rust
let filter_type = EnumParam::new("Type", 0, &["Lowpass", "Highpass", "Bandpass"]);
let polyphony = IntParam::new("Voices", 8, 1, 16);
```

#### BoolParam

Boolean toggle parameters:

```rust
let enabled = BoolParam::new("Enabled", true);
```

### Automation

Return all parameters via `parameters()`:

```rust
fn parameters(&self) -> Vec<Arc<dyn Param>> {
    vec![
        Arc::new(self.cutoff.clone()),
        Arc::new(self.resonance.clone()),
        Arc::new(self.type.clone()),
    ]
}
```

The host uses this to build the automation list and apply changes in real-time.

### ParamRegistry

For complex plugins, use `ParamRegistry`:

```rust
let mut registry = ParamRegistry::new();
registry.add(Arc::new(cutoff));
registry.add(Arc::new(resonance));

// Later:
for param in &registry.params {
    println!("{}", param.display());
}
```

---

## DSP Modules

### Filters

#### BiquadFilter (Direct-Form II Transposed)

Classic, stable biquad filter:

```rust
use smoothie_dsp::filters::{BiquadFilter, FilterType};

let mut bq = BiquadFilter::lowpass(1000.0, 0.707, 44100.0);
let output = bq.process(input_sample);
bq.reset();  // Reset state on transport restart
```

Supports: Lowpass, Highpass, Bandpass, Peaking, Shelf filters.

#### ZdfFilter (Zero-Delay Feedback)

Professional-grade state variable filter with superior modulation:

```rust
use smoothie_dsp::filters::ZdfFilter;

let mut zdf = ZdfFilter::new();
zdf.set_params(freq_hz, resonance, sample_rate);

let lp = zdf.process_lowpass(input);
let hp = zdf.process_highpass(input);
let bp = zdf.process_bandpass(input);
```

**Why ZDF?**
- Excellent modulation behavior (sweep without artifacts)
- Analog character (topology-preserving transform)
- Per-sample parameter changes without recomputation

### Oscillators

Wavetable oscillators with anti-aliasing:

```rust
use smoothie_dsp::oscillator::Oscillator;

let mut osc = Oscillator::new();
osc.set_frequency(440.0, sample_rate);
osc.set_phase(0.0);

let sample = osc.process(Waveform::Sine);
```

Features:
- Band-limited wavetables (reduce aliasing)
- Phase modulation support
- High-frequency aliasing reduction

---

## Plugin Formats

### VST3

Export via `smoothie-vst3`:

```toml
[dependencies]
smoothie-vst3 = { workspace = true }
```

Automatic features:
- Parameter automation
- Preset save/load
- MIDI support
- Sample-accurate parameter changes

### CLAP

Export via `smoothie-clap`:

```toml
[dependencies]
smoothie-clap = { workspace = true }
```

Automatic features:
- Parameter automation
- Fast parameter lookup
- Extension support ready

### AU / AAX

Similarly handled via `smoothie-au` and `smoothie-aax`.

---

## Complete Example: Simple Synth

```rust
use smoothie_core::prelude::*;
use smoothie_dsp::oscillator::Oscillator;
use smoothie_dsp::filters::ZdfFilter;
use smoothie_params::FloatParam;
use std::sync::Arc;

pub struct SimpleSynth {
    osc: Oscillator,
    filter: ZdfFilter,
    
    // Parameters
    cutoff: Arc<FloatParam>,
    resonance: Arc<FloatParam>,
    
    // State
    phase: f32,
}

impl Default for SimpleSynth {
    fn default() -> Self {
        Self {
            osc: Oscillator::new(),
            filter: ZdfFilter::new(),
            cutoff: Arc::new(FloatParam::new("Cutoff", 2000.0, 20.0, 20000.0)),
            resonance: Arc::new(FloatParam::new("Q", 1.0, 0.1, 10.0)),
            phase: 0.0,
        }
    }
}

impl SmoothiePlugin for SimpleSynth {
    const NAME: &'static str = "Simple Synth";
    const VENDOR: &'static str = "My Studio";
    const VERSION: &'static str = "0.1.0";
    const UID: PluginUid = PluginUid(*b"SMSY0001");
    
    fn audio_layouts() -> &'static [AudioLayout] {
        &[AudioLayout::stereo()]
    }

    fn parameters(&self) -> Vec<Arc<dyn smoothie_params::Param>> {
        vec![
            Arc::clone(&self.cutoff) as Arc<dyn smoothie_params::Param>,
            Arc::clone(&self.resonance) as Arc<dyn smoothie_params::Param>,
        ]
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        let cutoff = self.cutoff.value();
        let resonance = self.resonance.value();
        
        self.filter.set_params(cutoff, resonance, ctx.sample_rate as f32);
        
        for frame in ctx.frames_mut() {
            // Generate oscillator
            let osc_out = self.osc.process(smoothie_dsp::oscillator::Waveform::Sine);
            
            // Filter
            let filtered = self.filter.process_lowpass(osc_out);
            
            // Output (mono to stereo)
            frame[0] = filtered * 0.1;
            frame[1] = filtered * 0.1;
        }
        
        ProcessStatus::Ok
    }

    fn reset(&mut self) {
        self.filter.reset();
    }
}

smoothie_export!(SimpleSynth);
```

---

## Building & Testing

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

Optimizations enabled:
- `opt-level = 3` (aggressive)
- `lto = "thin"` (link-time optimization)
- `codegen-units = 1` (slower but better optimization)
- `panic = "abort"` (no unwind overhead)

### Testing

Run tests with:

```bash
cargo test
```

**Audio thread safety:**
- No heap allocation in `process()`
- Atomics only for parameter changes
- No panic allowed in real-time code

---

## Architecture

```
smoothie_elite/
├── crates/
│   ├── smoothie-core/        # SmoothiePlugin trait, audio buffer
│   ├── smoothie-params/      # Parameter system (FloatParam, etc)
│   ├── smoothie-dsp/         # Filters, oscillators, DSP utils
│   ├── smoothie-midi/        # MIDI event handling
│   ├── smoothie-presets/     # Preset save/load
│   │
│   ├── smoothie-clap/        # CLAP format wrapper
│   ├── smoothie-vst3/        # VST3 format wrapper
│   ├── smoothie-au/          # AU format wrapper
│   └── smoothie-aax/         # AAX format wrapper
│
└── FRAMEWORK_GUIDE.md        # This file
```

---

## Best Practices

### Real-Time Safety

✅ **DO:**
- Use `Arc<AtomicF32>` for parameters
- Allocate buffers in `initialize()`
- Process exactly what `ProcessContext` provides

❌ **DON'T:**
- Allocate memory in `process()`
- Lock mutexes in `process()`
- Call I/O functions from audio thread
- Panic in audio code

### Parameter Design

- Default values should be musically sensible
- Use proper ranges (e.g., 20Hz–20kHz for frequency)
- Provide display strings for UI ("2.5 dB", "440 Hz")

### DSP Quality

- Use ZDF for modulated filters (better sound)
- Use biquads for fixed filters (slightly cheaper)
- Test aliasing artifacts at high frequencies
- Consider sample rate when tuning algorithms

---

## Troubleshooting

**Plugin doesn't load in DAW:**
- Check `UID` is unique (32-bit little-endian)
- Verify `audio_layouts()` returns at least one layout
- Build in release mode for production

**Crackling audio:**
- Avoid heap allocation in `process()`
- Check for NaN/Inf values
- Reduce number of parameter reads per sample

**Parameter changes are laggy:**
- Verify `parameters()` returns all parameters
- Check smoothing settings on FloatParam
- Consider using EnumParam instead of float for modes

---

## Resources

- **VST3 Spec:** https://github.com/RustAudio/vst3-sys
- **CLAP Spec:** https://github.com/micahrj/clap-sys
- **DSP References:** See individual crate READMEs

