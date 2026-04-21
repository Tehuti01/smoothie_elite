# Delay — Stereo Echo Effect

> A stereo delay plugin with feedback and dry/wet control.

This example demonstrates core concepts for time-based audio effects in Smoothie Elite.

## What It Demonstrates

- Multi-parameter plugin design (3 parameters)
- Using `RingBuffer` from `smoothie-core` for efficient delay lines
- Proper `tail_length()` reporting so the host doesn't cut off echoes early
- Sample-rate-aware delay time calculation
- Clamping feedback to prevent runaway oscillation

## Parameters

| Index | Name | Range | Default | Unit |
|---|---|---|---|---|
| 0 | Delay Time | 1 – 2000 | 250 | ms |
| 1 | Feedback | 0.0 – 0.95 | 0.40 | linear |
| 2 | Mix | 0.0 – 1.0 | 0.50 | linear |

## Building

```bash
cd examples/delay
cargo build
cargo test
```

## Code Walkthrough

### 1. `SmoothiePlugin::new()` — Initialization

```rust
fn new(sample_rate: f32) -> Self {
    let delay_samples = (0.250 * sample_rate) as usize; // 250ms default
    Self {
        buffer_l: RingBuffer::new(),
        buffer_r: RingBuffer::new(),
        delay_samples: delay_samples.min(4095), // ring buffer capacity limit
        feedback: 0.4,
        mix: 0.5,
        sample_rate,
    }
}
```

Two ring buffers — one per channel. All memory is allocated here, never in `process()`.

### 2. `process()` — The Delay Algorithm

```rust
let delayed_l = self.buffer_l.read(self.delay_samples);
self.buffer_l.write(dry_l + delayed_l * self.feedback);
buffer[0][i] = dry_l * (1.0 - self.mix) + delayed_l * self.mix;
```

Classic delay topology:
1. **Read** the delayed sample from the ring buffer
2. **Write** current input + feedback back into the ring buffer
3. **Output** = dry signal blended with delayed signal

### 3. `tail_length()` — Letting the host know about echoes

```rust
fn tail_length(&self) -> usize {
    if self.feedback > 0.01 {
        self.delay_samples * 8  // ~8 echo repetitions of audible tail
    } else {
        self.delay_samples
    }
}
```

Without this, some DAWs will zero out the output the moment you stop playing — cutting off the echoes.

### 4. `set_sample_rate()` — Maintaining correct delay time

```rust
fn set_sample_rate(&mut self, sr: f32) {
    // Recalculate samples to maintain the same ms time at new rate
    self.delay_samples = ((self.delay_samples as f32 / self.sample_rate) * sr) as usize;
    self.sample_rate = sr;
}
```

Always recalculate time-domain values when sample rate changes.

## What to Try Next

- Extend to a tempo-synced delay (divide `host_tempo` to get beat divisions in seconds)
- Add a filter in the feedback path for "dark" tape-style decay
- Implement ping-pong delay by crossing L→R and R→L in the feedback loop
