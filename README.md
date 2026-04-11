# Smoothie Elite

> **The professional Rust framework for building world-class audio plugins.**  
> VST3 · CLAP · AU · AAX · Standalone — all from one codebase.

<p align="center">
  <img src="assets/smoothie-elite-banner.png" alt="Smoothie Elite" width="800"/>
</p>

<p align="center">
  <a href="https://github.com/Tehuti01/smoothie_elite/actions"><img src="https://img.shields.io/github/actions/workflow/status/Tehuti01/smoothie_elite/ci.yml?style=flat-square&label=CI" alt="CI"/></a>
  <a href="https://crates.io/crates/smoothie-core"><img src="https://img.shields.io/crates/v/smoothie-core?style=flat-square" alt="crates.io"/></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square" alt="License"/></a>
  <img src="https://img.shields.io/badge/rust-2021-orange?style=flat-square" alt="Rust Edition"/>
  <img src="https://img.shields.io/badge/platforms-macOS%20·%20Windows%20·%20Linux-lightgrey?style=flat-square" alt="Platforms"/>
</p>

---

## What is Smoothie Elite?

Smoothie Elite is a **complete, production-ready Rust framework** for creating professional audio plugins. It gives you everything — DSP algorithms, effects processing, synthesis engines, plugin format wrappers, a GPU-accelerated UI toolkit, preset management, licensing, and a CLI — so you can focus entirely on your sound, not infrastructure.

It is built from the ground up in Rust for:
- **Zero allocation on the audio thread** — no heap allocations in the hot path, ever
- **Real-time safety** — lock-free data structures, atomic parameters, wait-free ring buffers
- **Cross-platform** — macOS (Intel + Apple Silicon), Windows (x64), Linux (x64)
- **All major plugin formats** — VST3, CLAP, AU, AAX, and Standalone in one build

---

## Table of Contents

- [Features](#features)
- [Crate Overview](#crate-overview)
- [Quick Start](#quick-start)
- [Plugin Formats](#plugin-formats)
- [DSP Modules](#dsp-modules)
- [Effects Library](#effects-library)
- [Synthesis Engine](#synthesis-engine)
- [Modulation System](#modulation-system)
- [Parameter System](#parameter-system)
- [UI & Graphics](#ui--graphics)
- [Preset System](#preset-system)
- [MIDI System](#midi-system)
- [Licensing System](#licensing-system)
- [CLI — cargo smoothie](#cli--cargo-smoothie)
- [Build System — cargo xtask](#build-system--cargo-xtask)
- [DAW Compatibility](#daw-compatibility)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [License](#license)

---

## Features

### Plugin Formats
| Format | Status | Notes |
|--------|--------|-------|
| VST3   | ✅ Full | Steinberg SDK ABI, full parameter automation |
| CLAP   | ✅ Full | Modern open standard, polyphonic expression |
| AU     | 🔧 v0.2 | Audio Units v2/v3 (macOS) |
| AAX    | 🔧 v0.3 | Pro Tools format |
| Standalone | ✅ Full | Tauri desktop app with React/TSX UI |

### DSP Algorithms
- **Filters** — Biquad (LP, HP, BP, Notch, Peak, Shelf), One-pole, SVF (state variable), Allpass, Comb, Halfband
- **Oversampling** — 2× and 4× with polyphase halfband anti-aliasing
- **Distortion** — Softclip, Hardclip, Tanh shaper, Foldback, Asymmetric tube, Wavefolder, Chebyshev harmonics
- **Dynamics** — Compressor (RMS/peak, soft knee), Limiter (brick-wall, lookahead), Expander, Noise gate
- **Analysis** — FFT spectrum analyzer, LUFS meter (ITU-R BS.1770-4), True peak, RMS/VU, Pitch detector, Transient detector, BPM detector

### Effects (smoothie-fx)
| Effect | Type | Description |
|--------|------|-------------|
| FDN Reverb | Time | 8-line Feedback Delay Network with Hadamard mixing |
| Schroeder Reverb | Time | Classic 4-comb + 2-allpass topology |
| Stereo Delay | Time | Ping-pong, tempo-sync, hermite interpolation |
| Chorus | Modulation | 4-voice LFO modulated, stereo spread |
| Phaser | Modulation | Up to 12 allpass stages |
| Flanger | Modulation | BLEP-corrected, positive/negative feedback |
| Tremolo | Modulation | 6 LFO shapes, stereo mode |
| Vibrato | Modulation | Pitch-shifting delay with LFO |
| Auto-Wah | Filter | Envelope-following SVF sweep |
| Bitcrusher | Destruction | Bit-depth + sample-rate reduction with dither |
| Ring Modulator | Spectral | Carrier oscillator with wet/dry |
| Frequency Shifter | Spectral | Single-sideband via Hilbert transform |
| Pitch Shifter | Pitch | OLA-based with fractional delay |
| Stereo Widener | Stereo | Mid/side width control |
| Haas Widener | Stereo | Stereo spread via micro-delay |
| Crossfeed | Stereo | Headphone speaker simulation |
| Transient Shaper | Dynamics | Attack/sustain via envelope differentiation |
| Harmonic Exciter | Enhancement | HF harmonic generation + blend |
| Aural Exciter | Enhancement | Chebyshev polynomial 2nd/3rd harmonics |
| Tape Saturation | Saturation | HF roll-off, bias distortion, noise |
| Wow & Flutter | Modulation | Tape pitch instability simulation |
| Cabinet Sim | Tone | FIR convolution with built-in 1x12 / 4x12 IRs |
| Tube Overdrive | Distortion | Asymmetric tube character |
| Fuzz | Distortion | Hard clip + bias, DC blocked |
| Diode Clipper | Distortion | Germanium-style sigmoid |
| Wavefolder | Distortion | Multi-stage sine wavefolder |
| Parametric EQ | Filter | 8-band fully parametric |
| Compressor | Dynamics | RMS, soft-knee, auto-makeup |
| Stereo Compressor | Dynamics | Stereo-linked RMS compressor |
| Brick-wall Limiter | Dynamics | Lookahead, true peak, gain reduction metering |
| Expander | Dynamics | Downward expansion with range |
| Noise Gate | Dynamics | Hysteresis, hold time, attack/release |

### Math Library (smoothie-math)
- Fast approximations: `fast_sin`, `fast_cos`, `fast_tanh`, `fast_exp2`, `fast_log2`
- Smoothstep / smootherstep
- Window functions: Hann, Hamming, Blackman, Blackman-Harris, Flat-top, Kaiser, Triangular, Rectangular
- Interpolation: Linear, Hermite (4-point), Catmull-Rom, B-Spline, Optimal 4-point 4th-order, Allpass
- Frequency: MIDI↔Hz, semitones↔ratio, cents↔ratio, note names
- dB: linear↔dB, power↔dB, equal-power pan, stereo balance
- Perceptual scales: Mel, Bark, ERB, A-weighting, equal-loudness
- Statistics: RMS, peak, crest factor, mean, variance, ZCR
- Envelope followers: Peak, RMS, VU meter ballistics
- Phasor, PolyBLEP sawtooth/square, sinc
- Noise: White, Pink (Kellett 6-stage), Velvet
- Matrix math: 4×4, Hadamard in-place, Householder reflection
- Random: Xorshift32 PRNG (lock-free, allocation-free)

---

## Crate Overview

Smoothie Elite is a **Cargo workspace** of focused, composable crates:

```
smoothie-elite/
├── crates/
│   ├── smoothie-core        # Plugin trait, audio buffer, context types
│   ├── smoothie-params      # Parameter system with smoothing
│   ├── smoothie-dsp         # Biquad filters, oversampling, distortion primitives
│   ├── smoothie-math        # Fast DSP math, windows, interpolation, noise
│   ├── smoothie-fx          # Full effects library (reverb, delay, chorus, ...)
│   ├── smoothie-synth       # Polyphonic voice manager, wavetable, FM, additive
│   ├── smoothie-modulation  # Mod matrix, LFO, envelope generators
│   ├── smoothie-analysis    # FFT spectrum, LUFS, pitch detection, BPM
│   ├── smoothie-graph       # AudioProcessorGraph — node-based routing
│   ├── smoothie-io          # WAV/AIFF reader & writer, IR loader
│   ├── smoothie-midi        # MIDI parsing, note management, MPE
│   ├── smoothie-presets     # Preset serialization, banks, factory presets
│   ├── smoothie-ui          # egui-based GPU-rendered plugin UI
│   ├── smoothie-graphics    # wgpu GPU canvas, animation, shader system
│   ├── smoothie-licensing   # Hardware fingerprint, HMAC key validation, trial
│   ├── smoothie-network     # Update checker, preset sync, crash reporter
│   ├── smoothie-validator   # Plugin health checker with detailed reports
│   ├── smoothie-vst3        # VST3 format wrapper
│   ├── smoothie-clap        # CLAP format wrapper
│   ├── smoothie-au          # Audio Units wrapper (macOS)
│   ├── smoothie-aax         # AAX wrapper (Pro Tools)
│   └── smoothie-cli         # cargo-smoothie CLI tool
├── standalone/              # Tauri desktop app (React/TSX)
└── xtask/                   # Build automation (bundle, install, validate)
```

Each crate has a single responsibility and can be used independently.

---

## Quick Start

### Prerequisites

```bash
# Install Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the Smoothie CLI
cargo install --path crates/smoothie-cli

# Install xtask shortcut
cargo install --path xtask
```

### Create a new plugin

```bash
cargo smoothie new my-reverb --template effect
cd my-reverb
```

This scaffolds a full plugin project with:
- `Cargo.toml` pre-configured for VST3 + CLAP
- `src/lib.rs` with your plugin struct and parameter definitions
- `src/bin/standalone.rs` for running as a desktop app
- Default parameter smoothing, audio layout, and MIDI config

### Build and bundle

```bash
# Bundle all formats (VST3 + CLAP + Standalone)
cargo xtask bundle --release

# Bundle only VST3
cargo xtask bundle --vst3 --release

# Install to system plugin directories
cargo xtask install
```

### Minimal plugin example

```rust
use smoothie_core::prelude::*;

pub struct MyReverb {
    params: Arc<MyReverbParams>,
    reverb: smoothie_fx::FdnReverb,
}

#[derive(Params)]
pub struct MyReverbParams {
    #[id = "mix"]
    pub mix: FloatParam,

    #[id = "decay"]
    pub decay: FloatParam,
}

impl SmoothiePlugin for MyReverb {
    const NAME:    &'static str = "My Reverb";
    const VENDOR:  &'static str = "Your Name";
    const VERSION: &'static str = "1.0.0";
    const UID:     PluginUid = PluginUid::new(0x1234, 0x5678, 0x9ABC, 0xDEF0);

    fn audio_layouts() -> &'static [AudioLayout] {
        &[AudioLayout::stereo_in_stereo_out()]
    }

    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus {
        self.reverb.mix   = self.params.mix.smoothed.next();
        self.reverb.decay = self.params.decay.smoothed.next();

        for (in_l, in_r, out_l, out_r) in ctx.iter_stereo() {
            let (l, r) = self.reverb.process_stereo(*in_l, *in_r);
            *out_l = l;
            *out_r = r;
        }
        ProcessStatus::Normal
    }
}

// Export all formats in one line
smoothie_export!(MyReverb);
```

---

## Plugin Formats

### VST3

```rust
impl Vst3Plugin for MyPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"MyPlugin00000001";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Reverb];
}
```

The VST3 bundle is created automatically by `cargo xtask bundle --vst3`:

```
MyPlugin.vst3/
└── Contents/
    ├── MacOS/
    │   └── MyPlugin         ← universal binary (arm64 + x86_64)
    ├── Resources/
    └── Info.plist           ← generated automatically
```

### CLAP

```rust
impl ClapPlugin for MyPlugin {
    const CLAP_ID: &'static str = "com.yourname.my-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("My reverb plugin");
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Reverb,
        ClapFeature::Stereo,
    ];
}
```

CLAP supports polyphonic expression, per-note modulation, and non-destructive parameter automation — features unavailable in VST3.

### Standalone

The standalone binary uses [Tauri v2](https://tauri.app) and provides:
- System audio I/O via CPAL
- MIDI input via midir
- React/TSX UI with the full Seraphic design system
- Preset browser with drag-and-drop
- Plugin validator built in

Run standalone:
```bash
cargo run --bin my-plugin-standalone
```

---

## DSP Modules

### Biquad Filter (`smoothie-dsp`)

```rust
use smoothie_dsp::filters::{BiquadFilter, FilterType};

let mut filter = BiquadFilter::new();
filter.set_coefficients(FilterType::LowPass {
    freq: 1000.0,
    q: 0.707,
    sample_rate: 44100.0,
});

// In your process loop (zero allocation):
for sample in buffer.iter_mut() {
    *sample = filter.process(*sample);
}
```

Supported filter types:
- `LowPass`, `HighPass`, `BandPass`, `Notch`, `AllPass`
- `PeakingEq` (with gain_db)
- `LowShelf`, `HighShelf`

### Oversampling (`smoothie-dsp`)

```rust
use smoothie_dsp::oversampling::Oversample2x;

let mut os = Oversample2x::new(512); // max block size

os.process(&mut block, |upsampled| {
    // Process at 2× sample rate — alias-free distortion etc.
    for s in upsampled.iter_mut() {
        *s = wavefold(*s);
    }
});
```

---

## Effects Library

### FDN Reverb

8-line Feedback Delay Network using prime-number delay lengths and a Hadamard mixing matrix for maximum diffusion. True studio-quality reverb.

```rust
use smoothie_fx::FdnReverb;

let mut reverb = FdnReverb::new(sample_rate);
reverb.decay   = 0.85;  // tail length
reverb.damping = 0.3;   // high-frequency roll-off
reverb.mix     = 0.3;   // wet/dry

let (out_l, out_r) = reverb.process_stereo(in_l, in_r);
```

### Parametric EQ

8-band fully parametric EQ. Each band can be any type (Peak, Shelf, Pass, Notch...) and recalculates coefficients only when parameters change.

```rust
use smoothie_fx::ParametricEq;

let mut eq = ParametricEq::new(sample_rate);

// Boost 2kHz by 3dB with Q=2.0
eq.bands[4].set_freq(2000.0);
eq.bands[4].set_gain(3.0);
eq.bands[4].set_q(2.0);

let out = eq.process(sample);
```

### Compressor

RMS compressor with soft-knee, lookahead-ready ballistics, and auto-makeup gain.

```rust
use smoothie_fx::Compressor;

let mut comp = Compressor::new(sample_rate);
comp.threshold = -18.0;  // dBFS
comp.ratio     = 4.0;    // 4:1
comp.knee      = 6.0;    // soft knee width in dB
comp.makeup    = 3.0;    // makeup gain in dB
comp.set_attack_ms(5.0, sample_rate);
comp.set_release_ms(100.0, sample_rate);

let out = comp.process(sample);
let gr  = comp.gain_reduction(); // dB, for metering
```

### Brick-Wall Limiter

True lookahead limiter with configurable ceiling and release. Safe for any sample rate.

```rust
use smoothie_fx::BrickwallLimiter;

let mut limiter = BrickwallLimiter::new(sample_rate);
limiter.ceiling = -0.3;    // dBFS true peak ceiling
limiter.set_lookahead_ms(1.0);
limiter.set_release_ms(200.0);

let (out_l, out_r) = limiter.process(in_l, in_r);
```

---

## Synthesis Engine

`smoothie-synth` provides everything needed to build a polyphonic synthesizer:

```rust
use smoothie_synth::{VoiceManager, WavetableOscillator, FmEngine};

// Polyphonic voice manager (up to 64 voices)
let mut voices = VoiceManager::new(64);
voices.note_on(60, 100);   // MIDI note C4, velocity 100
voices.note_off(60);

// Wavetable oscillator with 2048-sample tables
let mut osc = WavetableOscillator::new(sample_rate);
osc.load_table(WaveTable::Saw);
osc.set_freq(440.0);
let sample = osc.next_sample();

// 4-operator FM engine
let mut fm = FmEngine::new(sample_rate);
fm.set_algorithm(FmAlgorithm::A); // 32 DX7-compatible algorithms
fm.set_operator_ratio(0, 1.0);
fm.set_operator_ratio(1, 2.0);
```

---

## Modulation System

`smoothie-modulation` provides a flexible mod matrix that connects any source to any destination.

```rust
use smoothie_modulation::{ModMatrix, ModSource, ModDest, Lfo, LfoShape};

let mut matrix = ModMatrix::new();
let mut lfo = Lfo::new(sample_rate);

lfo.shape = LfoShape::Sine;
lfo.rate  = 2.0;  // Hz
lfo.depth = 0.5;

// Route LFO → Filter Cutoff with amount 0.8
matrix.connect(ModSource::Lfo(0), ModDest::FilterCutoff, 0.8);

// Route MIDI velocity → VCA with amount 1.0
matrix.connect(ModSource::MidiVelocity, ModDest::Vca, 1.0);
```

Supported LFO shapes: Sine, Triangle, Square, Saw Up, Saw Down, Sample & Hold, Smooth Random

Supported mod sources: LFOs, Envelopes, MIDI CC, MIDI Velocity, Note pitch, Aftertouch, Macro knobs

---

## Parameter System

All parameters are **real-time safe** — they use atomic storage and per-sample smoothing with no locks.

```rust
use smoothie_params::{FloatParam, FloatRange, SmoothingStyle};

FloatParam::new(
    "Cutoff",             // display name
    1000.0,               // default value
    FloatRange::Skewed {
        min: 20.0,
        max: 20000.0,
        factor: 0.3,      // logarithmic skew
    },
)
.with_smoother(SmoothingStyle::Logarithmic(20.0))  // 20ms log smoothing
.with_unit(" Hz")
.with_value_to_string(|v| format!("{:.0} Hz", v))
```

### Smoothing Styles

| Style | Description | Use Case |
|-------|-------------|----------|
| `None` | No smoothing, instant | Switches, bypass |
| `Linear(ms)` | Linear ramp over N ms | Volume, pan |
| `Logarithmic(ms)` | Log ramp over N ms | Frequency, time |
| `Spring { stiffness, damping }` | Physical spring | Creative motion |

---

## UI & Graphics

`smoothie-ui` uses **egui** rendered with **wgpu** (Metal on macOS, DX12 on Windows, Vulkan on Linux) — no web technologies, pure GPU-rendered native UI.

```rust
use smoothie_ui::prelude::*;

pub fn build_ui(cx: &mut UiContext, params: &Arc<MyParams>) {
    cx.panel("Main", |ui| {
        ui.section("Filter", |ui| {
            ui.knob("Cutoff", &params.cutoff);
            ui.knob("Resonance", &params.resonance);
        });
        ui.section("Envelope", |ui| {
            ui.adsr(&params.attack, &params.decay, &params.sustain, &params.release);
        });
    });

    // GPU spectrum analyzer (zero-copy FFT display)
    ui.spectrum_analyzer(&cx.analysis_data);
}
```

### Design System

Smoothie Elite ships with a complete design system (`smoothie-graphics`):

```rust
use smoothie_graphics::theme::SeraphicTheme;

// Seraphic Orange dark theme (default)
cx.set_theme(SeraphicTheme::dark());

// Colours
// --bg:      #0e0e12  (near-black background)
// --panel:   #1c1c22  (panel background)
// --accent:  #ff781e  (Seraphic orange)
// --text:    #e8e8f0  (primary text)
// --subtle:  #6a6a7a  (secondary text)
```

---

## Preset System

`smoothie-presets` handles factory presets, user banks, and DAW recall.

```rust
use smoothie_presets::{PresetBank, Preset};

// Save current state
let preset = Preset::capture("My Init", &plugin);
bank.add(preset);
bank.save_to_file("my-presets.smoothie")?;

// Load a preset
let preset = bank.get("Vintage Room")?;
preset.apply_to(&mut plugin);
```

Presets are stored as **TOML** files — human-readable, diffable, and version-control friendly.

---

## MIDI System

`smoothie-midi` handles everything from basic note-on/off to full MPE (MIDI Polyphonic Expression).

```rust
use smoothie_midi::{MidiEvent, NoteTracker, MpeProcessor};

// Basic MIDI
match event {
    MidiEvent::NoteOn { channel, note, velocity } => { /* ... */ }
    MidiEvent::NoteOff { channel, note } => { /* ... */ }
    MidiEvent::PitchBend { channel, value } => { /* ... */ }
    MidiEvent::Cc { channel, controller, value } => { /* ... */ }
}

// MPE
let mut mpe = MpeProcessor::new();
mpe.process_event(event);
let slide  = mpe.get_slide(note);    // CC74 — timbre
let press  = mpe.get_pressure(note); // aftertouch
let glide  = mpe.get_pitch(note);    // pitch bend
```

---

## Licensing System

`smoothie-licensing` provides hardware-locked licensing with HMAC key validation and graceful trial mode.

```rust
use smoothie_licensing::{LicenseManager, LicenseStatus};

let mgr = LicenseManager::new("com.yourname.my-plugin");

match mgr.check() {
    LicenseStatus::Licensed { expires } => {
        // Full functionality
    }
    LicenseStatus::Trial { days_remaining } => {
        // Run in trial mode (you define the restrictions)
    }
    LicenseStatus::Expired | LicenseStatus::Invalid => {
        // Show activation UI
    }
}

// Activate with a license key
mgr.activate("SMTH-XXXX-XXXX-XXXX")?;
```

The hardware fingerprint combines CPU ID, machine UUID, and OS identifiers — no internet required for validation.

---

## CLI — cargo smoothie

Install once, use everywhere:

```bash
cargo install --path crates/smoothie-cli
```

```
USAGE:
    cargo smoothie <COMMAND>

COMMANDS:
    new         Scaffold a new plugin project
    build       Compile the plugin
    bundle      Bundle into VST3/CLAP/AU format
    validate    Run the plugin health checker
    standalone  Launch in standalone mode
    info        Show plugin metadata
    update      Check for framework updates
    docs        Open the documentation

EXAMPLES:
    cargo smoothie new my-chorus --template effect
    cargo smoothie bundle --vst3 --release
    cargo smoothie validate target/release/my-plugin.vst3
    cargo smoothie standalone
```

### Templates

| Template | Description |
|----------|-------------|
| `effect` | Stereo audio effect (reverb, delay, EQ...) |
| `instrument` | Polyphonic synthesizer |
| `analyzer` | Metering / analysis tool |
| `utility` | Gain, pan, routing utilities |
| `midi` | MIDI processor / arpeggiator |

---

## Build System — cargo xtask

```bash
# Bundle all formats in release mode
cargo xtask bundle --release

# Bundle only specific formats
cargo xtask bundle --vst3 --release
cargo xtask bundle --clap --release
cargo xtask bundle --standalone --release

# Install built bundles to system plugin directories
cargo xtask install

# Validate a plugin binary
cargo xtask validate

# Run all tests
cargo xtask test

# Run clippy on all crates
cargo xtask clippy

# Clean build artifacts
cargo xtask clean
```

### Installation paths

| Platform | VST3 | CLAP |
|----------|------|------|
| macOS    | `/Library/Audio/Plug-Ins/VST3/` | `/Library/Audio/Plug-Ins/CLAP/` |
| Windows  | `C:\Program Files\Common Files\VST3\` | `C:\Program Files\Common Files\CLAP\` |
| Linux    | `~/.vst3/` | `~/.clap/` |

---

## DAW Compatibility

| DAW | VST3 | CLAP | AU | Notes |
|-----|------|------|----|-------|
| Ableton Live 11+ | ✅ | ✅ | ✅ | Full automation |
| Logic Pro | ✅ | — | ✅ | AU preferred |
| Pro Tools | ✅ | — | — | AAX in v0.3 |
| FL Studio 21+ | ✅ | ✅ | — | Full support |
| Bitwig Studio | ✅ | ✅ | — | Best CLAP support |
| Reaper | ✅ | ✅ | ✅ | All formats |
| Cubase 12+ | ✅ | — | — | VST3 native |
| Studio One 6+ | ✅ | ✅ | — | Full support |
| Reason 12+ | ✅ | — | — | VST3 only |
| LMFL | ✅ | ✅ | — | Full support |
| Ardour 7+ | ✅ | ✅ | — | Full support |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Your Plugin Code                     │
│            (implements SmoothiePlugin trait)            │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│                   smoothie-core                         │
│   Plugin trait · AudioBuffer · ProcessContext           │
│   InitContext · PluginUid · FormatFlags                  │
└──────┬──────────┬──────────┬────────────────────────────┘
       │          │          │
┌──────▼───┐ ┌───▼────┐ ┌───▼────────────────────────────┐
│ smoothie │ │smoothie│ │         smoothie-params         │
│  -dsp    │ │  -fx   │ │   FloatParam · BoolParam        │
│ filters  │ │reverb  │ │   Smoothing · Automation        │
│ oversampl│ │chorus  │ └────────────────────────────────┘
│ distort  │ │delay.. │
└──────────┘ └────────┘
       │
┌──────▼──────────────────────────────────────────────────┐
│                   smoothie-math                         │
│  fast math · windows · interp · freq · dB · scales     │
│  stats · envelopes · phasors · noise · matrix           │
└─────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│              Format Wrappers (exported symbols)          │
│  smoothie-vst3   smoothie-clap   smoothie-au             │
│  nih_export_vst3!()  nih_export_clap!()                  │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│                    UI Layer                              │
│  smoothie-ui (egui)   smoothie-graphics (wgpu)           │
│  GPU-rendered · Metal/DX12/Vulkan · 60fps                │
└──────────────────────────────────────────────────────────┘
```

### Real-Time Safety Guarantees

Smoothie Elite enforces real-time safety at compile time where possible:

1. **No allocation in the audio thread** — All buffers are pre-allocated in `initialize()`. The `process()` function never calls `Box::new`, `Vec::push`, or any heap-allocating function.
2. **No blocking** — No mutexes in the hot path. Parameter updates use `atomic_float` and `crossbeam` lock-free queues.
3. **No syscalls** — No file I/O, no logging (in release), no OS calls in `process()`.
4. **Panic = abort** — The release profile sets `panic = "abort"` so panics never unwind across the C ABI boundary.

---

## Contributing

Contributions are welcome! Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) first.

```bash
# Clone and build
git clone https://github.com/Tehuti01/smoothie_elite.git
cd smoothie_elite
cargo build --workspace

# Run all tests
cargo xtask test

# Run clippy (zero warnings policy)
cargo xtask clippy

# Run a specific crate's tests
cargo test -p smoothie-math
cargo test -p smoothie-fx
```

### Areas looking for contributors
- `smoothie-au` — Audio Units v2/v3 wrapper (needs macOS developer)
- `smoothie-aax` — Pro Tools AAX format
- `smoothie-synth` — FM engine algorithms and wavetable editor
- `smoothie-analysis` — LUFS meter and BPM detector
- Documentation and examples

---

## License

Smoothie Elite is dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

You may use this software under either license at your option.

---

<p align="center">
  Built with ❤️ by <a href="https://seraphicsonic.com">Seraphic Sonic</a>
</p>
