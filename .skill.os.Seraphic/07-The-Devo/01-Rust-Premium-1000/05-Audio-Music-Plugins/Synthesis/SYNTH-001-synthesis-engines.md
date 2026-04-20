# SKILL DSP-001: SYNTHESIS ENGINES

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        SYNTHESIS ENGINES
                     Subtractive, Additive, FM, Wavetable
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## SUBTRACTIVE SYNTHESIS

```rust
use std::f32::consts::PI;

pub struct SubtractiveSynth {
    pub oscillators: Vec<Oscillator>,
    pub filter: Filter,
    pub amp_envelope: Envelope,
}

impl SubtractiveSynth {
    pub fn new() -> Self {
        SubtractiveSynth {
            oscillators: vec![Oscillator::new()],
            filter: Filter::new(),
            amp_envelope: Envelope::new(),
        }
    }
    
    pub fn process_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        
        for osc in &mut self.oscillators {
            sample += osc.process();
        }
        
        sample = self.filter.process(sample);
        sample *= self.amp_envelope.process();
        
        sample
    }
}

pub struct Oscillator {
    pub waveform: Waveform,
    pub frequency: f32,
    pub phase: f32,
    pub detune: f32,
}

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
    Noise,
}

impl Oscillator {
    pub fn process(&mut self) -> f32 {
        self.phase += self.frequency / 44100.0;
        if self.phase > 1.0 { self.phase -= 1.0; }
        
        match self.waveform {
            Waveform::Sine => (self.phase * 2.0 * PI).sin(),
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Triangle => 4.0 * (self.phase - 0.5).abs() - 1.0,
            Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
        }
    }
}

pub fn set_frequency(&mut self, freq: f32) {
    self.frequency = freq;
}
```

---

## FM SYNTHESIS

pub struct FMSynth {
    pub operators: Vec<FMOperator>,
    pub algorithm: Vec<usize>,
}

pub struct FMOperator {
    pub carrier_freq: f32,
    pub modulator: Option<Box<FMOperator>>,
    pub modulation_index: f32,
    pub output: f32,
}

impl FMSynth {
    pub fn process(&mut self) -> f32 {
        // Get modulator output
        let mod_output = if let Some(ref mut mod_op) = self.operators[0].modulator {
            mod_op.process() * self.operators[0].modulation_index
        } else { 0.0 };
        
        // Apply to carriers
        let mut output = 0.0;
        for (i, op) in self.operators.iter_mut().enumerate() {
            if i > 0 {
                op.carrier_freq = self.algorithm[i] as f32;
                output += op.process_with_modulation(mod_output);
            }
        }
        
        output
    }
}
```

---

## WAVETABLE SYNTHESIS

pub struct Wavetable {
    pub tables: Vec<Vec<f32>>,
    pub current_table: usize,
    pub position: f32,
    pub morph: f32,
}

impl Wavetable {
    pub fn new() -> Self {
        Wavetable {
            tables: Vec::new(),
            current_table: 0,
            position: 0.0,
            morph: 0.0,
        }
    }
    
    pub fn process(&mut self) -> f32 {
        let table = &self.tables[self.current_table];
        
        let index = self.position * table.len() as f32;
        let index0 = index as usize;
        let index1 = (index0 + 1) % table.len();
        let frac = index.fract();
        
        let sample0 = table[index0];
        let sample1 = table[index1];
        
        // Linear interpolation between wavetables
        sample0 * (1.0 - self.morph) + sample1 * self.morph
    }
    
    pub fn add_wavetable(&mut self, table: Vec<f32>) {
        self.tables.push(table);
    }
}
```

---

*Skill DSP-001 | Category: Synthesis | Complexity: Expert*