# EQ — 3-Band Parametric Equalizer

> A 3-band parametric EQ with low shelf, peak, and high shelf filters.

This example demonstrates chained filter topologies and frequency-domain shaping in Smoothie Elite.

## What It Demonstrates

- Chaining multiple `BiquadFilter` instances in series
- Implementing a low shelf, peak/bell, and high shelf filter
- Recalculating filter coefficients on sample rate change
- Multi-parameter plugin with logical parameter grouping

## Parameters

| Index | Name | Range | Default | Unit |
|---|---|---|---|---|
| 0 | Low Gain | -18 – +18 | 0 | dB |
| 1 | Low Freq | 20 – 500 | 100 | Hz |
| 2 | Mid Gain | -18 – +18 | 0 | dB |
| 3 | Mid Freq | 200 – 8000 | 1000 | Hz |
| 4 | Mid Q | 0.1 – 10 | 0.707 | — |
| 5 | High Gain | -18 – +18 | 0 | dB |
| 6 | High Freq | 2000 – 20000 | 8000 | Hz |

## Building

```bash
cd examples/eq
cargo build
cargo test
```

## Filter Theory

### Biquad Filters

Every band in this EQ uses a **Biquad filter** — a second-order IIR filter that implements the transfer function:

```
H(z) = (b0 + b1*z⁻¹ + b2*z⁻²) / (1 + a1*z⁻¹ + a2*z⁻²)
```

The `b` coefficients are the numerator (zeros), the `a` coefficients are the denominator (poles). Different arrangements of these 5 coefficients produce different filter shapes.

### Filter Types

| Type | Shape | Use |
|---|---|---|
| Low Shelf | Boost/cut below frequency | Warm up bass, thin out low end |
| Peak/Bell | Boost/cut at frequency | Surgical cuts, presence boosts |
| High Shelf | Boost/cut above frequency | Add air, remove harshness |

## Code Walkthrough

### 1. Three biquad filters in series

```rust
pub struct EqPlugin {
    low_shelf:  BiquadFilter,
    peak_mid:   BiquadFilter,
    high_shelf: BiquadFilter,
    // ... parameters
}
```

The signal flows: `input → low_shelf → peak_mid → high_shelf → output`.

### 2. Processing in series (zero allocation)

```rust
fn process(&mut self, buffer: &mut [&mut [f32]]) -> ProcessStatus {
    for channel in buffer.iter_mut() {
        for sample in channel.iter_mut() {
            let s = self.low_shelf.process(*sample);
            let s = self.peak_mid.process(s);
            *sample = self.high_shelf.process(s);
        }
    }
    ProcessStatus::Ok
}
```

Three function calls per sample. No branches, no allocation. The compiler typically inlines all three.

### 3. Coefficient recalculation on parameter change

```rust
fn set_param(&mut self, index: usize, value: f32) {
    match index {
        0 => { self.low_gain = value; self.update_low_shelf(); }
        1 => { self.low_freq = value; self.update_low_shelf(); }
        // ...
    }
}

fn update_low_shelf(&mut self) {
    self.low_shelf = BiquadFilter::low_shelf(self.low_freq, self.low_gain, self.sample_rate);
}
```

Coefficients are recomputed when parameters change, not every sample. This keeps `process()` allocation-free.

## What to Try Next

- Extend to a 7-band or 10-band graphic EQ using multiple peak filters at fixed frequencies
- Add a bypassing mechanism per band using a `BoolParam`
- Implement a dynamic EQ: use a sidechain envelope to modulate the gain of a band
- Draw the frequency response curve by evaluating `H(e^jω)` across the audio spectrum
