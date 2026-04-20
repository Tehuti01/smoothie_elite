# SKILL DSP-004: DYNAMICS PROCESSING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        DYNAMICS PROCESSING
                     Compressor, Limiter, Gate, Expander
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## COMPRESSOR

```rust
pub struct Compressor {
    pub threshold: f32,
    pub ratio: f32,
    pub attack: f32,
    pub release: f32,
    pub knee: f32,
    pub makeup_gain: f32,
    pub envelope: f32,
}

impl Compressor {
    pub fn process(&mut self, input: f32) -> f32 {
        let input_db = 20.0 * input.abs().max(1e-10).log10();
        
        let over = (self.threshold - input_db).max(0.0);
        let gr = if over < self.knee {
            over * 0.5 / self.knee
        } else {
            over
        };
        
        let gr_db = gr * (1.0 - 1.0 / self.ratio);
        
        // Envelope
        let attack = (self.attack * 1e-3).min(0.99);
        let release = (self.release * 1e-3).min(0.99);
        
        if gr_db > self.envelope {
            self.envelope += attack * (gr_db - self.envelope);
        } else {
            self.envelope += release * (gr_db - self.envelope);
        }
        
        let gain = 10.0_f32.powf((self.envelope + self.makeup_gain) / 20.0);
        input * gain
    }
}
```

---

## LOOKAHEAD LIMITER

```rust
pub struct LookaheadLimiter {
    pub threshold: f32,
    pub lookahead_ms: f32,
    pub buffer: Vec<f32>,
    pub position: usize,
}

impl LookaheadLimiter {
    pub fn new(lookahead_ms: f32, sample_rate: u32) -> Self {
        let max_samples = (lookahead_ms / 1000.0 * sample_rate as f32) as usize;
        LookaheadLimiter {
            threshold: 0.0,
            lookahead_ms,
            buffer: vec![0.0; max_samples.max(1)],
            position: 0,
        }
    }
    
    pub fn process(&mut self, input: f32) -> f32 {
        self.buffer[self.position] = input;
        self.position = (self.position + 1) % self.buffer.len();
        
        let gain = self.threshold / self.buffer.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        
        if gain < 1.0 {
            for sample in &mut self.buffer {
                *sample *= gain;
            }
        }
        
        self.buffer[self.position]
    }
}
```

---

*Skill DSP-004 | Category: Dynamics | Complexity: Expert*