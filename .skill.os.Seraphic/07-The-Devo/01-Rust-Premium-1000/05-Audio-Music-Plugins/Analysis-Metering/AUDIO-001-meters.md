# SKILL AUDIO-001: AUDIO METERS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        AUDIO METERS & ANALYSIS
                     FFT, Spectrum, Phase Scopes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## FFT ANALYZER

```rust
use std::f32::consts::PI;

pub struct FFTAnalyzer {
    pub size: usize,
    pub spectrum: Vec<f32>,
    pub window: Vec<f32>,
}

impl FFTAnalyzer {
    pub fn new(size: usize) -> Self {
        let window: Vec<f32> = (0..size)
            .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (size - 1) as f32).cos()))
            .collect();
        
        FFTAnalyzer { size, spectrum: vec![0.0; size / 2], window }
    }
    
    pub fn process(&mut self, samples: &[f32]) -> &[f32] {
        // Apply window
        let mut windowed: Vec<f32> = samples.iter()
            .zip(self.window.iter())
            .map(|(&s, &w)| s * w)
            .collect();
        
        // Zero pad
        windowed.resize(self.size, 0.0);
        
        // FFT
        self.fft_inplace(&mut windowed);
        
        // Magnitude
        for (i, chunk) in windowed.chunks(2).enumerate() {
            let mag = (chunk[0] * chunk[0] + chunk[1] * chunk[1]).sqrt();
            self.spectrum[i] = mag;
        }
        
        &self.spectrum
    }
}
```

---

*Skill AUDIO-001 | Category: Analysis | Complexity: Expert*