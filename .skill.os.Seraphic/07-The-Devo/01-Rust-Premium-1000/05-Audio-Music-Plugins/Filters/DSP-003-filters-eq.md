# SKILL DSP-003: FILTERS & EQUALIZATION

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        FILTERS & EQUALIZATION
                     Biquad, State Variable, Parametric EQ
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## BIQUAD FILTERS

```rust
pub struct BiquadFilter {
    pub b0: f32, pub b1: f32, pub b2: f32,
    pub a1: f32, pub a2: f32,
    pub x1: f32, pub x2: f32,
    pub y1: f32, pub y2: f32,
}

impl BiquadFilter {
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
                   - self.a1 * self.y1 - self.a2 * self.y2;
        
        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;
        
        output
    }
}

pub fn make_lowpass(sample_rate: f32, cutoff: f32, q: f32) -> BiquadFilter {
    let w0 = 2.0 * std::f32::consts::PI * cutoff / sample_rate;
    let alpha = w0.sin() / (2.0 * q);
    let cos = w0.cos();
    
    let b0 = (1.0 - cos) / 2.0;
    let b1 = 1.0 - cos;
    let b2 = (1.0 - cos) / 2.0;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * cos;
    let a2 = 1.0 - alpha;
    
    BiquadFilter { b0, b1, b2, a1, a2, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 }
}
```

---

## PARAMETRIC EQ

```rust
pub struct ParametricEQ {
    pub bands: Vec<EqBand>,
}

pub struct EqBand {
    pub enable: bool,
    pub frequency: f32,
    pub gain_db: f32,
    pub q: f32,
    pub filter: BiquadFilter,
}

impl ParametricEQ {
    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = input;
        for band in &mut self.bands {
            if band.enable {
                output = band.filter.process(output);
            }
        }
        output
    }
}
```

---

*Skill DSP-003 | Category: Filters | Complexity: Expert*