# Gain — Hello World Plugin

> The simplest possible Smoothie Elite audio plugin.

This example demonstrates:
- Implementing the `SmoothiePlugin` trait
- Single-parameter plugin (gain/volume)
- Smooth parameter transitions to avoid clicks
- Stereo processing

## Building

```bash
cargo build --release
```

## What it does

Applies a gain (volume) multiplier to the input signal. The gain parameter
smoothly interpolates to the target value over ~5ms to prevent audible clicks
when the user adjusts the knob.

## Code walkthrough

1. **`SmoothiePlugin::info()`** — Returns metadata (name, vendor, channel count)
2. **`SmoothiePlugin::new()`** — Initializes with unity gain and calculates smoothing coefficient
3. **`SmoothiePlugin::process()`** — Multiplies each sample by the smoothed gain value
4. **`set_param(0, value)`** — Sets the target gain (0.0–2.0)
