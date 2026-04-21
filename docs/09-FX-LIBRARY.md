# The Effects Library

The `smoothie-fx` crate provides full, production-ready audio effects built from the primitives in `smoothie-dsp`.

## 1. FDN Reverb (Feedback Delay Network)

A studio-grade, 8-line algorithmic reverb.
- **Architecture:** Uses 8 parallel delay lines with lengths set to mutually prime numbers to prevent metallic ringing.
- **Mixing:** The delay lines are fed back into each other using a Householder/Hadamard matrix, ensuring maximum acoustic diffusion.
- **Deterministic Tail (Phase XVII):** The reverb tail can be offloaded, allowing infinite sustain without stalling the CPU.

## 2. Dynamics Processing

### The Elite Compressor
An RMS-sensing, soft-knee compressor with auto-makeup gain.
- Uses smoothed envelope followers to prevent clicking on fast attack times.
- Features a visual Gain Reduction meter output for the UI.

### The Brickwall Limiter
A lookahead limiter designed for the master bus.
- Delays the audio signal slightly to "see" transients before they happen.
- Uses True-Peak detection to guarantee the signal never clips the DAC.

## 3. IronStack Saturation

The crown jewel of Smoothie Elite's tone generation.
- **Tube Overdrive:** Simulates asymmetric triode clipping using Chebyshev polynomials.
- **Tape Saturation:** Introduces high-frequency roll-off, magnetic hysteresis, and subtle Wow & Flutter pitch instability.
- **Bitcrusher:** Intentional digital degradation. Reduces sample rate (via sample-and-hold) and bit depth (with dither noise to prevent quantization harshness).

## 4. Modulation Effects

- **Stereo Chorus:** 4 independent voices modulated by out-of-phase LFOs.
- **Phaser:** Up to 12 cascaded Allpass filters sweeping across the spectrum.
- **Flanger:** Short modulated delay with extreme positive and negative feedback limits.
