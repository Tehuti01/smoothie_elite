# The Parameter System

The `smoothie-params` crate handles all state that the user can manipulate. Audio thread parameters must be lock-free and heavily smoothed to avoid audio artifacts.

## 1. The Core Parameter Types

Smoothie Elite provides three core parameter types: `FloatParam`, `IntParam`, and `BoolParam`.

### FloatParam
The workhorse for 90% of plugin controls (Gain, Cutoff, Mix).
```rust
let cutoff = FloatParam::new("Cutoff", 1000.0)
    .with_range(FloatRange::Skewed { min: 20.0, max: 20000.0, factor: 0.3 })
    .with_smoother(SmoothingStyle::Logarithmic(20.0)) // 20ms log smoothing
    .with_unit(" Hz");
```

### IntParam
For discrete steps or modes.
```rust
let mode = IntParam::new("Mode", 0)
    .with_range(0, 3)
    .with_labels(&["Lowpass", "Highpass", "Bandpass", "Notch"]);
```

### BoolParam
For binary toggles (Bypass, Phase Invert).
```rust
let bypass = BoolParam::new("Bypass", false)
    .with_labels(["Active", "Bypassed"]);
```

## 2. Parameter Smoothing (Zipper Noise Prevention)

If a user snaps a knob from 0.0 to 1.0 instantly, it causes a discontinuity in the waveform (a click or "zipper noise"). The `Smoother` struct handles this.

Available styles:
- **`None`**: Instantaneous change (used for bypass or routing).
- **`Linear(ms)`**: Ramps the value linearly over `ms` milliseconds.
- **`Logarithmic(ms)`**: Ramps logarithmically. Excellent for frequencies and EQ gains.
- **`Spring { stiffness, damping }`**: Uses a physical spring model. When the user moves the knob, the internal DSP value "bounces" to the target.

## 3. Automation and Host Sync

When a DAW automates a parameter, it sends normalized values `(0.0 to 1.0)`. The parameter system automatically handles:
1. Denormalizing the value back to its true range (e.g., `0.5` -> `1000 Hz`).
2. Pushing the new value into the lock-free atomic storage.
3. Notifying the UI thread to redraw the knob position.
