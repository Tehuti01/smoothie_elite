import numpy as np
import scipy.fftpack

# 🌀 spectral_auditor.py v0.2.0 — Strophe 4 Resonance Auditor
# Analyzes the FFT of an audio signal to detect PHI-resonant harmonic distribution.

PHI = (1.0 + 5.0**0.5) / 2.0

def audit_spectral_alignment(signal, sample_rate=44100):
    print("🚀 INITIATING STROPHE 4: SPECTRAL RESONANCE AUDIT...")
    
    # Perform FFT
    fft_data = np.abs(scipy.fftpack.fft(signal))
    freqs = scipy.fftpack.fftfreq(len(signal), 1/sample_rate)
    
    # Find dominant peaks
    peak_indices = np.where(fft_data > np.max(fft_data) * 0.1)[0]
    peak_freqs = sorted(np.abs(freqs[peak_indices]))
    unique_peaks = []
    for f in peak_freqs:
        if not any(np.isclose(f, up, atol=10.0) for up in unique_peaks):
            unique_peaks.append(f)
            
    print(f"   Detected {len(unique_peaks)} dominant harmonic nodes.")
    
    # Check for PHI ratios between peaks
    phi_matches = 0
    for i in range(len(unique_peaks) - 1):
        ratio = unique_peaks[i+1] / unique_peaks[i]
        if np.isclose(ratio, PHI, atol=0.05) or np.isclose(ratio, PHI**2, atol=0.05):
            print(f"   ✓ RESONANT NODE DETECTED: {unique_peaks[i]:.2f}Hz -> {unique_peaks[i+1]:.2f}Hz (Ratio: {ratio:.4f})")
            phi_matches += 1

    if phi_matches > 0:
        print(f"✅ AUDIT SUCCESS: {phi_matches} PHI-resonant harmonic relationships confirmed.")
    else:
        print("⚠️  AUDIT WARNING: No PHI-resonant nodes found. Signal may be Obsidian-Era linear.")

if __name__ == "__main__":
    # Generate a dummy PHI-aligned signal for demo
    t = np.linspace(0, 1, 44100)
    f0 = 440.0
    signal = np.sin(2 * np.pi * f0 * t) + 0.5 * np.sin(2 * np.pi * f0 * PHI * t)
    audit_spectral_alignment(signal)
