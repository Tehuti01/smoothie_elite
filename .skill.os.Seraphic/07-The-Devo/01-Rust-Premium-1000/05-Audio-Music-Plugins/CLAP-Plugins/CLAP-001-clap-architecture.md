# SKILL CLAP-001: CLAP PLUGIN ARCHITECTURE

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        CLAP PLUGIN ARCHITECTURE
                     CLAP Plugin Development
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## CLAP BASICS

```rust
use clap::{plugin::ClapPlugin, plugin::ProcessData, audio::AudioBuffer};

pub struct ClapSynth {
    sample_rate: f64,
    gain: f32,
}

impl ClapPlugin for ClapSynth {
    const NAME: &'static str = "ClapSynth";
    const VENDOR: &'static str = "Your Company";
    const VERSION: &'static str = "1.0.0";
    const DESCRIPTION: &'static str = "A simple synthesizer";
    const FEATURES: &'static [clap::plugin::Feature] = &[
        clap::plugin::Feature::Synthesizer,
    ];
    
    fn init(&mut self) -> Result<(), clap::error::Error> {
        Ok(())
    }
    
    fn activate(&mut self, sample_rate: f64, _: usize, _: usize) -> Result<(), clap::error::Error> {
        self.sample_rate = sample_rate;
        Ok(())
    }
    
    fn process(&mut self, data: &mut ProcessData) {
        let output = data.audio_output.get_mut(0);
        
        for (i, frame) in output.frames_mut().enumerate() {
            frame[0] = self.gain * self.last_sample;
            
            // Generate sine wave
            self.phase += self.frequency / self.sample_rate;
            if self.phase >= 1.0 { self.phase -= 1.0; }
            self.last_sample = (self.phase * 2.0 * std::f32::consts::PI).sin();
        }
    }
}
```

---

*Skill CLAP-001 | Category: CLAP | Complexity: Expert*