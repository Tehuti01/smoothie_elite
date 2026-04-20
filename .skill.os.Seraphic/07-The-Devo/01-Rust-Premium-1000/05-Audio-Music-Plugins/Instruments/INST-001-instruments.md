# SKILL INST-001: INSTRUMENT PLUGINS

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        INSTRUMENT PLUGINS
                     Synthesizers, Samplers, Drum Machines
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## SIMPLE SYNTH

```rust
pub struct Synthesizer {
    pub oscillators: Vec<Oscillator>,
    pub envelope: ADSR,
    pub filter: BiquadFilter,
    pub master_volume: f32,
}

impl Synthesizer {
    pub fn new() -> Self {
        Synthesizer {
            oscillators: vec![Oscillator::new()],
            envelope: ADSR::new(),
            filter: BiquadFilter::new(),
            master_volume: 0.8,
        }
    }
    
    pub fn note_on(&mut self, note: u8, velocity: u8) {
        let freq = 440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0);
        
        for osc in &mut self.oscillators {
            osc.frequency = freq;
            osc.velocity = velocity as f32 / 127.0;
        }
        
        self.envelope.trigger_attack();
    }
    
    pub fn note_off(&mut self, _note: u8) {
        self.envelope.trigger_release();
    }
}
```

---

*Skill INST-001 | Category: Instruments | Complexity: Expert*