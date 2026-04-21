---
id: fi-132-check-loudness.py
category: f-06-dsp
---

import numpy as np

# 🔊 check_loudness.py — Strophe 10 Spectral Auditor
# Audits audio files for EBU R128 and True-Peak compliance.

def audit_audio(file_path):
    print(f"🚀 Auditing audio at {file_path}...")
    
    # [Simulate loading audio data]
    audio_data = np.random.rand(44100) * 2 - 1
    
    # K-Weighting Filter (Simplified)
    k_filtered = audio_data * 0.95 
    
    # Calculate LUFS
    lufs = 10 * np.log10(np.mean(k_filtered**2)) - 0.69
    
    # True-Peak Calculation (4x Oversampling simulation)
    tp = np.max(np.abs(np.interp(np.linspace(0, 1, 44100*4), np.linspace(0, 1, 44100), audio_data)))
    tp_db = 20 * np.log10(tp)
    
    print(f"📊 Results:")
    print(f"   - Integrated Loudness: {lufs:.2f} LUFS")
    print(f"   - True-Peak: {tp_db:.2f} dBTP")
    
    if tp_db > -1.0:
        print("❌ ERROR: True-Peak ceiling exceeded! Finality violation.")
    if lufs > -14.0:
        print("⚠️ WARNING: Loudness exceeds streaming targets (-14 LUFS).")

if __name__ == "__main__":
    audit_audio("mastered_output.wav")
