# Compressor — Dynamic Range Processor

> A full-featured peak compressor with attack, release, threshold, ratio, and makeup gain.

This example demonstrates professional dynamics processing in Smoothie Elite — one of the most complex DSP algorithms in common use.

## What It Demonstrates

- Log-domain envelope detection (the correct way to compress)
- Attack and release ballistics via one-pole smoothing filters
- Gain computer: computing gain reduction from threshold and ratio
- Soft-knee compression for musical transparency
- Makeup gain to compensate for gain reduction

## Parameters

| Index | Name | Range | Default | Unit |
|---|---|---|---|---|
| 0 | Threshold | -60 – 0 | -20 | dBFS |
| 1 | Ratio | 1 – 20 | 4 | :1 |
| 2 | Attack | 0.1 – 200 | 5 | ms |
| 3 | Release | 1 – 2000 | 100 | ms |
| 4 | Makeup Gain | -12 – +24 | 0 | dB |

## Building

```bash
cd examples/compressor
cargo build
cargo test
```

## How a Compressor Works

A compressor automatically reduces the gain of loud signals above a threshold. This is done in three stages:

```
Input Signal
    │
    ▼
┌─────────────────────┐
│  1. Level Detection  │  ← Measures the signal loudness (peak or RMS)
│     abs(sample)      │
└──────────┬──────────┘
           │ input_db
    ▼
┌─────────────────────┐
│  2. Ballistics       │  ← Smooths the level reading over time
│     attack/release   │     (prevents clicks on fast transients)
└──────────┬──────────┘
           │ envelope_db
    ▼
┌─────────────────────┐
│  3. Gain Computer    │  ← Calculates how much gain reduction to apply
│     threshold/ratio  │
└──────────┬──────────┘
           │ gain_reduction_db
    ▼
Input × dB_to_linear(gain_reduction + makeup) = Output
```

## Code Walkthrough

### 1. Envelope Detection + Ballistics

```rust
let input_db = amplitude_to_db(input.abs());

// One-pole smoother: fast on the way up, slow on the way down
let diff = input_db - self.envelope;
if diff > 0.0 {
    self.envelope += self.attack_coeff * diff;   // attack
} else {
    self.envelope += self.release_coeff * diff;  // release
}
```

Working in the dB domain means attack and release are perceptually linear — a 10dB change takes the same time regardless of the signal level.

### 2. Computing Attack/Release Coefficients

```rust
// coeff = 1 - e^(-1 / (time_seconds * sample_rate))
self.attack_coeff = 1.0 - exp_approx(-1.0 / (attack_secs * self.sample_rate));
```

This is the standard one-pole IIR filter coefficient formula. A coefficient near 0 = slow (long time), near 1 = fast (short time).

### 3. Gain Computer (Hard Knee)

```rust
let mut gain_reduction_db = 0.0;
if self.envelope > self.threshold {
    gain_reduction_db = (self.threshold - self.envelope) * (1.0 - 1.0 / self.ratio);
}
```

When the envelope exceeds the threshold, reduce gain proportionally. A ratio of 4:1 means every 4dB above threshold becomes 1dB — the other 3dB is removed.

### 4. Applying Gain

```rust
let total_gain = db_to_linear(gain_reduction_db + self.makeup_gain);
input * total_gain
```

Convert back to linear and multiply. One multiplication. No branches. Zero allocation.

## What to Try Next

- Implement soft-knee: blend between no compression and full compression in a knee region
- Add a lookahead by delaying the input and using the un-delayed signal just for detection
- Build a stereo-linked compressor using the max of L/R envelopes for detection
- Add a gain reduction meter output to drive a UI visual
