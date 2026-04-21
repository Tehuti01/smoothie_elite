# The Core Crates

The `smoothie-core` crate is the absolute foundation of the framework. It defines the traits and types that all plugins must implement, regardless of whether they are compiled as VST3, CLAP, or Standalone.

## 1. The `SmoothiePlugin` Trait

Every plugin must implement this trait. It defines the plugin's identity, lifecycle, and DSP loop.

```rust
pub trait SmoothiePlugin: Default + Send + Sync {
    // Identity
    const NAME: &'static str;
    const VENDOR: &'static str;
    const VERSION: &'static str;
    const UID: PluginUid; // A unique 4-byte identifier

    // Audio Layouts
    fn audio_layouts() -> &'static [AudioLayout];

    // Parameter Registry
    fn parameters(&self) -> Vec<Arc<dyn Param>>;

    // Lifecycle
    fn initialize(&mut self, ctx: &InitContext) -> bool { true }
    fn reset(&mut self) {}
    
    // The Hot Path
    fn process(&mut self, ctx: &mut ProcessContext) -> ProcessStatus;

    // State Management
    fn save_state(&self) -> Vec<u8> { vec![] }
    fn load_state(&mut self, state: &[u8]) {}
}
```

## 2. ProcessContext

The `ProcessContext` is passed to the `process` function on every audio block. It provides zero-allocation access to the audio buffers and temporal data.

```rust
pub struct ProcessContext<'a> {
    pub sample_rate: f64,
    pub block_size: usize,
    
    // Transport
    pub tempo: f64,          // BPM
    pub playing: bool,
    pub timeline_pos: f64,   // Position in beats
    
    // Buffers (Interleaved or Deinterleaved)
    pub audio: AudioBuffer<'a>,
    
    // MIDI
    pub midi_events: &'a [MidiEvent],
}
```

## 3. AudioBuffer

`AudioBuffer` is a smart wrapper around raw `&mut [f32]` pointers provided by the DAW. It provides safe, bounds-checked iterators for mono, stereo, and surround processing.

```rust
// Iterate over stereo frames
for (l, r) in ctx.audio.iter_stereo_mut() {
    *l = dsp_process_left(*l);
    *r = dsp_process_right(*r);
}
```

## 4. FormatFlags & Layouts

Plugins must explicitly state their supported formats and channel configurations to ensure DAW compatibility.

```rust
fn audio_layouts() -> &'static [AudioLayout] {
    &[
        AudioLayout::mono(),
        AudioLayout::stereo(),
        AudioLayout::stereo_in_surround_out(SurroundFormat::FivePointOne),
    ]
}
```
