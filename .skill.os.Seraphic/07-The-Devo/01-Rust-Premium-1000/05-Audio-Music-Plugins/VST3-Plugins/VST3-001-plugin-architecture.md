# SKILL VST-001: VST3 PLUGIN ARCHITECTURE

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        VST3 PLUGIN ARCHITECTURE
                     Professional VST3 Plugin Development
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive VST3 plugin development including audio processing, parameter automation,
MIDI handling, and UI integration.

---

## VST3 BASICS

### Audio Plugin Structure

```rust
use vst3::prelude::*;
use vst3::plugin::{Plugin, AudioBlock, ProcessData};
use vst3::parameter::{Parameter, ParameterInfo, ParameterType};

#[derive(Default)]
pub struct MySynth {
    sample_rate: f64,
    params: Parameters,
}

impl Plugin for MySynth {
    fn init(&self) -> Result<(), vst3::panic::Panic> {
        Ok(())
    }
    
    fn get_plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "MySynth".into(),
            vendor: "Your Company".into(),
            version: "1.0.0".into(),
            category: PluginCategory::Synthesizer,
            ..Default::default()
        }
    }
    
    fn get_parameter_count(&self) -> usize {
        3 // gain, attack, release
    }
    
    fn get_parameter_info(&self, index: usize) -> Option<ParameterInfo> {
        match index {
            0 => Some(ParameterInfo {
                identifier: "gain".into(),
                name: "Gain".into(),
                unit: "%".into(),
                min: 0.0,
                max: 100.0,
                default: 80.0,
                parameter_type: ParameterType::Linear,
                ..Default::default()
            }),
            1 => Some(ParameterInfo {
                identifier: "attack".into(),
                name: "Attack".into(),
                unit: "ms".into(),
                min: 1.0,
                max: 2000.0,
                default: 10.0,
                parameter_type: ParameterType::Exponential,
                ..Default::default()
            }),
            2 => Some(ParameterInfo {
                identifier: "release".into(),
                name: "Release".into(),
                unit: "ms".into(),
                min: 10.0,
                max: 5000.0,
                default: 500.0,
                parameter_type: ParameterType::Exponential,
                ..Default::default()
            }),
            _ => None,
        }
    }
    
    fn get_factory(&self) -> vst3::prelude::Factory {
        // Add factory classes
    }
    
    fn set_sample_rate(&mut self, rate: f64) {
        self.sample_rate = rate;
    }
    
    fn process(&mut self, data: &mut ProcessData) {
        let gain = self.params.gain / 100.0;
        
        for channel in data.audio_output {
            for sample in channel.iter_mut() {
                *sample *= gain;
            }
        }
    }
}
```

---

## PARAMETER AUTOMATION

### Reading Automation

```rust
impl MySynth {
    fn update_parameters(&mut self, data: &ProcessData) {
        for (index, param) in self.params.iter_mut().enumerate() {
            if let Some(value) = data.get_parameter(index) {
                *param = value;
            }
        }
    }
}
```

---

## MIDI PROCESSING

### MIDI Events

```rust
use vst3::event::{Event, EventList, MidiEvent, NoteOnEvent, NoteOffEvent};

impl MySynth {
    fn process_midi(&mut self, events: &EventList) {
        for event in events {
            match event {
                Event::Midi(midi) => {
                    let status = midi.data[0];
                    let note = midi.data[1];
                    let velocity = midi.data[2];
                    
                    // Note on
                    if status & 0x90 == 0x90 {
                        self.note_on(note, velocity);
                    }
                    // Note off
                    else if status & 0x80 == 0x80 {
                        self.note_off(note);
                    }
                }
                Event::NoteOn(on) => {
                    self.note_on(on.pitch, on.velocity);
                }
                Event::NoteOff(off) => {
                    self.note_off(off.pitch);
                }
                _ => {}
            }
        }
    }
}
```

---

## PATCH CHUNKS

```rust
impl vst3::preset::Patcher for MySynth {
    fn get_state(&self) -> Vec<u8> {
        // Serialize state
        let mut state = Vec::new();
        // Write params
        state.write(&self.params.gain.to_le_bytes());
        state.write(&self.params.attack.to_le_bytes());
        state
    }
    
    fn set_state(&mut self, data: &[u8]) -> Result<(), vst3::error::Error> {
        // Read params
        self.params.gain = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        self.params.attack = f32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        Ok(())
    }
}
```

---

## UI INTEGRATION

### Custom View

```rust
use vst3::view::{View, ViewEditor, Editor, Control};

impl View for MySynth {
    fn init(&self, editor: &ViewEditor) -> Result<(), vst3::error::Error> {
        editor.set_size(800, 400);
        Ok(())
    }
    
    fn draw(&self, context: &DrawContext) {
        // Draw custom UI
    }
}
```

---

## RECAP

1. **VST3 SDK** - Use vst3sdk crate
2. **Parameters** - Use ParameterInfo for each param
3. **Audio block** - Process AudioBlock for audio
4. **Events** - Handle MIDI in ProcessData

---

*Skill VST-001 | Category: VST3 | Complexity: Expert*