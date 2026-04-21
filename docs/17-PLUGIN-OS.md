# Plugin-OS — Modular Node System

[← Validation & Testing](16-VALIDATOR-TESTING.md) | [Seraphic-Prime →](18-SERAPHIC-PRIME.md)

---

Plugin-OS is a modular node ecosystem within Smoothie Elite. Instead of writing DSP algorithms from scratch, you assemble plugins from pre-built, optimized nodes — similar to modular synthesizer patching.

---

## Architecture

```
Your Plugin
    │
    ├── Uses nodes from plugin-os-nodes-filter
    ├── Uses nodes from plugin-os-nodes-dyn
    ├── Uses nodes from plugin-os-ui-widgets-basic
    │
    └── Connected via plugin-os-graph (topology scheduler)
```

Each `plugin-os-*` crate is a library of 10–20 specialized nodes. Nodes implement the `Node` trait from `plugin-os-core`:

```rust
pub trait Node: Send + Sync {
    type Config;
    type Input;
    type Output;

    /// Initialize the node. Allocate all state here.
    fn new(config: Self::Config, sample_rate: f32) -> Self;

    /// Process a block of audio or control data.
    /// Must not allocate or block.
    fn process(&mut self, input: &Self::Input) -> Self::Output;

    /// Reset all internal state.
    fn reset(&mut self);
}
```

---

## DSP Nodes

### Filter Nodes — `plugin-os-nodes-filter`

| Node | Function | Description |
|---|---|---|
| `ZdfLadderFilter` | `process_sample(f32) -> f32` | Zero-delay feedback Moog ladder. Accurate resonance compensation. |
| `SvfFilterNode` | `process_svf(f32) -> (f32, f32, f32)` | State-variable filter. Simultaneous LP/BP/HP outputs. |
| `BiquadNode` | `process_sample(f32) -> f32` | Direct-form II transposed biquad. Fast, general-purpose. |
| `FirFilterNode` | `process_fir(f32, &[f32]) -> f32` | Linear-phase FIR with pre-computed windowed kernel. |
| `CombFilterNode` | `process_comb(f32) -> f32` | Feedback comb filter for reverb and special effects. |
| `AllpassNode` | `process_ap(f32) -> f32` | Allpass filter for phasing and reverb diffusion. |

```rust
use plugin_os_nodes_filter::{ZdfLadderFilter, FilterConfig};

let mut filter = ZdfLadderFilter::new(FilterConfig {
    frequency:  1000.0,
    resonance:  0.7,
}, sample_rate);

filter.set_frequency(2000.0, sample_rate);
filter.set_resonance(0.9);

let output = filter.process_sample(input);
```

---

### Oscillator Nodes — `plugin-os-nodes-osc` / `plugin-os-nodes-synth`

| Node | Function | Description |
|---|---|---|
| `WavetableOscNode` | `step(phase_inc: f32) -> f32` | Anti-aliased wavetable with Hermite interpolation |
| `PolyBlepNode` | `next_sample() -> f32` | PolyBLEP for Saw, Square, Triangle without wavetables |
| `FmCoreNode` | `modulate(carrier: f32, mod: f32) -> f32` | Phase modulation core |
| `NoiseNode` | `next_white() -> f32` | White + Pink + Brown noise generators |
| `SineNode` | `next_sample() -> f32` | Low-cost sine oscillator via Maclaurin |

```rust
use plugin_os_nodes_synth::WavetableOscNode;

let mut osc = WavetableOscNode::new(sample_rate);
osc.set_frequency(440.0);
osc.set_wavetable(Wavetable::SAW);

let sample = osc.step(osc.phase_increment());
```

---

### Dynamics Nodes — `plugin-os-nodes-dyn`

| Node | Function | Description |
|---|---|---|
| `VcaCompressorNode` | `compress(f32, f32) -> f32` | VCA-style compressor with log-domain gain |
| `SoftClipperNode` | `clip(f32) -> f32` | `tanh`-based soft saturation |
| `HardClipperNode` | `clip(f32) -> f32` | Hard `clamp(-1, 1)` clipper |
| `EnvelopeFollowerNode` | `follow(f32) -> f32` | Peak or RMS level tracking |
| `GateNode` | `gate(f32) -> f32` | Noise gate with hysteresis |

```rust
use plugin_os_nodes_dyn::{VcaCompressorNode, CompressorConfig};

let mut comp = VcaCompressorNode::new(CompressorConfig {
    threshold: -20.0,
    ratio:     4.0,
    attack:    0.005,
    release:   0.1,
}, sample_rate);

let output = comp.compress(input, envelope_input);
```

---

### Effects Nodes

**`plugin-os-nodes-fx-time`** — Delay, Reverb, Echo

```rust
use plugin_os_nodes_fx_time::DelayNode;

let mut delay = DelayNode::new(DelayConfig {
    max_delay_ms: 2000.0,
    feedback:     0.5,
}, sample_rate);

delay.set_delay_ms(250.0);
let output = delay.process(input);
```

**`plugin-os-nodes-fx-mod`** — Chorus, Phaser, Flanger

**`plugin-os-nodes-fx-dist`** — Saturation, Bitcrusher, Distortion

---

## UI Widgets

### Basic Widgets — `plugin-os-ui-widgets-basic`

All widgets are GPU-rendered via wgpu. Framerate is 144Hz-capable.

```rust
use plugin_os_ui_widgets_basic::{Knob, KnobConfig};

// In build_ui()
Knob::render(cx, &KnobConfig {
    label:    "Cutoff",
    param:    &params.cutoff,
    size:     48.0,
    color:    Color::GOLD,
    arc_bg:   Color::DARK_GRAY,
});
```

### Visualizer Widgets — `plugin-os-ui-widgets-visualizer`

```rust
use plugin_os_ui_widgets_visualizer::{SpectrumDisplay, Oscilloscope};

SpectrumDisplay::update(cx, spectrum_bins, SpectrumConfig {
    fft_size:  2048,
    min_db:   -80.0,
    max_db:    0.0,
    log_freq:  true,
});

Oscilloscope::render(cx, &waveform_buffer, OscilloscopeConfig {
    color:  Color::GREEN,
    height: 100.0,
});
```

### Meter Widgets — `plugin-os-ui-widgets-meter`

```rust
use plugin_os_ui_widgets_meter::VuMeter;

VuMeter::render(cx, &MeterConfig {
    level_l: plugin.level_l.load(Ordering::Relaxed),
    level_r: plugin.level_r.load(Ordering::Relaxed),
    peak_hold_ms: 2000.0,
    show_gr: true,
    gr_level: plugin.gain_reduction.load(Ordering::Relaxed),
});
```

---

## Connecting Nodes with the Graph

For complex signal routing, use `plugin-os-graph`:

```rust
use plugin_os_graph::{Graph, NodeId};

let mut graph = Graph::new(sample_rate);

// Add nodes to the graph
let osc_id    = graph.add_node(PolyBlepNode::new(sample_rate));
let filter_id = graph.add_node(ZdfLadderFilter::new(config, sample_rate));
let comp_id   = graph.add_node(VcaCompressorNode::new(comp_config, sample_rate));
let out_id    = graph.add_output();

// Connect: osc → filter → compressor → output
graph.connect(osc_id,    filter_id);
graph.connect(filter_id, comp_id);
graph.connect(comp_id,   out_id);

// Schedule: topological sort runs once, then is fixed
graph.compile();

// In process() — zero allocation
graph.process_block(audio_buffer);
```

---

## Plugin-OS Registry

Discover available nodes at runtime:

```rust
use plugin_os_registry::Registry;

let registry = Registry::global();

for node_info in registry.iter() {
    println!("{} — {} ({})", node_info.name, node_info.description, node_info.crate_path);
}

// Instantiate by name
let node = registry.create("ZdfLadderFilter", sample_rate)?;
```

---

*Next: [Seraphic-Prime →](18-SERAPHIC-PRIME.md)*
