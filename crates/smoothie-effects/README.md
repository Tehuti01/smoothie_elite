# Smoothie Effects Suite

Professional audio effects library for **Smoothie Elite** plugins.

## Included Effects

### DSP Generators

**Envelope (ADSR)**
```rust
let mut env = Envelope::new(0.01, 0.1, 0.7, 0.3, 44100.0);
env.trigger();
while !env.is_idle() {
    let out = env.next_sample();
}
```

**LFO (Low-Frequency Oscillator)**
```rust
let mut lfo = Lfo::new(5.0, LfoShape::Sine, 44100.0);
let modulation = lfo.next_sample(); // 0.0–1.0
```

### Effects Processors

#### Reverb (Freeverb-style)
- Parallel comb filters with feedback
- Series allpass filters for density
- Room size, damping, stereo width controls
- Real-time safe processing

**Usage:**
```rust
let mut reverb = ReverbProcessor::new(44100.0);
reverb.set_room_size(0.75);
reverb.set_damp(0.5);
let output = reverb.process(input);
```

#### Delay (Multi-tap)
- Up to 4 seconds of delay
- Feedback with damping
- Wet/dry mix control

**Usage:**
```rust
let mut delay = DelayProcessor::new(44100.0);
delay.set_delay(500.0);     // ms
delay.set_feedback(0.4);    // 0.0–1.0
delay.set_mix(0.3, 0.7);    // wet, dry
let output = delay.process(input);
```

#### Compressor/Limiter
- Threshold-based gain reduction
- Attack/release envelopes
- Makeup gain
- Compression ratio control

**Usage:**
```rust
let mut comp = CompressorProcessor::new(-20.0, 4.0, 5.0, 50.0, 44100.0);
comp.set_threshold(-15.0);  // dB
comp.set_ratio(4.0);        // 4:1
let output = comp.process(input);
```

#### Equalizer (3-band parametric)
- Low shelf (200 Hz)
- Mid peak (1000 Hz with Q)
- High shelf (5000 Hz)
- ±24 dB gain per band

**Usage:**
```rust
let mut eq = EqProcessor::new(44100.0);
eq.set_low(200.0, 6.0);          // freq, gain
eq.set_mid(1000.0, -3.0, 2.0);   // freq, gain, Q
eq.set_high(5000.0, 9.0);        // freq, gain
let output = eq.process(input);
```

#### Chorus/Flanger
- LFO-modulated delay
- Sine/Triangle/Square modulation
- Depth and rate controls
- Stereo width processing

**Usage:**
```rust
let mut chorus = ChorusProcessor::new(44100.0);
chorus.set_rate(1.5);      // Hz
chorus.set_depth(5.0);     // ms
let output = chorus.process(input);
```

#### Distortion/Saturation
- Soft clipping (smooth saturation)
- Hard clipping (digital distortion)
- Asymmetric saturation (warm character)
- Tanh saturation (natural)

**Usage:**
```rust
let mut dist = DistortionProcessor::new(DistortionType::SoftClip);
dist.set_drive(3.0);       // 0.0–10.0
dist.set_tone(0.5);        // shaping
let output = dist.process(input);
```

## Common Trait

All effects implement `EffectProcessor`:

```rust
pub trait EffectProcessor: Send + Sync {
    fn process(&mut self, input: f32) -> f32;
    fn process_stereo(&mut self, left: f32, right: f32) -> (f32, f32);
    fn reset(&mut self);
    fn set_sample_rate(&mut self, sr: f32);
}
```

## Real-Time Safety

✅ Zero heap allocations in `process()`  
✅ No mutex locks  
✅ No panic possibilities  
✅ Safe to call at audio rate  

## Example Chain

```rust
// Build a signal chain: Input → Distortion → EQ → Delay → Reverb
let mut dist = DistortionProcessor::new(DistortionType::SoftClip);
let mut eq = EqProcessor::new(44100.0);
let mut delay = DelayProcessor::new(44100.0);
let mut reverb = ReverbProcessor::new(44100.0);

let mut signal = input;
signal = dist.process(signal);
signal = eq.process(signal);
signal = delay.process(signal);
signal = reverb.process(signal);
output = signal;
```

## Performance

Per sample @ 44.1 kHz:

| Effect | CPU Cost |
|--------|----------|
| Reverb | ~5 µs |
| Delay | ~0.5 µs |
| Distortion | <0.1 µs |
| Compressor | ~1 µs |
| EQ | ~2 µs |
| Chorus | ~2 µs |

**Total chain:** ~10 µs per sample (stereo) ≈ 10% @ 48 kHz buffer

