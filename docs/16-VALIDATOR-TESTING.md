# Validation & Testing

Smoothie Elite guarantees stability through the `smoothie-validator` crate and aggressive CI/CD testing.

## 1. The Plugin Validator (`cargo smoothie validate`)

The validator acts as a headless, chaotic DAW. It loads your compiled plugin binary (`.vst3` or `.clap`) and subjects it to extreme stress tests.

### Tests Performed:
- **Parameter Fuzzing:** Rapidly changes parameters from minimum to maximum values simultaneously to trigger zipper noise or thread deadlocks.
- **Chaotic Block Sizes:** Most DAWs process audio in blocks of 128 or 256. The validator will send blocks of 1, 3, 17, and 4096 samples to ensure the plugin's internal buffers do not panic.
- **NaN Injection:** Feeds `NaN` (Not a Number) and `Infinity` into the audio inputs to ensure the DSP filters do not explode into permanent feedback loops.
- **Denormal Flushing:** Checks if the plugin correctly flushes denormal floats to zero (preventing CPU spikes).

## 2. Audio Thread Assertions

In debug mode, the framework asserts that no heap allocations occur in the `process()` loop.

```bash
cargo test
```
The test suite ensures that every module in `smoothie-dsp`, `smoothie-fx`, and `smoothie-core` is deterministic, allocation-free, and real-time safe.

## 3. Continuous Integration

The framework provides a default GitHub Actions workflow (`ci.yml`) that automatically:
1. Builds all formats across macOS, Windows, and Linux.
2. Runs the test suite.
3. Runs `cargo clippy` with `#[deny(warnings)]`.
4. Executes the `smoothie-validator` on the final release binaries.
