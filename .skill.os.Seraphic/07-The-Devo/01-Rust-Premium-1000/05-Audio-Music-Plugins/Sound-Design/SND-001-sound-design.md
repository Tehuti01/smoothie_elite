# SKILL SND-001: SOUND DESIGN PRINCIPLES

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        SOUND DESIGN PRINCIPLES
                     Audio Design Theory
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## SOUND DESIGN BASICS

```rust
/// Sound Design Elements
pub struct SoundDesign {
    pub oscillators: Vec<OscillatorType>,
    pub modifiers: Vec<Modifier>,
    pub envelope: ADSR,
    pub filter: FilterSettings,
}

pub struct ADSR {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl ADSR {
    pub fn process(&self, time: f32, state: EnvelopeState) -> f32 {
        match state {
            EnvelopeState::Attack => {
                if time < self.attack {
                    time / self.attack
                } else {
                    1.0
                }
            }
            EnvelopeState::Decay => {
                let t = (time - self.attack) / self.decay;
                1.0 - (1.0 - self.sustain) * t
            }
            EnvelopeState::Sustain => self.sustain,
            EnvelopeState::Release => {
                let t = time / self.release;
                self.sustain * (1.0 - t)
            }
        }
    }
}
```

---

*Skill SND-001 | Category: Sound Design | Complexity: Expert*