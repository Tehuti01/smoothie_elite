# Technical Reference

[← Seraphic-Prime](18-SERAPHIC-PRIME.md) | [Crate Map →](CRATE_MAP.md)

---

This reference provides a condensed API summary for the most commonly used types and functions across the framework. For full documentation, run `cargo doc --workspace --open`.

---

## Core Traits

### `SmoothiePlugin`

```rust
pub trait SmoothiePlugin: Send + Sync {
    fn info() -> PluginInfo where Self: Sized;
    fn new(sample_rate: f32) -> Self where Self: Sized;
    fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus;
    fn set_sample_rate(&mut self, sr: f32);
    fn reset(&mut self);

    // Optional — default implementations provided
    fn param_count(&self) -> usize { 0 }
    fn get_param(&self, index: usize) -> f32 { 0.0 }
    fn set_param(&mut self, index: usize, value: f32) {}
    fn get_param_name(&self, index: usize) -> &'static str { "" }
    fn tail_length(&self) -> usize { 0 }
    fn latency(&self) -> usize { 0 }
}
```

### `AudioProcessor`

```rust
pub trait AudioProcessor: Send + Sync {
    fn process(&mut self, input: f32) -> f32;
    fn process_stereo(&mut self, l: f32, r: f32) -> (f32, f32);
    fn reset(&mut self);
    fn set_sample_rate(&mut self, sr: f32);
}
```

---

## Math Functions — `smoothie_core::math`

| Function | Signature | Description |
|---|---|---|
| `fast_sin` | `(x: f32) -> f32` | Sine approximation, < 0.1% error |
| `fast_cos` | `(x: f32) -> f32` | Cosine approximation |
| `fast_tanh` | `(x: f32) -> f32` | Hyperbolic tangent, for saturation |
| `exp_approx` | `(x: f32) -> f32` | e^x approximation |
| `db_to_linear` | `(db: f32) -> f32` | 10^(db/20) |
| `linear_to_db` | `(linear: f32) -> f32` | 20 × log10(linear) |
| `amplitude_to_db` | `(amp: f32) -> f32` | Same as `linear_to_db` |
| `midi_to_hz` | `(note: u8) -> f32` | 440 × 2^((note-69)/12) |
| `hz_to_midi` | `(hz: f32) -> f32` | 69 + 12 × log2(hz/440) |
| `lerp` | `(a: f32, b: f32, t: f32) -> f32` | Linear interpolation |
| `clamp` | `(x: f32, lo: f32, hi: f32) -> f32` | Clamp to [lo, hi] |

---

## Filter Quick Reference — `smoothie_dsp::filters`

### `BiquadFilter` constructors

```rust
BiquadFilter::low_pass(freq: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::high_pass(freq: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::band_pass(freq: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::notch(freq: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::all_pass(freq: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::peaking(freq: f32, gain_db: f32, q: f32, sr: f32) -> BiquadFilter
BiquadFilter::low_shelf(freq: f32, gain_db: f32, sr: f32) -> BiquadFilter
BiquadFilter::high_shelf(freq: f32, gain_db: f32, sr: f32) -> BiquadFilter
```

### `BiquadFilter` methods

```rust
filter.process(input: f32) -> f32
filter.set_frequency(freq: f32, sr: f32)
filter.set_q(q: f32)
filter.set_gain_db(db: f32)
filter.reset()
```

### `StateVariableFilter`

```rust
let mut svf = StateVariableFilter::new(freq, q, sr);
let (lp, bp, hp) = svf.process_svf(input);
svf.set_frequency(freq, sr);
svf.set_resonance(q);
```

### `LadderFilter`

```rust
let mut ladder = LadderFilter::new(sr);
ladder.set_frequency(freq, sr);     // 20 – 20000 Hz
ladder.set_resonance(0.0..=1.0);    // 1.0 = self-oscillation
let output = ladder.process(input);
```

---

## Effect Quick Reference — `smoothie_effects`

```rust
// Reverb
let mut rv = ReverbEffect::new(sr);
rv.set_room_size(0.0..=1.0);
rv.set_damping(0.0..=1.0);
rv.set_mix(0.0..=1.0);
let out = rv.process(input);

// Compressor
let mut comp = Compressor::new(sr);
comp.set_threshold(db);     // typically -40 to 0
comp.set_ratio(ratio);      // 1.0 to 100.0
comp.set_attack(secs);      // 0.0001 to 1.0
comp.set_release(secs);     // 0.001 to 3.0
comp.set_makeup_gain(db);
let out = comp.process(input);

// Saturator
let mut sat = Saturator::new();
sat.set_type(SaturationType::Tube);
sat.set_drive(linear_gain);
sat.set_mix(0.0..=1.0);
let out = sat.process(input);
```

---

## Oscillator Quick Reference — `smoothie_dsp::oscillators`

```rust
// Basic oscillator
let mut osc = Oscillator::new(sr);
osc.set_frequency(hz);
osc.set_mode(OscillatorMode::Sine); // Sine, Square, Saw, Triangle, WhiteNoise
let sample = osc.process(0.0);

// Wavetable oscillator
let mut wt = WavetableOscillator::new(sr);
wt.set_frequency(hz);
wt.set_wavetable(custom_table);
let sample = wt.next_sample();
```

---

## Envelope Quick Reference — `smoothie_dsp::envelope_mod`

```rust
// ADSR
let mut env = AdsrEnvelope::new(sr);
env.set_attack(0.01);    // seconds
env.set_decay(0.1);
env.set_sustain(0.7);    // 0.0–1.0 level
env.set_release(0.3);
env.trigger();           // Note On
env.release();           // Note Off
let amp = env.process(); // call per sample → 0.0–1.0

// LFO
let mut lfo = Lfo::new(sr);
lfo.set_rate(1.0);       // Hz
lfo.set_shape(LfoShape::Sine);
lfo.set_depth(0.5);
let mod_val = lfo.process(); // call per sample → -depth..+depth
```

---

## Plugin-OS Filter Node Reference

### `plugin-os-nodes-filter`

| Node | Key method | Algorithm |
|---|---|---|
| `ZdfLadderFilter` | `process_sample(f32) -> f32` | ZDF with resonance compensation |
| `SvfFilterNode` | `process_svf(f32) -> SvfOut` | Trapezoidal SVF topology |
| `BiquadNode` | `process_sample(f32) -> f32` | DF-II transposed |
| `FirFilterNode` | `process_fir(f32, &[f32]) -> f32` | Windowed-sinc FIR |
| `CombFilterNode` | `process_comb(f32) -> f32` | Delay-line comb |
| `AllpassNode` | `process_ap(f32) -> f32` | First/second-order allpass |
| `MoogLadder4pole` | `process(f32) -> f32` | Moog ladder (4-pole, 24dB/oct) |

### `plugin-os-nodes-dyn`

| Node | Key method | Algorithm |
|---|---|---|
| `VcaCompressorNode` | `compress(f32, env: f32) -> f32` | Log-domain VCA |
| `SoftClipperNode` | `clip(f32) -> f32` | `tanh(x)` soft saturation |
| `HardClipperNode` | `clip(f32) -> f32` | `clamp(-1, 1)` |
| `EnvelopeFollowerNode` | `follow(f32) -> f32` | Peak + RMS |
| `GateNode` | `gate(f32) -> f32` | Hysteresis gate |
| `LimiterNode` | `limit(f32) -> f32` | 50:1 compressor |

---

## Common Patterns

### Pattern: Smoothed parameter in process()

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    for sample in buffer[0].iter_mut() {
        // Read one smoothed value per sample — never read raw AtomicF32 per sample
        let gain = self.params.gain.smoothed.next();
        *sample *= gain;
    }
    ProcessStatus::Ok
}
```

### Pattern: Filter with per-block coefficient update

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    // Update filter once per block (not per sample) — much cheaper
    let cutoff = self.params.cutoff.smoothed.next_block();
    self.filter.set_frequency(cutoff, self.sample_rate);

    for sample in buffer[0].iter_mut() {
        *sample = self.filter.process(*sample);
    }
    ProcessStatus::Ok
}
```

### Pattern: Stereo processing with mid/side

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    let n = buffer[0].len();
    for i in 0..n {
        let l = buffer[0][i];
        let r = buffer[1][i];

        // M/S encode
        let mid  = (l + r) * 0.5;
        let side = (l - r) * 0.5;

        // Process mid and side independently
        let mid_out  = self.mid_processor.process(mid);
        let side_out = self.side_processor.process(side);

        // M/S decode
        buffer[0][i] = mid_out + side_out;
        buffer[1][i] = mid_out - side_out;
    }
    ProcessStatus::Ok
}
```

### Pattern: Instrument with MIDI events

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    // Handle MIDI events first — before generating audio
    for event in self.pending_midi.drain(..) {
        match event {
            MidiEvent::NoteOn { note, velocity } => {
                self.oscillator.set_frequency(midi_to_hz(note));
                self.envelope.trigger();
            }
            MidiEvent::NoteOff { .. } => {
                self.envelope.release();
            }
            _ => {}
        }
    }

    // Generate audio
    for sample in buffer[0].iter_mut() {
        let osc = self.oscillator.process(0.0);
        let env = self.envelope.process();
        *sample = osc * env;
    }
    buffer[1].copy_from_slice(buffer[0]);  // mono → stereo

    ProcessStatus::Ok
}
```

---

*See also: [Crate Map](CRATE_MAP.md) · `cargo doc --workspace --open`*
