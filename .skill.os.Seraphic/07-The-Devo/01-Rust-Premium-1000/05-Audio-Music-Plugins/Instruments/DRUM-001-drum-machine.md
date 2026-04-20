# SKILL DRUM-001: DRUM MACHINE

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        DRUM MACHINE
                     Sample Playback, Synthesis
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## DRUM MACHINE

```rust
pub struct DrumMachine {
    pub pads: Vec<Pad>,
    pub outputs: [f32; 2],
}

pub struct Pad {
    pub sample: Option<AudioSample>,
    pub envelope: ADSR,
    pub pitch: f32,
    pub pan: f32,
    pub volume: f32,
    pub one_shot: bool,
}

impl DrumMachine {
    pub fn new() -> Self {
        DrumMachine {
            pads: (0..16).map(|_| Pad::new()).collect(),
            outputs: [0.0, 0.0],
        }
    }
    
    pub fn trigger(&mut self, pad: usize, velocity: u8) {
        if let Some(ref mut p) = self.pads[pad].sample {
            p.play(velocity as f32 / 127.0);
            self.pads[pad].envelope.trigger_attack();
        }
    }
    
    pub fn process(&mut self) -> (f32, f32) {
        let mut output = [0.0f32; 2];
        
        for pad in &self.pads {
            if let Some(ref sample) = pad.sample {
                let sample_out = sample.process() * pad.volume;
                output[0] += sample_out * (1.0 - pan);
                output[1] += sample_out * pan;
            }
        }
        
        output[0] *= 0.5;
        output[1] *= 0.5;
        
        (output[0], output[1])
    }
}
```

---

*Skill DRUM-001 | Category: Drum Machine | Complexity: Expert*