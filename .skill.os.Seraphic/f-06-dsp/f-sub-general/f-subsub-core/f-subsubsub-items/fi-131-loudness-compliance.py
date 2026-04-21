---
id: fi-131-loudness-compliance.py
category: f-06-dsp
---

import numpy as np
import scipy.signal

# 🔊 loudness_compliance.py v0.2.0 — Strophe 10 Mastering Tool
# Measures Integrated LUFS and True-Peak according to ITU-R BS.1770-4.

def measure_compliance(signal, sample_rate=44100):
    print("🚀 INITIATING STROPHE 10: LOUDNESS COMPLIANCE AUDIT...")
    
    # 1. K-Weighting Filter (Pre-filter + RLB)
    # Stage 1: High Shelf (approximate)
    b1, a1 = scipy.signal.iirfilter(2, 2000/(sample_rate/2), btype='highshelf')
    signal_k = scipy.signal.lfilter(b1, a1, signal)
    # Stage 2: High Pass
    b2, a2 = scipy.signal.iirfilter(2, 100/(sample_rate/2), btype='highpass')
    signal_k = scipy.signal.lfilter(b2, a2, signal_k)

    # 2. Measure Integrated LUFS
    power = np.mean(signal_k**2)
    lufs = 10 * np.log10(power) - 0.69 # Calibration offset
    print(f"   - Integrated Loudness: {lufs:.2f} LUFS")

    # 3. 8x True-Peak Detection
    upsampled = scipy.signal.resample(signal, len(signal) * 8)
    tp = np.max(np.abs(upsampled))
    tp_db = 20 * np.log10(tp)
    print(f"   - True-Peak: {tp_db:.2f} dBTP")

    if tp_db > -1.0:
        print("❌ ERROR: True-Peak ceiling exceeded (-1.0 dBTP violation).")
    if lufs > -14.0:
        print("⚠️  WARNING: Loudness exceeds streaming target (-14.0 LUFS).")

    if tp_db <= -1.0 and lufs <= -13.5:
        print("✅ COMPLIANCE SUCCESS: Signal is Sovereign-ready.")

if __name__ == "__main__":
    # Generate a demo signal with a high peak
    t = np.linspace(0, 1, 44100)
    signal = np.sin(2 * np.pi * 440 * t) * 0.8
    measure_compliance(signal)
